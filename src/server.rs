use crate::sighting::save_to_file;
#[cfg(feature = "detect")]
use crate::BirdDetector;
use crate::{Capture, MJpeg, Sighting, WebCam};
use rocket::fs::FileServer;
use rocket::fs::NamedFile;
use rocket::http::{ContentType, Status};
use rocket::response::stream::ByteStream;
use rocket::serde::json::Json;
use rocket::State;
use rocket::{delete, get, routes};
use rocket::{Build, Rocket};
use std::path::Path;
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub struct DetectorState {
    #[cfg(feature = "detect")]
    pub mutex: Arc<Mutex<BirdDetector>>,
}

#[get("/generate_204")]
fn generate_204() -> Status {
    Status::NoContent
}

#[get("/sightings?<start>&<end>")]
fn sightings(
    sightings: &State<Arc<Mutex<Vec<Sighting>>>>,
    start: Option<usize>,
    end: Option<usize>,
) -> Json<Vec<Sighting>> {
    let sightings = match sightings.lock() {
        Ok(sightings) => sightings.to_vec(),
        Err(err) => {
            println!("{}", err);
            Vec::new()
        }
    };
    let sightings = match (start, end) {
        (Some(start), Some(end)) => {
            let end = end.min(sightings.len()).max(0);
            sightings[start.max(0).min(end)..end].to_vec()
        }
        (Some(start), None) => sightings[start..].to_vec(),
        _ => sightings,
    };
    Json(sightings)
}

#[get("/webcam")]
fn webcam(capture: &'_ State<Arc<Mutex<WebCam>>>) -> (Status, (ContentType, ByteStream<MJpeg>)) {
    let capture: Arc<Mutex<WebCam>> = { capture.inner().clone() };

    (
        Status::Ok,
        (
            ContentType::new("multipart", "x-mixed-replace").with_params([("boundary", "frame")]),
            ByteStream(MJpeg::new(capture)),
        ),
    )
}

#[get("/frame")]
fn frame(capture: &'_ State<Arc<Mutex<WebCam>>>) -> (Status, (ContentType, Vec<u8>)) {
    let frame = {
        let mut capture = capture.lock().unwrap();
        capture.bytes_jpeg().unwrap()
    };
    (Status::Ok, (ContentType::JPEG, frame))
}

#[get("/sightings/<id>")]
async fn sighting(sightings: &State<Arc<Mutex<Vec<Sighting>>>>, id: String) -> Option<NamedFile> {
    let filename = {
        let sightings = sightings.lock().unwrap();
        let sighting = sightings
            .iter()
            .filter(|sighting| sighting.uuid == id)
            .last()
            .cloned();
        let sighting = sighting.unwrap();
        format!("{}_{}.jpg", sighting.species, sighting.uuid)
    };

    NamedFile::open(Path::new("sightings/").join(filename))
        .await
        .ok()
}

#[delete("/sightings/<id>")]
async fn delete_sighting(sightings: &State<Arc<Mutex<Vec<Sighting>>>>, id: String) {
    let filename = {
        let sightings = sightings.lock().unwrap();
        let sighting = sightings
            .iter()
            .filter(|sighting| sighting.uuid == id)
            .last()
            .cloned();
        let sighting = sighting.unwrap();
        format!("{}_{}.jpg", sighting.species, sighting.uuid)
    };

    std::fs::remove_file(Path::new("sightings/").join(filename)).unwrap();

    let sightings = {
        let mut sightings = sightings.lock().unwrap();
        let index = sightings.iter().position(|x| x.uuid == id).unwrap();
        sightings.remove(index);
        sightings.to_vec()
    };
    save_to_file(sightings, "sightings/sightings.db").unwrap()
}

pub fn server(sightings: Arc<Mutex<Vec<Sighting>>>, capture: Arc<Mutex<WebCam>>) -> Rocket<Build> {
    rocket::build()
        .mount("/", FileServer::from("yew-app/dist"))
        .mount(
            "/",
            routes![
                frame,
                sightings,
                sighting,
                delete_sighting,
                webcam,
                generate_204
            ],
        )
        .manage(sightings)
        .manage(capture)
}
