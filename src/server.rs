#[cfg(feature = "detect")]
use crate::BirdDetector;
use crate::{MJpeg, Sighting, WebCam};
use rocket::fs::NamedFile;
use rocket::http::{ContentType, Status};
use rocket::response::content::Custom;
use rocket::response::stream::ByteStream;
use rocket::serde::json::Json;
use rocket::State;
use rocket::{get, routes};
use rocket::{Build, Rocket};
use rocket_include_static_resources::{
    cached_static_response_handler, static_resources_initializer,
};
use rocket_include_static_resources::{EtagIfNoneMatch, StaticContextManager, StaticResponse};
use std::path::Path;
use std::sync::{Arc, Mutex};

cached_static_response_handler! {
    259_200;
    "/index.js" => cached_indexjs => "indexjs",
    "/index.css" => cached_indexcss => "indexcss",
    "/favicon.ico" => cached_favicon => "favicon",
}

#[get("/")]
fn index(
    static_resources: &State<StaticContextManager>,
    etag_if_none_match: EtagIfNoneMatch,
) -> StaticResponse {
    static_resources.build(&etag_if_none_match, "index")
}

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
fn sightings(sightings: &State<Arc<Mutex<Vec<Sighting>>>>, start: Option<usize>, end: Option<usize>) -> Json<Vec<Sighting>> {
    let sightings = match sightings.lock() {
        Ok(sightings) => sightings.to_vec(),
        Err(err) => {
            println!("{}", err);
            Vec::new()
        }
    };
    let sightings = match (start, end) {
        (Some(start), Some(end)) => sightings[start.max(0)..end.min(sightings.len())].to_vec(),
        (Some(start), None) => sightings[start..].to_vec(),
        _ => sightings
    };
    Json(sightings)
}

#[get("/webcam")]
fn webcam(capture: &'_ State<Arc<Mutex<WebCam>>>) -> Custom<ByteStream<MJpeg>> {
    let capture: Arc<Mutex<WebCam>> = { capture.inner().clone() };

    Custom(
        ContentType::with_params("multipart", "x-mixed-replace", ("boundary", "frame")),
        ByteStream(MJpeg::new(capture)),
    )
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

pub fn server(sightings: Arc<Mutex<Vec<Sighting>>>, capture: Arc<Mutex<WebCam>>) -> Rocket<Build> {
    rocket::build()
        .attach(static_resources_initializer!(
            "indexjs" => "static/index.js",
            "indexcss" => "static/index.css",
            "favicon" => "static/favicon.ico",
            "index" => ("static", "index.html"),
        ))
        .mount(
            "/",
            routes![cached_indexjs, cached_indexcss, cached_favicon],
        )
        .mount(
            "/",
            routes![index, sightings, sighting, webcam, generate_204],
        )
        .manage(sightings)
        .manage(capture)
}
