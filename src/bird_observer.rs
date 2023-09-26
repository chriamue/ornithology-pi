use crate::{detect::Detector, observer::Observable, BirdDetector};
use crate::{observer::Observer, DataSighting, Sighting, WebCam};
use std::sync::{Arc, Mutex};
use std::{thread, time};

pub struct BirdObserver {
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
        sighting.0.save("sightings/sightings.db").unwrap();
    }
}

impl Observer for BirdObserver {
    fn notify(&self, sighting: DataSighting) {
        let mut sightings = self.sightings.lock().unwrap();
        sightings.push(sighting.0.clone());
        drop(sightings);
        println!("{:?}", sighting.0.species);
        self.save(sighting);
    }
}

pub async fn run_detector(sightings: Arc<Mutex<Vec<Sighting>>>, capture: Arc<Mutex<WebCam>>) {
    let observer = BirdObserver { sightings };

    let mut birddetector = BirdDetector::new(capture);

    birddetector.register(Box::new(observer));

    let seconds = time::Duration::from_secs(2);

    loop {
        birddetector.detect_next();
        thread::sleep(seconds);
    }
}
