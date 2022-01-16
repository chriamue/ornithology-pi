#[macro_use]
extern crate rocket;
#[macro_use]
extern crate rocket_include_static_resources;

use ornithology_pi::{detector::Detector, BirdDetector};
use ornithology_pi::{
    observer::{Observable, Observer},
    DataSighting, Sighting,
};
use rocket::fs::NamedFile;
use rocket::serde::json::Json;
use rocket::State;
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

struct BirdObserver {
    pub sightings: Arc<Mutex<Vec<Sighting>>>,
}

unsafe impl Send for BirdObserver {}
unsafe impl Sync for BirdObserver {}

impl BirdObserver {
    fn save(&self, sighting: DataSighting) {
        let image = sighting.1;
        image
            .save(format!(
                "sightings/{}_{}.jpg",
                sighting.0.species, sighting.0.uuid
            ))
            .unwrap();
    }

    fn get(&self, id: String) -> Option<Sighting> {
        let sightings = self.sightings.lock().unwrap();
        match sightings
            .iter()
            .filter(|sighting| sighting.uuid == id)
            .last()
        {
            Some(sighting) => Some(sighting.clone()),
            _ => None,
        }
    }
}

impl Observer for BirdObserver {
    fn notify(&self, sighting: DataSighting) {
        let mut sightings = self.sightings.lock().unwrap();
        sightings.push(sighting.0.clone());
        println!("{:?}", sighting.0.species);
        self.save(sighting);
    }
}

#[derive(Clone)]
pub struct DetectorState {
    pub mutex: Arc<Mutex<BirdDetector>>,
}

#[get("/sightings")]
fn sightings(detector: &State<DetectorState>) -> Json<Vec<Sighting>> {
    let detector = detector.mutex.lock().unwrap();
    let observer = detector.observers().last().unwrap().as_any();

    match observer.downcast_ref::<BirdObserver>() {
        Some(bird_observer) => {
            let sightings = bird_observer.sightings.lock().unwrap();
            let sightings = sightings.to_vec();
            Json(sightings)
        }
        _ => Json(Vec::new()),
    }
}

#[get("/sightings/<id>")]
async fn sighting(detector: &State<DetectorState>, id: String) -> Option<NamedFile> {
    let filename = {
        let detector = detector.mutex.lock().unwrap();
        let observer = detector.observers().last().unwrap().as_any();

        let obs = observer.downcast_ref::<BirdObserver>().unwrap();
        let sighting = obs.get(id).unwrap().clone();
        format!("{}_{}.jpg", sighting.species, sighting.uuid)
    };

    NamedFile::open(Path::new("sightings/").join(filename))
        .await
        .ok()
}

#[get("/detect")]
fn detect(detector: &State<DetectorState>) {
    let mut detector = detector.mutex.lock().unwrap();
    detector.detect_next();
}

#[tokio::main]
async fn main() {
    let sightings: Arc<Mutex<Vec<Sighting>>> = Arc::new(Mutex::new(Vec::new()));

    let observer = BirdObserver {
        sightings: sightings,
    };

    let mut birddetector = BirdDetector::default();

    birddetector.register(Box::new(observer));

    let detector = DetectorState {
        mutex: Arc::new(Mutex::new(birddetector)),
    };

    let rocket = rocket::build()
        .attach(static_resources_initializer!(
            "indexjs" => "static/index.js",
            "indexcss" => "static/index.css",
            "favicon" => "static/favicon.ico",
            "index" => ("static", "index.html"),
        ))
        .mount("/", routes![cached_indexjs, cached_indexcss, cached_favicon])
        .mount("/", routes![index, sightings, sighting, detect])
        .manage(detector);
    rocket.launch().await.unwrap()
}
