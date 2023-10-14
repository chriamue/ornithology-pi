use crate::bluetooth::Message;
use crate::sighting::save_to_file;
use crate::Sighting;
use image::imageops::FilterType;
use std::error::Error;
use std::io::Cursor;
use std::sync::{Arc, Mutex};
use tokio::io::{AsyncWrite, AsyncWriteExt};

pub async fn handle_message(
    message: Message,
    sightings: &Arc<Mutex<Vec<Sighting>>>,
    stream: &mut (dyn AsyncWrite + Unpin + Send),
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
