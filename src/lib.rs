// Publicly re-exporting types for external use.
pub use self::{
    capture::Capture,
    config::Config,
    sighting::{DataSighting, Sighting},
};

#[cfg(feature = "webcam")]
pub use self::{capture::WebCam, mjpeg::MJpeg};

// Type aliases for convenience.
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

// Modules related to capturing and processing images.
pub mod capture;
pub mod cli;
pub mod config;
pub mod logger;

#[cfg(feature = "webcam")]
pub mod mjpeg;
pub mod sighting;

// Error handling module.
pub mod errors;

// Observer module.
pub mod observer;

#[cfg(feature = "detect")]
pub mod bird_observer;
#[cfg(feature = "detect")]
pub mod detect;

#[cfg(feature = "detect")]
pub use self::{
    bird_observer::run_detector, bird_observer::BirdObserver, detect::BirdDetector, detect::Crop,
    detect::Label,
};

// Bluetooth feature module.
#[cfg(feature = "bluetooth")]
pub mod bluetooth;

// Hotspot feature module.
#[cfg(feature = "hotspot")]
pub mod hotspot;

// Server feature module.
#[cfg(feature = "server")]
pub mod server;
