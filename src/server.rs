use crate::BirdDetector;
use crate::{Capture, Sighting, WebCam};
use format_bytes::format_bytes;
use image::{DynamicImage, ImageBuffer, Rgb};
use rocket::fs::NamedFile;
use rocket::http::ContentType;
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
use std::time::Duration;
use tokio::time;

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
    pub mutex: Arc<Mutex<BirdDetector>>,
}

#[get("/sightings")]
fn sightings(sightings: &State<Arc<Mutex<Vec<Sighting>>>>) -> Json<Vec<Sighting>> {
    let sightings = sightings.lock().unwrap();
    let sightings = sightings.to_vec();
    Json(sightings)
}

#[get("/webcam")]
fn webcam() -> Custom<ByteStream![Vec<u8>]> {
    let mut capture = WebCam::default();

    Custom(
        ContentType::with_params("multipart", "x-mixed-replace", ("boundary", "frame")),
        ByteStream! {
            let mut interval = time::interval(Duration::from_millis(50));
            loop {
                interval.tick();
                let base_img: ImageBuffer<Rgb<u8>, Vec<u8>> = capture.frame().unwrap();
                let base_img: DynamicImage = DynamicImage::ImageRgb8(base_img);
                let mut buf = vec![];
                base_img.write_to(&mut buf, image::ImageOutputFormat::Jpeg(60));
                let data = format_bytes!(b"\r\n--frame\r\nContent-Type: image/jpeg\r\n\r\n{}", &buf);
                yield data
            }
        },
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

pub fn server(sightings: Arc<Mutex<Vec<Sighting>>>) -> Rocket<Build> {
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
        .mount("/", routes![index, sightings, sighting, webcam])
        .manage(sightings)
}
