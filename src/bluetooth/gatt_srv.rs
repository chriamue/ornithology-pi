use crate::Sighting;
use bluer::{
    adv::Advertisement,
    gatt::local::{Application, Characteristic, CharacteristicRead, Service},
};
use futures::FutureExt;
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::{
    io::{AsyncBufReadExt, BufReader},
    time::sleep,
};

use super::MANUFACTURER_ID;

pub const SERVICE_UUID: uuid::Uuid = uuid::Uuid::from_u128(0xF00DC0DE00001);
pub const LAST_SIGHTING_CHARACTERISTIC: uuid::Uuid = uuid::Uuid::from_u128(0xF00DC0DE00003);
pub const SIGHTING_COUNT_CHARACTERISTIC: uuid::Uuid = uuid::Uuid::from_u128(0xF00DC0DE00004);
pub const LAST_SPECIES_CHARACTERISTIC: uuid::Uuid = uuid::Uuid::from_u128(0xF00DC0DE00005);

pub fn last_sighting_characteristic(sightings: Arc<Mutex<Vec<Sighting>>>) -> Characteristic {
    Characteristic {
        uuid: LAST_SIGHTING_CHARACTERISTIC,
        read: Some(CharacteristicRead {
            read: true,
            fun: Box::new(move |req| {
                let sightings = sightings.clone();
                async move {
                    let value = {
                        let value = sightings.lock().unwrap();
                        let sighting = value.last().unwrap();
                        serde_json::json!(sighting)
                    };
                    let value = value.to_string().as_bytes().to_vec();
                    println!("Read request {:?} with value {:x?}", &req, &value);
                    Ok(value)
                }
                .boxed()
            }),
            ..Default::default()
        }),
        ..Default::default()
    }
}

pub fn sighting_count_characteristic(sightings: Arc<Mutex<Vec<Sighting>>>) -> Characteristic {
    Characteristic {
        uuid: SIGHTING_COUNT_CHARACTERISTIC,
        read: Some(CharacteristicRead {
            read: true,
            fun: Box::new(move |req| {
                let sightings = sightings.clone();
                async move {
                    let value = {
                        let value = sightings.lock().unwrap();
                        serde_json::json!(value.len())
                    };
                    let value = value.to_string().as_bytes().to_vec();
                    println!("Read request {:?} with value {:x?}", &req, &value);
                    Ok(value)
                }
                .boxed()
            }),
            ..Default::default()
        }),
        ..Default::default()
    }
}

pub fn last_species_characteristic(sightings: Arc<Mutex<Vec<Sighting>>>) -> Characteristic {
    Characteristic {
        uuid: LAST_SPECIES_CHARACTERISTIC,
        read: Some(CharacteristicRead {
            read: true,
            fun: Box::new(move |req| {
                let sightings = sightings.clone();
                async move {
                    let value = {
                        let value = sightings.lock().unwrap();
                        let sighting = value.last().unwrap();
                        serde_json::json!(sighting.species)
                    };
                    let value = value.to_string().as_bytes().to_vec();
                    println!("Read request {:?} with value {:x?}", &req, &value);
                    Ok(value)
                }
                .boxed()
            }),
            ..Default::default()
        }),
        ..Default::default()
    }
}

pub async fn run_advertise(
    adapter: &bluer::Adapter,
    sightings: Arc<Mutex<Vec<Sighting>>>,
) -> bluer::Result<bluer::gatt::local::ApplicationHandle> {
    let mut manufacturer_data = BTreeMap::new();
    manufacturer_data.insert(MANUFACTURER_ID, vec![0x21, 0x22, 0x23, 0x24]);
    let le_advertisement = Advertisement {
        service_uuids: vec![SERVICE_UUID].into_iter().collect(),
        manufacturer_data,
        discoverable: Some(true),
        local_name: Some("ornithology-pi".to_string()),
        ..Default::default()
    };
    let _adv_handle = adapter.advertise(le_advertisement).await?;

    let app = Application {
        services: vec![Service {
            uuid: SERVICE_UUID,
            primary: true,
            characteristics: vec![
                last_sighting_characteristic(sightings.clone()),
                sighting_count_characteristic(sightings.clone()),
                last_species_characteristic(sightings.clone()),
            ],
            ..Default::default()
        }],
        ..Default::default()
    };
    let app_handle = adapter.serve_gatt_application(app).await?;

    Ok(app_handle)
}

pub async fn run(sightings: Arc<Mutex<Vec<Sighting>>>) -> bluer::Result<()> {
    let session = bluer::Session::new().await?;
    let adapter_names = session.adapter_names().await?;
    let adapter_name = adapter_names.first().expect("No Bluetooth adapter present");
    let adapter = session.adapter(adapter_name)?;
    adapter.set_powered(true).await?;

    println!(
        "Advertising on Bluetooth adapter {} with address {}",
        &adapter_name,
        adapter.address().await?
    );

    let app_handle = run_advertise(&adapter, sightings).await.unwrap();
    println!("Service ready. Press enter to quit.");
    let stdin = BufReader::new(tokio::io::stdin());
    let mut lines = stdin.lines();
    let _ = lines.next_line().await;

    println!("Removing service and advertisement");
    drop(app_handle);
    sleep(Duration::from_secs(1)).await;
    Ok(())
}
