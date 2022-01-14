use image::ImageBuffer;
use image::Rgb;
use nokhwa::Camera;
use nokhwa::CameraFormat;
use nokhwa::FrameFormat;
use std::error::Error;

use crate::errors::NoDevice;

pub struct Capture {
    pub device: Camera,
}

impl Capture {
    pub fn new(width: u32, height: u32) -> Result<Self, Box<dyn Error>> {
        match Camera::new(
            0,
            Some(CameraFormat::new_from(
                width,
                height,
                FrameFormat::MJPEG,
                30,
            )),
        ) {
            Ok(camera) => Ok(Self { device: camera }),
            _ => Err(NoDevice.into()),
        }
    }

    pub fn frame(&mut self) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>, Box<dyn Error>> {
        if !self.device.is_stream_open() {
            self.device.open_stream().unwrap();
        }
        Ok(self.device.frame().unwrap())
    }
}

impl Default for Capture {
    fn default() -> Self {
        Self::new(640, 480).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn default() {
        let mut capture = Capture::default();
        assert!(capture.frame().is_ok());
        assert!(capture.frame().unwrap().width() == 640);
    }
}
