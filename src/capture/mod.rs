use image::{ImageBuffer, Rgb};
use std::error::Error;

pub trait Capture {
    fn frame(&mut self) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>, Box<dyn Error>>;
}

mod webcam;
pub use webcam::WebCam;
