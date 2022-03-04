use crate::Sighting;
use crate::sighting::save_to_file;
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
use std::{
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::{
    io::{AsyncReadExt, AsyncWriteExt},
    time::sleep,
};

use super::Message;

use super::MANUFACTURER_ID;
pub const SERVICE_UUID: uuid::Uuid = uuid::Uuid::from_u128(0xF00DC0DE00001);
pub const CHARACTERISTIC_UUID: uuid::Uuid = uuid::Uuid::from_u128(0xF00DC0DE00002);
pub const CHANNEL: u8 = 7;
pub const MTU: u16 = 8192;

async fn handle_connection(
    sightings: Arc<Mutex<Vec<Sighting>>>,
    stream: &mut Stream,
    addr: Address,
) -> Result<(), Box<dyn Error>> {
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
                break;
            }
        };
        let buf = &buf[..n];

        let message_stream = serde_json::Deserializer::from_slice(buf).into_iter::<Message>();

        for message in message_stream {
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
                    println!("{:?}", sighting);
                    let response = serde_json::to_vec(&Message::LastResponse {
                        last: sighting.clone(),
                    })
                    .unwrap();

                    if let Err(err) = stream.write_all(&response).await {
                        println!("Write failed: {}", &err);
                        continue;
                    }
                }
                Ok(Message::SightingIdsRequest) => {
                    let sightings = {
                        let mutex: Vec<Sighting> = sightings.lock().unwrap().to_vec();
                        let sightings: Vec<String> = mutex.into_iter().map(|i| i.uuid).collect();
                        sightings
                    };
                    let response = serde_json::to_vec(&Message::SightingIdsResponse {
                        ids: sightings.clone(),
                    })
                    .unwrap();

                    if let Err(err) = stream.write_all(&response).await {
                        println!("Write failed: {}", &err);
                        continue;
                    }
                }
                Ok(Message::SightingRequest { uuid }) => {
                    println!("sighting {}", uuid);
                    let sighting = {
                        let sightings = sightings.lock().unwrap();
                        let sighting = sightings
                            .iter()
                            .filter(|sighting| sighting.uuid == uuid)
                            .last()
                            .cloned();
                        sighting.unwrap_or_default()
                    };
                    let response =
                        serde_json::to_vec(&Message::SightingResponse { sighting }).unwrap();

                    if let Err(err) = stream.write_all(&response).await {
                        println!("Write failed: {}", &err);
                        continue;
                    }
                }
                Ok(Message::RemoveSightingRequest { uuid }) => {
                    println!("remove sighting {}", uuid);
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
                    })
                    .unwrap();

                    if let Err(err) = stream.write_all(&response).await {
                        println!("Write failed: {}", &err);
                        continue;
                    }
                }
                Ok(Message::ImageRequest { uuid }) => {
                    println!("{}", uuid);
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
                            let mut buf = vec![];
                            base_img
                                .write_to(&mut buf, image::ImageOutputFormat::Jpeg(60))
                                .unwrap();
                            buf
                        }
                        Err(err) => {
                            println!("{:?}", err);
                            vec![]
                        }
                    };
                    let base64_img = format!("data:image/jpeg;{}", base64::encode(&buf));
                    let response = serde_json::to_vec(&Message::ImageResponse {
                        uuid,
                        base64: base64_img.clone(),
                    })
                    .unwrap();

                    if let Err(err) = stream.write_all(&response).await {
                        println!("Write failed: {}", &err);
                        continue;
                    }
                }
                _ => {
                    let text = std::str::from_utf8(buf).unwrap();
                    println!("Echoing {} bytes: {}", buf.len(), text);
                    if let Err(err) = stream.write_all(buf).await {
                        println!("Write failed: {}", &err);
                        continue;
                    }
                }
            }
        }
    }

    println!("{} disconnected", &addr);
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

    println!("Registered profile {}", profile.uuid);

    let mut hndl = session.register_profile(profile).await?;

    println!("Listening on channel {}", CHANNEL);

    loop {
        println!("\nWaiting for connection...");
        let req = hndl.next().await.expect("received no connect request");
        let sa = req.device();
        let mut stream = match req.accept() {
            Ok(v) => v,
            Err(err) => {
                println!("Accepting connection failed: {}", &err);
                continue;
            }
        };
        let recv_mtu = MTU;

        println!(
            "Accepted connection from {:?} with receive MTU {} bytes",
            &sa, &recv_mtu
        );
        match handle_connection(sightings.clone(), &mut stream, sa).await {
            Err(err) => println!("{:?}", err),
            _ => (),
        }
    }

    println!("Removing advertisement");
    drop(hndl);
    drop(_agent_hndl);
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

    println!(
        "Advertising on Bluetooth adapter {} with address {}",
        &adapter_name,
        adapter.address().await?
    );

    run_session(&session, sightings).await.unwrap();
    sleep(Duration::from_secs(1)).await;
    Ok(())
}
