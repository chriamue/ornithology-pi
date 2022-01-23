use image::{ImageBuffer, Rgb};
#[cfg(feature = "bluetooth")]
use ornithology_pi::bluetooth::run_bluetooth;
#[cfg(feature = "server")]
use ornithology_pi::server::server;
use ornithology_pi::{detector::Detector, BirdDetector};
use ornithology_pi::{
    observer::{Observable, Observer},
    Capture, DataSighting, Sighting, WebCam,
};
use std::sync::{Arc, Mutex};
use std::{thread, time};

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

async fn run_detector(sightings: Arc<Mutex<Vec<Sighting>>>) {
    let capture: Arc<Mutex<dyn Capture<Item = ImageBuffer<Rgb<u8>, Vec<u8>>>>> =
        Arc::new(Mutex::new(WebCam::default()));
    let observer = BirdObserver { sightings };

    let mut birddetector = BirdDetector::new(capture);

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
    let _capture: Arc<Mutex<dyn Capture<Item = ImageBuffer<Rgb<u8>, Vec<u8>>>>> =
        Arc::new(Mutex::new(WebCam::default()));

    #[cfg(feature = "bluetooth")]
    let bluetooth_thread = tokio::spawn(run_bluetooth(sightings.clone()));
    let detector_thread = tokio::spawn(run_detector(sightings.clone()));

    #[cfg(feature = "server")]
    {
        let launcher = server(sightings.clone());
        launcher.launch().await.unwrap();
    }

    #[cfg(feature = "bluetooth")]
    bluetooth_thread.abort();
    detector_thread.abort();
}
