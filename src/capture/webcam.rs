use futures::Stream;
use image::ImageBuffer;
use image::Rgb;
use nokhwa::nokhwa_initialize;
use nokhwa::pixel_format::RgbFormat;
use nokhwa::utils::CameraIndex;
use nokhwa::{
    query,
    utils::{ApiBackend, CameraFormat, FrameFormat, RequestedFormat, RequestedFormatType},
    Buffer, CallbackCamera,
};
use std::error::Error;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::Context;
use std::task::Poll;

use super::Capture;

fn callback(_image: Buffer) {}

pub struct WebCam {
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    running: Arc<Mutex<bool>>,
    device: CallbackCamera,
}

impl WebCam {
    pub fn new(width: u32, height: u32, fps: u32) -> Result<Self, Box<dyn Error>> {
        let running = Arc::new(Mutex::new(false));
        nokhwa_initialize(|granted| {
            println!("Camera access granted {}", granted);
        });

        let cameras = query(ApiBackend::Auto).unwrap();
        cameras.iter().for_each(|cam| println!("{:?}", cam));

        //let format = RequestedFormat::new::<RgbFormat>(RequestedFormatType::None);
        let format = RequestedFormat::new::<RgbFormat>(RequestedFormatType::Exact(
            CameraFormat::new_from(width, height, FrameFormat::MJPEG, fps),
        ));

        // let camera_index = cameras.first().unwrap().index().clone(); // is video1 not video0
        let camera_index = CameraIndex::Index(0 as u32);

        let mut device = CallbackCamera::new(camera_index, format, callback).unwrap();
        device.open_stream()?;

        let mut webcam = Self {
            width,
            height,
            fps,
            running,
            device,
        };
        webcam.start();
        Ok(webcam)
    }

    pub fn start(&mut self) {
        let running = self.running.clone();
        *running.lock().unwrap() = true;
    }

    pub fn stop(&mut self) {
        *self.running.lock().unwrap() = false;
    }
}

impl Default for WebCam {
    fn default() -> Self {
        Self::new(640, 480, 30).unwrap()
    }
}

impl Capture for WebCam {
    fn frame(&mut self) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>, Box<dyn Error>> {
        let buffer = self.device.last_frame()?;
        let frame = buffer.decode_image::<RgbFormat>()?;
        Ok(frame)
    }
}

impl Stream for WebCam {
    type Item = ImageBuffer<Rgb<u8>, Vec<u8>>;
    fn poll_next(mut self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        if *self.running.lock().unwrap() {
            Poll::Ready(Some(self.frame().unwrap()))
        } else {
            Poll::Ready(None)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use futures::StreamExt;

    #[ignore]
    #[test]
    fn default() {
        let mut capture = WebCam::default();
        assert!(capture.frame().is_ok());
        assert!(capture.frame().unwrap().width() == 1920);
    }

    #[ignore = "blocked webcam"]
    #[tokio::test]
    async fn stream_started() {
        let mut webcam = WebCam::default();
        webcam.stop();
        let stream = webcam.next().await;
        assert!(stream.is_none());
        webcam.start();
        let stream = webcam.next().await;
        assert!(stream.is_some());
    }

    #[tokio::test]
    async fn stream_stopped() {
        let mut webcam = WebCam::new(640, 480, 30).unwrap();
        webcam.stop();
        let stream = webcam.next().await;
        assert!(stream.is_none());
    }
}
