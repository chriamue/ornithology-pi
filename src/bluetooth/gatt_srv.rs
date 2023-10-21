use crate::bluetooth::setup_session;
use crate::Sighting;
use bluer::{
    adv::Advertisement,
    gatt::{
        local::{
            characteristic_control, Application, ApplicationHandle, Characteristic,
            CharacteristicControl, CharacteristicControlEvent, CharacteristicControlHandle,
            CharacteristicNotify, CharacteristicNotifyMethod, CharacteristicRead,
            CharacteristicWrite, CharacteristicWriteMethod, Service,
        },
        CharacteristicReader, CharacteristicWriter,
    },
};
use futures::FutureExt;
use futures::{future, pin_mut, StreamExt};
use std::{
    collections::BTreeMap,
    sync::{Arc, Mutex},
    time::Duration,
};
use tokio::{
    io::{AsyncBufReadExt, AsyncReadExt, AsyncWriteExt, BufReader},
    time::sleep,
};

use super::{CHARACTERISTIC_UUID, MANUFACTURER_ID};

use super::{CHANNEL, MTU, SERVICE_UUID};

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
                    log::debug!("Read request {:?} with value {:x?}", &req, &value);
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
                    log::debug!("Read request {:?} with value {:x?}", &req, &value);
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
                    log::debug!("Read request {:?} with value {:x?}", &req, &value);
                    Ok(value)
                }
                .boxed()
            }),
            ..Default::default()
        }),
        ..Default::default()
    }
}

pub fn stream_characteristic(
    char_handle: CharacteristicControlHandle,
    sightings: Arc<Mutex<Vec<Sighting>>>,
) -> Characteristic {
    Characteristic {
        uuid: CHARACTERISTIC_UUID,
        write: Some(CharacteristicWrite {
            write_without_response: true,
            method: CharacteristicWriteMethod::Io,
            ..Default::default()
        }),
        notify: Some(CharacteristicNotify {
            notify: true,
            method: CharacteristicNotifyMethod::Io,
            ..Default::default()
        }),
        control_handle: char_handle,
        ..Default::default()
    }
}

pub async fn run_advertise(
    adapter: &bluer::Adapter,
) -> bluer::Result<bluer::adv::AdvertisementHandle> {
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
    Ok(adv_handle)
}

pub async fn run_app(
    adapter: &bluer::Adapter,
    sightings: Arc<Mutex<Vec<Sighting>>>,
) -> bluer::Result<(ApplicationHandle, CharacteristicControl)> {
    let (char_control, char_handle) = characteristic_control();
    let app = Application {
        services: vec![Service {
            uuid: SERVICE_UUID,
            primary: true,
            characteristics: vec![
                stream_characteristic(char_handle, sightings.clone()),
                // last_sighting_characteristic(sightings.clone()),
                // sighting_count_characteristic(sightings.clone()),
                // last_species_characteristic(sightings.clone()),
            ],
            ..Default::default()
        }],
        ..Default::default()
    };
    let app_handle = adapter.serve_gatt_application(app).await?;
    Ok((app_handle, char_control))
}

pub async fn listen(
    char_control: CharacteristicControl,
    sightings: Arc<Mutex<Vec<Sighting>>>,
) -> bluer::Result<()> {
    let mut read_buf = Vec::new();
    let mut reader_opt: Option<CharacteristicReader> = None;
    let mut writer_opt: Option<CharacteristicWriter> = None;
    pin_mut!(char_control);

    loop {
        tokio::select! {
            evt = char_control.next() => {
                match evt {
                    Some(CharacteristicControlEvent::Write(req)) => {
                        log::debug!("Accepting write request event with MTU {}", req.mtu());
                        read_buf = vec![0; req.mtu()];
                        reader_opt = Some(req.accept()?);
                    },
                    Some(CharacteristicControlEvent::Notify(notifier)) => {
                        log::debug!("Accepting notify request event with MTU {}", notifier.mtu());
                        writer_opt = Some(notifier);
                    },
                    None => break,
                }
            },
            read_res = async {
                match &mut reader_opt {
                    Some(reader) if writer_opt.is_some() => reader.read(&mut read_buf).await,
                    _ => future::pending().await,
                }
            } => {
                match read_res {
                    Ok(0) => {
                        log::debug!("Read stream ended");
                        reader_opt = None;
                    }
                    Ok(n) => {
                        let value = read_buf[..n].to_vec();
                        log::debug!("Echoing {} bytes: {:x?} ... {:x?}", value.len(), &value[0..4.min(value.len())], &value[value.len().saturating_sub(4) ..]);
                        if value.len() < 512 {
                            log::debug!("");
                        }
                        if let Err(err) = writer_opt.as_mut().unwrap().write_all(&value).await {
                            log::debug!("Write failed: {}", &err);
                            writer_opt = None;
                        }
                    }
                    Err(err) => {
                        log::debug!("Read stream error: {}", &err);
                        reader_opt = None;
                    }
                }
            }
        }
    }
    Ok(())
}

pub async fn run(sightings: Arc<Mutex<Vec<Sighting>>>) -> bluer::Result<()> {
    let session = bluer::Session::new().await?;
    setup_session(&session).await?;
    let adapter = session.default_adapter().await?;

    let adv_handle = run_advertise(&adapter).await.unwrap();
    let (app_handle, char_control) = run_app(&adapter, sightings.clone()).await.unwrap();

    let listen_handle = tokio::spawn(listen(char_control, sightings.clone()));

    log::info!("Service ready. Press enter to quit.");
    let stdin = BufReader::new(tokio::io::stdin());
    let mut lines = stdin.lines();
    let _ = lines.next_line().await;

    log::info!("Removing service and advertisement");
    drop(adv_handle);
    drop(app_handle);
    //drop(char_control);
    drop(listen_handle);
    sleep(Duration::from_secs(1)).await;
    Ok(())
}
