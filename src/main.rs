#[cfg(feature = "bluetooth")]
use ornithology_pi::bluetooth::run_bluetooth;
use ornithology_pi::config;
#[cfg(feature = "hotspot")]
use ornithology_pi::hotspot::Hotspot;
#[cfg(feature = "server")]
use ornithology_pi::server::server;
#[cfg(feature = "detect")]
use ornithology_pi::{detector::Detector, observer::Observable, BirdDetector};
use ornithology_pi::{observer::Observer, DataSighting, Sighting, WebCam};
use std::sync::{Arc, Mutex};
#[cfg(feature = "detect")]
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
        sighting.0.save("sightings/sightings.db").unwrap();
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

#[cfg(feature = "detect")]
async fn run_detector(sightings: Arc<Mutex<Vec<Sighting>>>, capture: Arc<Mutex<WebCam>>) {
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
    let config = config::load_config();
    let sightings: Arc<Mutex<Vec<Sighting>>> = Arc::new(Mutex::new(
        ornithology_pi::sighting::load_from_file("sightings/sightings.db").unwrap_or_default(),
    ));
    let capture: Arc<Mutex<WebCam>> = Arc::new(Mutex::new(
        WebCam::new(config.camera.width, config.camera.height, config.camera.fps).unwrap(),
    ));

    println!("Loaded Config: {:?}", config);

    #[cfg(feature = "bluetooth")]
    let bluetooth_thread = tokio::spawn(run_bluetooth(sightings.clone()));
    #[cfg(feature = "detect")]
    let detector_thread = tokio::spawn(run_detector(sightings.clone(), capture.clone()));

    #[cfg(feature = "hotspot")]
    let mut hotspot = Hotspot::default();
    #[cfg(feature = "hotspot")]
    hotspot.start();
    #[cfg(feature = "server")]
    {
        let launcher = server(sightings.clone(), capture.clone());
        launcher.launch().await.unwrap();
    }

    #[cfg(feature = "bluetooth")]
    bluetooth_thread.abort();
    #[cfg(feature = "detect")]
    detector_thread.abort();

    #[cfg(feature = "hotspot")]
    hotspot.stop();
}
