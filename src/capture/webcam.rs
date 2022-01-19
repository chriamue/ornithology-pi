use image::ImageBuffer;
use image::Rgb;
use nokhwa::Camera;
use nokhwa::CameraFormat;
use nokhwa::FrameFormat;
use std::error::Error;

use super::Capture;
use crate::errors::NoDevice;

pub struct WebCam {
    pub device: Camera,
}

impl WebCam {
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
}

impl Default for WebCam {
    fn default() -> Self {
        Self::new(1920, 1080).unwrap()
    }
}

impl Capture for WebCam {
    fn frame(&mut self) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>, Box<dyn Error>> {
        if !self.device.is_stream_open() {
            self.device.open_stream().unwrap();
        }
        self.device.frame().unwrap();
        self.device.frame().unwrap();
        self.device.frame().unwrap();
        self.device.frame().unwrap();
        Ok(self.device.frame().unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[ignore]
    #[test]
    fn default() {
        let mut capture = WebCam::default();
        assert!(capture.frame().is_ok());
        assert!(capture.frame().unwrap().width() == 640);
    }
}
