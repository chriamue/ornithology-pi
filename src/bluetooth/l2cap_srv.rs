use crate::Sighting;
use bluer::{
    adv::Advertisement,
    l2cap::{SocketAddr, Stream, StreamListener, PSM_LE_DYN_START},
};
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    time::sleep,
};

use super::Message;
use super::MANUFACTURER_ID;

pub const SERVICE_UUID: uuid::Uuid = uuid::Uuid::from_u128(0xF00DC0DE00001);
pub const CHARACTERISTIC_UUID: uuid::Uuid = uuid::Uuid::from_u128(0xF00DC0DE00002);
pub const PSM: u16 = PSM_LE_DYN_START + 5;

async fn handle_connection(
    sightings: Arc<Mutex<Vec<Sighting>>>,
    stream: &mut Stream,
    addr: SocketAddr,
) {
    let recv_mtu = stream.as_ref().recv_mtu().unwrap();

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

    println!("{} disconnected", &addr.addr);
}

pub async fn run(sightings: Arc<Mutex<Vec<Sighting>>>) -> bluer::Result<()> {
    let session = bluer::Session::new().await?;
    let adapter_names = session.adapter_names().await?;
    let adapter_name = adapter_names.first().expect("No Bluetooth adapter present");
    let adapter = session.adapter(adapter_name)?;
    let adapter_addr = adapter.address().await?;
    let _channel = 42;

    let adapter_addr_type = adapter.address_type().await?;
    adapter.set_powered(true).await?;

    println!(
        "Advertising on Bluetooth adapter {} with address {}",
        &adapter_name,
        adapter.address().await?
    );
    let mut manufacturer_data = BTreeMap::new();
    manufacturer_data.insert(MANUFACTURER_ID, vec![0x21, 0x22, 0x23, 0x24]);
    let le_advertisement = Advertisement {
        service_uuids: vec![SERVICE_UUID].into_iter().collect(),
        manufacturer_data,
        discoverable: Some(true),
        local_name: Some("ornithology-pi".to_string()),
        ..Default::default()
    };
    let adv_handle = adapter.advertise(le_advertisement).await?;

    let local_sa = SocketAddr::new(adapter_addr, adapter_addr_type, PSM);
    let listener = StreamListener::bind(local_sa).await?;

    println!(
        "Listening on PSM {}. Press enter to quit.",
        listener.as_ref().local_addr()?.psm
    );
    let stdin = BufReader::new(tokio::io::stdin());
    let mut lines = stdin.lines();

    loop {
        println!("\nWaiting for connection...");

        let (mut stream, sa) = tokio::select! {
            l = listener.accept() => {
                match l {
                    Ok(v) => v,
                    Err(err) => {
                        println!("Accepting connection failed: {}", &err);
                        continue;
                    }}
            },
            _ = lines.next_line() => break,
        };
        let recv_mtu = stream.as_ref().recv_mtu()?;

        println!(
            "Accepted connection from {:?} with receive MTU {} bytes",
            &sa, &recv_mtu
        );
        handle_connection(sightings.clone(), &mut stream, sa).await
    }

    println!("Removing advertisement");
    drop(adv_handle);
    sleep(Duration::from_secs(1)).await;
    Ok(())
}
