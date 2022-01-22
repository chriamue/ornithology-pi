pub mod detector;
pub use detector::BirdDetector;

pub mod capture;
pub use capture::Capture;
pub use capture::WebCam;

pub mod crop;
pub use crop::Crop;
pub mod label;
pub use label::Label;

pub mod sighting;
pub use sighting::{DataSighting, Sighting};

pub mod errors;
pub mod observer;

#[cfg(feature = "bluetooth")]
pub mod bluetooth;
#[cfg(feature = "server")]
pub mod server;
