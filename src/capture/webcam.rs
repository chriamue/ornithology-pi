use futures::Stream;
use image::ImageBuffer;
use image::Rgb;
use nokhwa::CameraFormat;
use nokhwa::FrameFormat;
use nokhwa::ThreadedCamera;
use std::error::Error;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::Context;
use std::task::Poll;

use super::Capture;

fn callback(_image: ImageBuffer<Rgb<u8>, Vec<u8>>) {}

pub struct WebCam {
    pub width: u32,
    pub height: u32,
    pub fps: u32,
    running: Arc<Mutex<bool>>,
    device: ThreadedCamera,
}

impl WebCam {
    pub fn new(width: u32, height: u32, fps: u32) -> Result<Self, Box<dyn Error>> {
        let running = Arc::new(Mutex::new(false));
        let mut device = ThreadedCamera::new(
            0,
            Some(CameraFormat::new_from(
                width,
                height,
                FrameFormat::YUYV,
                fps,
            )),
        )
        .unwrap();
        device.open_stream(callback).unwrap();

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
        let frame = self.device.last_frame();
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
