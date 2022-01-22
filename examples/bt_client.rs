use bluer::{gatt::remote::Characteristic, AdapterEvent, Device, Result};
use futures::{pin_mut, StreamExt};
use ornithology_pi::{
    bluetooth::{CHARACTERISTIC_UUID, SERVICE_UUID},
    Sighting,
};
use std::str;
use std::time::Duration;
use tokio::time::sleep;

async fn find_our_characteristic(device: &Device) -> Result<Option<Characteristic>> {
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
                for char in service.characteristics().await? {
                    let uuid = char.uuid().await?;
                    println!("    Characteristic UUID: {}", &uuid);
                    if uuid == CHARACTERISTIC_UUID {
                        println!("    Found our characteristic!");
                        return Ok(Some(char));
                    }
                }
            }
        }

        println!("    Not found!");
    }

    Ok(None)
}

async fn exercise_characteristic(char: &Characteristic) -> Result<()> {
    println!("    Characteristic flags: {:?}", char.flags().await?);
    sleep(Duration::from_secs(1)).await;

    if char.flags().await?.read {
        println!("    Reading characteristic value");
        let value = char.read().await?;
        println!("    Read value: {:x?}", &value);

        let value = str::from_utf8(&value).unwrap();
        let sightings: Vec<Sighting> = serde_json::from_str(value).unwrap();
        println!("    Read value: {:?}", &sightings);
        sleep(Duration::from_secs(1)).await;
    }

    Ok(())
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
        let mut done = false;
        while let Some(evt) = discover.next().await {
            match evt {
                AdapterEvent::DeviceAdded(addr) => {
                    let device = adapter.device(addr)?;
                    match find_our_characteristic(&device).await {
                        Ok(Some(char)) => match exercise_characteristic(&char).await {
                            Ok(()) => {
                                println!("    Characteristic exercise completed");
                                done = true;
                            }
                            Err(err) => {
                                println!("    Characteristic exercise failed: {}", &err);
                            }
                        },
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
            if done {
                break;
            }
        }
        println!("Stopping discovery");
    }

    sleep(Duration::from_secs(1)).await;
    Ok(())
}
