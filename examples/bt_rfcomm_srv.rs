use ornithology_pi::{bluetooth::rfcomm_srv::run, Sighting};
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    let sightings: Vec<Sighting> = vec![Sighting::new("Unknown".to_string())];
    let sightings: Arc<Mutex<Vec<Sighting>>> = Arc::new(Mutex::new(sightings));
    run(sightings).await.unwrap();
}
