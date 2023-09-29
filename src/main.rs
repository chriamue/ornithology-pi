#[cfg(feature = "detect")]
use ornithology_pi::bird_observer::run_detector;
#[cfg(feature = "bluetooth")]
use ornithology_pi::bluetooth::run_bluetooth;
use ornithology_pi::cli::Cli;
use ornithology_pi::config;
#[cfg(feature = "hotspot")]
use ornithology_pi::hotspot::Hotspot;
#[cfg(feature = "server")]
use ornithology_pi::server::server;
use ornithology_pi::{Sighting, WebCam};
use std::sync::{Arc, Mutex};

fn init_logger(cli: &Cli) {
    // default log level is info
    if let Err(_) = std::env::var("RUST_LOG") {
        std::env::set_var("RUST_LOG", "info");
    }
    // override log level if set in cli
    if let Some(log_level) = cli.log_level.as_ref() {
        std::env::set_var("RUST_LOG", log_level);
    }
    pretty_env_logger::init_timed();
}

#[tokio::main]
async fn main() {
    let cli = Cli::new();
    init_logger(&cli);

    cli.evaluate();

    let config = config::load_config();
    let config = config::merge_cli_config(&config, &cli);

    let sightings: Arc<Mutex<Vec<Sighting>>> = Arc::new(Mutex::new(
        ornithology_pi::sighting::load_from_file("sightings/sightings.db").unwrap_or_default(),
    ));
    let capture: Arc<Mutex<WebCam>> = Arc::new(Mutex::new(
        WebCam::new(config.camera.width, config.camera.height, config.camera.fps).unwrap(),
    ));

    log::info!("Loaded Config: {:?}", config);

    #[cfg(feature = "bluetooth")]
    let bluetooth_thread = tokio::spawn(run_bluetooth(sightings.clone()));
    #[cfg(feature = "detect")]
    let detector_thread = tokio::spawn(run_detector(sightings.clone(), capture.clone()));

    #[cfg(feature = "hotspot")]
    let mut hotspot = Hotspot::default();
    #[cfg(feature = "hotspot")]
    hotspot.start();
    #[cfg(feature = "server")]
    if cli.server.unwrap_or(false) {
        let launcher = server(&config, sightings.clone(), capture.clone());
        let _launched = launcher.await;
    }

    #[cfg(feature = "bluetooth")]
    bluetooth_thread.abort();
    #[cfg(feature = "detect")]
    detector_thread.abort();

    #[cfg(feature = "hotspot")]
    hotspot.stop();
}
