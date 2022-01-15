#[macro_use]
extern crate rocket;

use ornithology_pi::{detector::Detector, BirdDetector};
use ornithology_pi::{
    observer::{Observable, Observer},
    DataSighting, Sighting,
};
use rocket::serde::json::Json;
use rocket::State;
use std::sync::{Arc, Mutex};

struct BirdObserver {
    //sightings: &'static mut SightingsState,
}

unsafe impl Send for BirdObserver {}
unsafe impl Sync for BirdObserver {}

impl Observer for BirdObserver {
    fn notify(&self, sighting: DataSighting) {
        println!("{:?}", sighting.0.species);
    }
}

#[get("/")]
fn index() -> &'static str {
    "Hello, world!"
}

#[derive(Debug, Clone)]
pub struct SightingsState {
    pub mutex: Arc<Mutex<Vec<Sighting>>>,
}

#[derive(Clone)]
pub struct DetectorState {
    pub mutex: Arc<Mutex<BirdDetector>>,
}

#[get("/sightings")]
fn sightings(sightings: &State<SightingsState>) -> Json<Vec<Sighting>> {
    let sightings: Vec<Sighting> = sightings.mutex.lock().unwrap().to_vec();

    Json(sightings)
}

#[get("/detect")]
fn detect(detector: &State<DetectorState>) {
    let mut detector = detector.mutex.lock().unwrap();
    detector.detect_next();
}

#[tokio::main]
async fn main() {
    let mut sightings = SightingsState {
        mutex: Arc::new(Mutex::new(Vec::new())),
    };

    let observer = BirdObserver {
        //sightings: &mut sightings,
    };

    let mut birddetector = BirdDetector::default();

    birddetector.register(Box::new(observer));

    let detector = DetectorState {
        mutex: Arc::new(Mutex::new(birddetector)),
    };

    let rocket = rocket::build()
        .mount("/", routes![index, sightings, detect])
        .manage(sightings)
        .manage(detector);
    rocket.launch().await.unwrap()
}
