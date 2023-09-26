// Publicly re-exporting types for external use.
pub use self::{
    capture::{Capture, WebCam},
    config::Config,
    mjpeg::MJpeg,
    sighting::{DataSighting, Sighting},
};

// Type aliases for convenience.
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;

// Modules related to capturing and processing images.
pub mod capture;
pub mod config;
pub mod mjpeg;
pub mod sighting;

// Error handling module.
pub mod errors;

// Observer module.
pub mod observer;

#[cfg(feature = "detect")]
pub mod detect;

// Feature-specific modules and exports.
#[cfg(feature = "detect")]
pub use self::{detect::BirdDetector, detect::Crop, detect::Label};

// Bluetooth feature module.
#[cfg(feature = "bluetooth")]
pub mod bluetooth;

// Hotspot feature module.
#[cfg(feature = "hotspot")]
pub mod hotspot;

// Server feature module.
#[cfg(feature = "server")]
pub mod server;
