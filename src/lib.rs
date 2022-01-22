pub mod detector;
pub use detector::BirdDetector;

#[cfg(feature = "bluetooth")]
pub mod bluetooth;
#[cfg(feature = "bluetooth")]
pub use bluetooth::Bluetooth;

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

#[cfg(feature = "server")]
pub mod server;
