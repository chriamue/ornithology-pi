use ornithology_pi::{bluetooth::rfcomm_srv::run, Sighting};
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    let mut sightings: Vec<Sighting> =
        ornithology_pi::sighting::load_from_file("sightings/sightings.db").unwrap_or_default();
    if sightings.len() < 1 {
        sightings.push(Sighting::new("Unknown".to_string()))
    }
    let sightings: Arc<Mutex<Vec<Sighting>>> = Arc::new(Mutex::new(sightings));
    run(sightings).await.unwrap();
}
