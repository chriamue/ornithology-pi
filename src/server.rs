use crate::BirdDetector;
use crate::Sighting;
use rocket::fs::NamedFile;
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
    pub mutex: Arc<Mutex<BirdDetector>>,
}

#[get("/sightings")]
fn sightings(sightings: &State<Arc<Mutex<Vec<Sighting>>>>) -> Json<Vec<Sighting>> {
    let sightings = sightings.lock().unwrap();
    let sightings = sightings.to_vec();
    Json(sightings)
}

#[get("/sightings/<id>")]
async fn sighting(sightings: &State<Arc<Mutex<Vec<Sighting>>>>, id: String) -> Option<NamedFile> {
    let filename = {
        let sightings = sightings.lock().unwrap();
        let sighting = match sightings
            .iter()
            .filter(|sighting| sighting.uuid == id)
            .last()
        {
            Some(sighting) => Some(sighting.clone()),
            _ => None,
        };
        let sighting = sighting.unwrap();
        format!("{}_{}.jpg", sighting.species, sighting.uuid)
    };

    NamedFile::open(Path::new("sightings/").join(filename))
        .await
        .ok()
}

pub fn server(sightings: Arc<Mutex<Vec<Sighting>>>) -> Rocket<Build> {
    let rocket = rocket::build()
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
        .mount("/", routes![index, sightings, sighting])
        .manage(sightings);
    rocket
}