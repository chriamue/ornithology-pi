use ornithology_pi::{bluetooth::run_bluetooth, Sighting};
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    let sightings: Vec<Sighting> = vec![Sighting::new("Unknown".to_string())];
    let sightings: Arc<Mutex<Vec<Sighting>>> = Arc::new(Mutex::new(sightings));
    run_bluetooth(sightings).await.unwrap();
}
