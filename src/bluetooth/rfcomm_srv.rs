use super::Message;
use crate::bluetooth::setup_session;
use crate::sighting::save_to_file;
use crate::Sighting;
use base64;
use bluer::{
    adv::Advertisement,
    agent::Agent,
    rfcomm::{Profile, Role, Stream},
    Address,
};
use futures::StreamExt;
use image::{self, imageops::FilterType};
use std::error::Error;
use std::io::Cursor;
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

async fn handle_message(
    message: Message,
    sightings: &Arc<Mutex<Vec<Sighting>>>,
    stream: &mut Stream,
) -> Result<(), Box<dyn Error>> {
    match message {
        Message::Ping => {
            log::debug!("{:?}", Message::Ping);
            let response = serde_json::to_vec(&Message::Pong).unwrap();

            stream.write_all(&response).await?;
        }
        Message::Pong => {
            log::debug!("{:?}", Message::Pong);
        }
        Message::CountRequest => {
            let count = {
                let len = sightings.lock().unwrap().len();
                len as u64
            };
            let response = serde_json::to_vec(&Message::CountResponse { count }).unwrap();
            stream.write_all(&response).await?;
        }
        Message::LastRequest => {
            let sighting = {
                let mutex = sightings.lock().unwrap();
                let last = mutex.last();

                last.unwrap().clone()
            };
            log::debug!("{:?}", sighting);
            let response = serde_json::to_vec(&Message::LastResponse {
                last: sighting.clone(),
            })?;

            stream.write_all(&response).await?;
        }
        Message::SightingIdsRequest => {
            let sightings = {
                let mutex: Vec<Sighting> = sightings.lock().unwrap().to_vec();
                let sightings: Vec<String> = mutex.into_iter().map(|i| i.uuid).collect();
                sightings
            };
            let response = serde_json::to_vec(&Message::SightingIdsResponse {
                ids: sightings.clone(),
            })
            .unwrap();

            stream.write_all(&response).await?;
        }
        Message::SightingRequest { uuid } => {
            log::debug!("sighting {}", uuid);
            let sighting = {
                let sightings = sightings.lock().unwrap();
                let sighting = sightings
                    .iter()
                    .filter(|sighting| sighting.uuid == uuid)
                    .last()
                    .cloned();
                sighting.unwrap_or_default()
            };
            let response = serde_json::to_vec(&Message::SightingResponse { sighting }).unwrap();

            stream.write_all(&response).await?;
        }
        Message::RemoveSightingRequest { uuid } => {
            log::debug!("remove sighting {}", uuid);
            let sightings = {
                let mut sightings = sightings.lock().unwrap();
                let index = sightings.iter().position(|x| x.uuid == uuid).unwrap();
                sightings.remove(index);
                sightings.to_vec()
            };
            save_to_file(sightings.clone(), "sightings/sightings.db").unwrap();
            let sightings = {
                let sightings: Vec<String> = sightings.into_iter().map(|i| i.uuid).collect();
                sightings
            };
            let response = serde_json::to_vec(&Message::SightingIdsResponse {
                ids: sightings.clone(),
            })?;

            stream.write_all(&response).await?;
        }
        Message::ImageRequest { uuid } => {
            log::debug!("{}", uuid);
            let filename = {
                let sightings = sightings.lock().unwrap();
                let sighting = sightings
                    .iter()
                    .filter(|sighting| sighting.uuid == uuid)
                    .last()
                    .cloned();
                let sighting = sighting.unwrap_or_default();
                format!("{}_{}.jpg", sighting.species, sighting.uuid)
            };
            let buf = match image::open(format!("sightings/{}", filename)) {
                Ok(base_img) => {
                    let base_img = base_img.resize(640, 480, FilterType::Gaussian);
                    let mut buf = Cursor::new(Vec::new());
                    base_img
                        .write_to(&mut buf, image::ImageOutputFormat::Jpeg(60))
                        .unwrap();
                    buf.into_inner()
                }
                Err(err) => {
                    log::debug!("{:?}", err);
                    vec![]
                }
            };
            let base64_img = format!("data:image/jpeg;{}", base64::encode(&buf));
            let response = serde_json::to_vec(&Message::ImageResponse {
                uuid,
                base64: base64_img.clone(),
            })?;

            stream.write_all(&response).await?
        }
        _ => {
            log::debug!("Read {:?} bytes", message);
        }
    }
    Ok(())
}

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
