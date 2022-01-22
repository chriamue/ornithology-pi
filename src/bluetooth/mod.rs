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

pub const SERVICE_UUID: uuid::Uuid = uuid::Uuid::from_u128(0xF00DC0DE00002);
pub const CHARACTERISTIC_UUID: uuid::Uuid = uuid::Uuid::from_u128(0xF00DC0DE00002);
pub const MANUFACTURER_ID: u16 = 0xf00d;

pub async fn run_bluetooth(sightings: Arc<Mutex<Vec<Sighting>>>) -> bluer::Result<()> {
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

    println!(
        "Serving GATT service on Bluetooth adapter {}",
        &adapter_name
    );
    let app = Application {
        services: vec![Service {
            uuid: SERVICE_UUID,
            primary: true,
            characteristics: vec![Characteristic {
                uuid: CHARACTERISTIC_UUID,
                read: Some(CharacteristicRead {
                    read: true,
                    fun: Box::new(move |req| {
                        let value = sightings.clone();
                        async move {
                            let value = value.lock().unwrap().clone();
                            let value = serde_json::json!(value);
                            let value = value.to_string().as_bytes().to_vec();
                            println!("Read request {:?} with value {:x?}", &req, &value);
                            Ok(value)
                        }
                        .boxed()
                    }),
                    ..Default::default()
                }),
                ..Default::default()
            }],
            ..Default::default()
        }],
        ..Default::default()
    };
    let app_handle = adapter.serve_gatt_application(app).await?;

    println!("Service ready. Press enter to quit.");
    let stdin = BufReader::new(tokio::io::stdin());
    let mut lines = stdin.lines();
    let _ = lines.next_line().await;

    println!("Removing service and advertisement");
    drop(app_handle);
    drop(adv_handle);
    sleep(Duration::from_secs(1)).await;

    Ok(())
}
