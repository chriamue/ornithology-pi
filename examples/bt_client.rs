use bluer::{
    l2cap::{SocketAddr, Stream},
    Adapter, Address, AddressType,
};
use bluer::{AdapterEvent, Device, Result};
use futures::{pin_mut, StreamExt};
use ornithology_pi::bluetooth::{Message, PSM, SERVICE_UUID};

use std::time::Duration;
use tokio::io::AsyncReadExt;
use tokio::io::AsyncWriteExt;
use tokio::time::sleep;

async fn find_our_service(device: &Device) -> Result<Option<()>> {
    let addr = device.address();
    let uuids = device.uuids().await?.unwrap_or_default();
    println!("Discovered device {} with service UUIDs {:?}", addr, &uuids);
    let md = device.manufacturer_data().await?;
    println!("    Manufacturer data: {:x?}", &md);

    match uuids.contains(&SERVICE_UUID) {
        true => {
            println!("    Device provides our service!");
            Ok(Some(()))
        }
        false => Ok(None),
    }
}

async fn find_device(adapter: Adapter) -> Option<Device> {
    let discover = adapter.discover_devices().await.ok()?;
    pin_mut!(discover);
    while let Some(evt) = discover.next().await {
        match evt {
            AdapterEvent::DeviceAdded(addr) => {
                let device = adapter.device(addr).ok()?;
                match find_our_service(&device).await {
                    Ok(Some(())) => {
                        return Some(device);
                    }
                    Ok(None) => (),
                    Err(err) => {
                        println!("    Device failed: {}", &err);
                        let _ = adapter.remove_device(device.address()).await;
                    }
                }
                match device.disconnect().await {
                    Ok(()) => println!("    Device disconnected"),
                    Err(err) => println!("    Device disconnection failed: {}", &err),
                }
                println!();
            }
            AdapterEvent::DeviceRemoved(addr) => {
                println!("Device removed {}", addr);
            }
            _ => (),
        }
    }
    println!("Stopping discovery");
    None
}

async fn discover_address() -> Option<Address> {
    let session = bluer::Session::new().await.unwrap();
    let adapter_names = session.adapter_names().await.unwrap();
    let adapter_name = adapter_names.first().expect("No Bluetooth adapter present");
    let adapter = session.adapter(adapter_name).unwrap();
    adapter.set_powered(true).await.unwrap();

    println!(
        "Discovering on Bluetooth adapter {} with address {} - {}\n",
        &adapter_name,
        adapter.address().await.unwrap(),
        &SERVICE_UUID
    );

    let device = find_device(adapter).await.unwrap();
    Some(device.address())
}

async fn handle_connection(stream: &mut Stream, addr: Address) {
    let recv_mtu = stream.as_ref().recv_mtu().unwrap();

    println!(
        "Connected to {:?} with receive MTU {} bytes",
        &addr, &recv_mtu
    );

    let request = serde_json::to_vec(&Message::CountRequest).unwrap();

    if let Err(err) = stream.write_all(&request).await {
        println!("Write failed: {}", &err);
    }

    let mut n = 0;
    loop {
        n += 1;

        // Vary buffer size between MTU and smaller value to test
        // partial reads.
        let buf_size = if n % 5 == 0 { recv_mtu - 70 } else { recv_mtu };
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
            }
            Ok(Message::Pong) => {
                println!("{:?}", Message::Pong);

                sleep(Duration::from_secs(1)).await;
                let request = serde_json::to_vec(&Message::Ping).unwrap();

                if let Err(err) = stream.write_all(&request).await {
                    println!("Write failed: {}", &err);
                    continue;
                }
            }
            Ok(Message::CountResponse { count }) => {
                println!("Counted {}", count);
                let request = serde_json::to_vec(&Message::LastRequest).unwrap();

                if let Err(err) = stream.write_all(&request).await {
                    println!("Write failed: {}", &err);
                }
            }
            Ok(Message::LastResponse { last }) => {
                println!("Sighting {:?}", last);
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

#[tokio::main]
async fn main() -> bluer::Result<()> {
    //let target_address = discover_address().await.unwrap();

    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    adapter.set_powered(true).await?;

    let target_address: Address = "00:24:D6:A1:9A:BD".parse().expect("invalid address");

    let target_sa = SocketAddr::new(target_address.clone(), AddressType::LePublic, PSM);
    println!("Connecting to {:?}", &target_sa);
    let mut stream = Stream::connect(target_sa).await.expect("connection failed");
    println!("Local address: {:?}", stream.as_ref().local_addr()?);
    println!("Remote address: {:?}", stream.peer_addr()?);
    println!("Send MTU: {:?}", stream.as_ref().send_mtu());
    println!("Recv MTU: {}", stream.as_ref().recv_mtu()?);
    println!("Security: {:?}", stream.as_ref().security()?);

    sleep(Duration::from_secs(1)).await;

    println!("Flow control: {:?}", stream.as_ref().flow_control());

    handle_connection(&mut stream, target_address).await;

    sleep(Duration::from_secs(1)).await;
    Ok(())
}
