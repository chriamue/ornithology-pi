#[cfg(feature = "detect")]
pub mod detector;
#[cfg(feature = "detect")]
pub use detector::BirdDetector;

pub mod capture;
pub use capture::Capture;
pub use capture::WebCam;
pub mod mjpeg;
pub use mjpeg::MJpeg;

#[cfg(feature = "detect")]
pub mod crop;
#[cfg(feature = "detect")]
pub use crop::Crop;
#[cfg(feature = "detect")]
pub mod label;
#[cfg(feature = "detect")]
pub use label::Label;

pub mod sighting;
pub use sighting::{DataSighting, Sighting};

pub mod errors;

pub mod observer;

#[cfg(feature = "bluetooth")]
pub mod bluetooth;

#[cfg(feature = "hotspot")]
pub mod hotspot;
#[cfg(feature = "server")]
pub mod server;

pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Result<T> = std::result::Result<T, Error>;
