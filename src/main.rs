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
    pub sightings: SightingsState,
}

unsafe impl Send for BirdObserver {}
unsafe impl Sync for BirdObserver {}

impl Observer for BirdObserver {
    fn notify(&self, sighting: DataSighting) {
        let mut sightings = self.sightings.mutex.lock().unwrap();
        sightings.push(sighting.0.clone());
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
fn sightings(detector: &State<DetectorState>) -> Json<Vec<Sighting>> {
    let detector = detector.mutex.lock().unwrap();
    let observer = detector.observers().last().unwrap().as_any();

    match observer.downcast_ref::<BirdObserver>() {
        Some(bird_observer) => {
            let sightings = bird_observer.sightings.mutex.lock().unwrap();
            let sightings = sightings.to_vec();
            Json(sightings)
        },
        _ => Json(Vec::new()),
    }
}

#[get("/detect")]
fn detect(detector: &State<DetectorState>) {
    let mut detector = detector.mutex.lock().unwrap();
    detector.detect_next();
}

#[tokio::main]
async fn main() {
    let sightings = SightingsState {
        mutex: Arc::new(Mutex::new(Vec::new())),
    };

    let observer = BirdObserver {
        sightings: sightings,
    };

    let mut birddetector = BirdDetector::default();

    birddetector.register(Box::new(observer));

    let detector = DetectorState {
        mutex: Arc::new(Mutex::new(birddetector)),
    };

    let rocket = rocket::build()
        .mount("/", routes![index, sightings, detect])
        //.manage(sightings)
        .manage(detector);
    rocket.launch().await.unwrap()
}
