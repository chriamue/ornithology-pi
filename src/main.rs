use ornithology_pi::{detector::Detector, BirdDetector};
use ornithology_pi::{
    observer::{Observable, Observer},
    DataSighting, Sighting,
};
use std::sync::{Arc, Mutex};
use std::{thread, time};

use ornithology_pi::server::server;

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
}

impl Observer for BirdObserver {
    fn notify(&self, sighting: DataSighting) {
        let mut sightings = self.sightings.lock().unwrap();
        sightings.push(sighting.0.clone());
        println!("{:?}", sighting.0.species);
        self.save(sighting);
    }
}

async fn run_detector(sightings: Arc<Mutex<Vec<Sighting>>>) -> () {
    let observer = BirdObserver {
        sightings: sightings,
    };

    let mut birddetector = BirdDetector::default();

    birddetector.register(Box::new(observer));

    let seconds = time::Duration::from_secs(2);

    loop {
        birddetector.detect_next();
        thread::sleep(seconds);
    }
}

#[tokio::main]
async fn main() {
    let sightings: Arc<Mutex<Vec<Sighting>>> = Arc::new(Mutex::new(Vec::new()));
    let detector_thread = tokio::spawn(run_detector(sightings.clone()));
    let launcher = server(sightings.clone());
    launcher.launch().await.unwrap();
    detector_thread.abort();
}
