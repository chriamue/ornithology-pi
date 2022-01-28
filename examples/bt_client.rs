use bluer::{AdapterEvent, Device, Result};
use bluer::{
    l2cap::{SocketAddr, Stream}, AddressType,
};
use futures::{pin_mut, StreamExt};
use ornithology_pi::{
    bluetooth::{PSM, SERVICE_UUID},
};

use std::time::Duration;
use tokio::io::AsyncWriteExt;
use tokio::time::sleep;

async fn find_our_service(device: &Device) -> Result<Option<()>> {
    let addr = device.address();
    let uuids = device.uuids().await?.unwrap_or_default();
    println!("Discovered device {} with service UUIDs {:?}", addr, &uuids);
    let md = device.manufacturer_data().await?;
    println!("    Manufacturer data: {:x?}", &md);

    if uuids.contains(&SERVICE_UUID) {
        println!("    Device provides our service!");
        if !device.is_connected().await? {
            println!("    Connecting...");
            let mut retries = 2;
            loop {
                match device.connect().await {
                    Ok(()) => break,
                    Err(err) if retries > 0 => {
                        println!("    Connect error: {}", &err);
                        retries -= 1;
                    }
                    Err(err) => return Err(err),
                }
            }
            println!("    Connected");
        } else {
            println!("    Already connected");
        }

        println!("    Enumerating services...");
        for service in device.services().await? {
            let uuid = service.uuid().await?;
            println!("    Service UUID: {}", &uuid);
            if uuid == SERVICE_UUID {
                println!("    Found our service!");
                return Ok(Some(()));
            }
        }
        println!("    Not found!");
    }

    Ok(None)
}

#[tokio::main]
async fn main() -> bluer::Result<()> {
    let session = bluer::Session::new().await?;
    let adapter_names = session.adapter_names().await?;
    let adapter_name = adapter_names.first().expect("No Bluetooth adapter present");
    let adapter = session.adapter(adapter_name)?;
    adapter.set_powered(true).await?;

    {
        println!(
            "Discovering on Bluetooth adapter {} with address {} - {}\n",
            &adapter_name,
            adapter.address().await?,
            &SERVICE_UUID
        );
        let discover = adapter.discover_devices().await?;
        pin_mut!(discover);
        while let Some(evt) = discover.next().await {
            match evt {
                AdapterEvent::DeviceAdded(addr) => {
                    let device = adapter.device(addr)?;
                    match find_our_service(&device).await {
                        Ok(Some(())) => {
                            let target_sa =
                                SocketAddr::new(device.address(), AddressType::LePublic, PSM);
                            println!("Connecting to {:?}", &target_sa);
                            let mut stream =
                                Stream::connect(target_sa).await.expect("connection failed");
                            println!("Local address: {:?}", stream.as_ref().local_addr()?);
                            println!("Remote address: {:?}", stream.peer_addr()?);
                            println!("Send MTU: {:?}", stream.as_ref().send_mtu());
                            println!("Recv MTU: {}", stream.as_ref().recv_mtu()?);
                            println!("Security: {:?}", stream.as_ref().security()?);
                            println!("Flow control: {:?}", stream.as_ref().flow_control());
                            const HELLO_MSG: &[u8] = b"Hello from l2cap_server!";
                            stream.write_all(HELLO_MSG).await.unwrap();
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
    }

    sleep(Duration::from_secs(1)).await;
    Ok(())
}
