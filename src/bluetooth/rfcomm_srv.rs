use super::handle_message;
use super::Message;
use crate::bluetooth::setup_session;
use crate::Sighting;
use bluer::{
    agent::Agent,
    rfcomm::{Profile, Role, Stream},
    Address,
};
use futures::StreamExt;
use std::error::Error;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    time::sleep,
};

use super::CHANNEL;
use super::CHARACTERISTIC_UUID;
use super::MANUFACTURER_ID;
use super::MTU;
use super::SERVICE_UUID;

async fn handle_connection(
    sightings: Arc<Mutex<Vec<Sighting>>>,
    stream: &mut Stream,
    addr: Address,
) -> Result<(), Box<dyn Error>> {
    let recv_mtu = MTU;

    log::debug!(
        "Accepted connection from {:?} with receive MTU {} bytes",
        &addr,
        &recv_mtu
    );

    if let Err(err) = stream
        .write_all(format!("{:?}", sightings.lock().unwrap().len()).as_bytes())
        .await
    {
        log::debug!("Write failed: {}", &err);
    }

    loop {
        let buf_size = recv_mtu;
        let mut buf = vec![0; buf_size as _];

        let n = match stream.read(&mut buf).await {
            Ok(0) => {
                log::debug!("Stream ended");
                break;
            }
            Ok(n) => n,
            Err(err) => {
                log::debug!("Read failed: {}", &err);
                break;
            }
        };
        let buf = &buf[..n];

        let message_stream = serde_json::Deserializer::from_slice(buf).into_iter::<Message>();

        for message in message_stream {
            match message {
                Ok(valid_message) => {
                    match handle_message(valid_message, &sightings, stream).await {
                        Ok(_) => (),
                        Err(e) => log::debug!("Error handling message: {:?}", e),
                    }
                }
                Err(e) => log::debug!("Error parsing message: {:?}", e),
            }
        }
    }

    log::debug!("{} disconnected", &addr);
    Ok(())
}

pub async fn run_session(
    session: &bluer::Session,
    sightings: Arc<Mutex<Vec<Sighting>>>,
) -> bluer::Result<()> {
    let agent = Agent::default();
    let _agent_hndl = session.register_agent(agent).await?;

    let profile = Profile {
        uuid: SERVICE_UUID,
        name: Some("ornithology-pi".to_string()),
        channel: Some(CHANNEL.into()),
        role: Some(Role::Server),
        require_authentication: Some(false),
        require_authorization: Some(false),
        auto_connect: Some(true),
        ..Default::default()
    };

    log::debug!("Registered profile {}", profile.uuid);

    let mut hndl = session.register_profile(profile).await?;

    log::debug!("Listening on channel {}", CHANNEL);

    loop {
        log::debug!("\nWaiting for connection...");
        let req = hndl.next().await.expect("received no connect request");
        let sa = req.device();
        let mut stream = match req.accept() {
            Ok(v) => v,
            Err(err) => {
                log::debug!("Accepting connection failed: {}", &err);
                continue;
            }
        };
        let recv_mtu = MTU;

        log::debug!(
            "Accepted connection from {:?} with receive MTU {} bytes",
            &sa,
            &recv_mtu
        );
        match handle_connection(sightings.clone(), &mut stream, sa).await {
            Err(err) => log::debug!("{:?}", err),
            _ => (),
        }
    }

    log::debug!("Removing advertisement");
    drop(hndl);
    drop(_agent_hndl);
}

pub async fn run(sightings: Arc<Mutex<Vec<Sighting>>>) -> bluer::Result<()> {
    let session = bluer::Session::new().await?;

    setup_session(&session).await?;

    run_session(&session, sightings).await.unwrap();
    sleep(Duration::from_secs(1)).await;
    Ok(())
}
