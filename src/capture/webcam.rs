use image::ImageBuffer;
use image::Rgb;
use nokhwa::Camera;
use nokhwa::CameraFormat;
use nokhwa::FrameFormat;
use std::error::Error;
use std::sync::{Arc, Mutex};
use std::thread;
use std::time::Duration;

use super::Capture;

pub struct WebCam {
    frame: Arc<Mutex<ImageBuffer<Rgb<u8>, Vec<u8>>>>,
    running: Arc<Mutex<bool>>,
}

impl WebCam {
    pub fn new(width: u32, height: u32) -> Result<Self, Box<dyn Error>> {
        let frame = Arc::new(Mutex::new(ImageBuffer::new(width, height)));
        let running = Arc::new(Mutex::new(false));

        let mut webcam = Self { frame, running };
        webcam.start();
        Ok(webcam)
    }

    fn capture(device: &mut Camera, frame: Arc<Mutex<ImageBuffer<Rgb<u8>, Vec<u8>>>>) {
        if !device.is_stream_open() {
            device.open_stream().unwrap();
        }
        let new_frame = device.frame().unwrap();
        *frame.lock().unwrap() = new_frame;
    }

    pub fn start(&mut self) {
        let width = self.frame.lock().unwrap().width();
        let height = self.frame.lock().unwrap().height();
        let running = self.running.clone();
        let frame = self.frame.clone();

        thread::spawn(move || {
            let mut camera = Camera::new(
                0,
                Some(CameraFormat::new_from(
                    width,
                    height,
                    FrameFormat::MJPEG,
                    30,
                )),
            )
            .unwrap();
            *running.lock().unwrap() = true;
            loop {
                if *running.lock().unwrap() == false {
                    break;
                }
                Self::capture(&mut camera, frame.clone());
                thread::sleep(Duration::from_millis(20));
            }
        });
    }

    pub fn stop(&mut self) {
        *self.running.lock().unwrap() = false;
    }
}

impl Default for WebCam {
    fn default() -> Self {
        Self::new(1920, 1080).unwrap()
    }
}

impl Capture for WebCam {
    fn frame(&mut self) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>, Box<dyn Error>> {
        Ok(self.frame.lock().unwrap().clone())
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
        assert!(capture.frame().unwrap().width() == 1920);
    }
}
