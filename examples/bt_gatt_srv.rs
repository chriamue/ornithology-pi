use log::LevelFilter;
use ornithology_pi::{bluetooth::gatt_srv::run, Sighting};
use std::sync::{Arc, Mutex};

#[tokio::main]
async fn main() {
    pretty_env_logger::formatted_builder()
        .filter_level(LevelFilter::Debug)
        .init();
    let sightings: Vec<Sighting> = vec![Sighting::new("Unknown".to_string())];
    let sightings: Arc<Mutex<Vec<Sighting>>> = Arc::new(Mutex::new(sightings));
    run(sightings).await.unwrap();
}
