use crate::Sighting;
use bluer::{
    agent::Agent,
    rfcomm::{Profile, Role, Stream},
    Address,
};
use futures::StreamExt;
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    time::sleep,
};

use super::Message;

pub const SERVICE_UUID: uuid::Uuid = uuid::Uuid::from_u128(0xF00DC0DE00001);
pub const CHARACTERISTIC_UUID: uuid::Uuid = uuid::Uuid::from_u128(0xF00DC0DE00002);
pub const CHANNEL: u8 = 7;
pub const MTU: u16 = 8192;

async fn handle_connection(
    sightings: Arc<Mutex<Vec<Sighting>>>,
    stream: &mut Stream,
    addr: Address,
) {
    let recv_mtu = MTU;

    println!(
        "Accepted connection from {:?} with receive MTU {} bytes",
        &addr, &recv_mtu
    );

    if let Err(err) = stream
        .write_all(format!("{:?}", sightings.lock().unwrap().len()).as_bytes())
        .await
    {
        println!("Write failed: {}", &err);
    }

    loop {
        let buf_size = recv_mtu;
        let mut buf = vec![0; buf_size as _];

        let n = match stream.read(&mut buf).await {
            Ok(0) => {
                println!("Stream ended");
                break;
            }
            Ok(n) => n,
            Err(err) => {
                println!("Read failed: {}", &err);
                continue;
            }
        };
        let buf = &buf[..n];

        let message = serde_json::from_slice::<Message>(buf);
        match message {
            Ok(Message::Ping) => {
                println!("{:?}", Message::Ping);
                let response = serde_json::to_vec(&Message::Pong).unwrap();

                if let Err(err) = stream.write_all(&response).await {
                    println!("Write failed: {}", &err);
                    continue;
                }
            }
            Ok(Message::Pong) => {
                println!("{:?}", Message::Pong);
            }
            Ok(Message::CountRequest) => {
                let count = {
                    let len = sightings.lock().unwrap().len();
                    len as u64
                };
                let response = serde_json::to_vec(&Message::CountResponse { count }).unwrap();
                if let Err(err) = stream.write_all(&response).await {
                    println!("Write failed: {}", &err);
                    continue;
                }
            }
            Ok(Message::LastRequest) => {
                let sighting = {
                    let mutex = sightings.lock().unwrap();
                    let last = mutex.last();

                    last.unwrap().clone()
                };
                let response = serde_json::to_vec(&Message::LastResponse {
                    last: sighting.clone(),
                })
                .unwrap();
                if let Err(err) = stream.write_all(&response).await {
                    println!("Write failed: {}", &err);
                    continue;
                }
            }
            _ => {
                println!("Echoing {} bytes", buf.len());
                if let Err(err) = stream.write_all(buf).await {
                    println!("Write failed: {}", &err);
                    continue;
                }
            }
        }
    }

    println!("{} disconnected", &addr);
}

pub async fn run(sightings: Arc<Mutex<Vec<Sighting>>>) -> bluer::Result<()> {
    let session = bluer::Session::new().await?;
    let adapter_names = session.adapter_names().await?;
    let adapter_name = adapter_names.first().expect("No Bluetooth adapter present");
    let adapter = session.adapter(adapter_name)?;
    adapter.set_powered(true).await?;
    adapter.set_discoverable(true).await?;
    adapter.set_discoverable_timeout(0).await?;
    adapter.set_pairable(false).await?;

    let agent = Agent::default();
    let _agent_hndl = session.register_agent(agent).await?;

    let profile = Profile {
        uuid: SERVICE_UUID,
        name: Some("ornithology-pi".to_string()),
        channel: Some(CHANNEL.into()),
        role: Some(Role::Server),
        require_authentication: Some(false),
        require_authorization: Some(false),
        ..Default::default()
    };
    let mut hndl = session.register_profile(profile).await?;

    eprintln!("Registered profile");

    println!(
        "Advertising on Bluetooth adapter {} with address {}",
        &adapter_name,
        adapter.address().await?
    );

    println!("Listening on channel {}", CHANNEL);

    let stdin = BufReader::new(tokio::io::stdin());
    let mut lines = stdin.lines();

    loop {
        println!("\nWaiting for connection...");
        //let req = hndl.next().await.expect("received no connect request");
        //eprintln!("Connect from {}", req.device());

        let (mut stream, sa) = tokio::select! {
            req = hndl.next() => {
                let req = req.expect("received no connect request");
                let sa = req.device();
                match req.accept() {
                    Ok(v) => (v, sa),
                    Err(err) => {
                        println!("Accepting connection failed: {}", &err);
                        continue;
                    }}
            },
            _ = lines.next_line() => break,
        };
        let recv_mtu = MTU; // stream.as_ref().recv_mtu()?;

        println!(
            "Accepted connection from {:?} with receive MTU {} bytes",
            &sa, &recv_mtu
        );
        handle_connection(sightings.clone(), &mut stream, sa).await
    }

    println!("Removing advertisement");
    drop(hndl);
    sleep(Duration::from_secs(1)).await;
    Ok(())
}
