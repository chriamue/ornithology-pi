use format_bytes::format_bytes;
use futures::Stream;
use image::DynamicImage;
use image::ImageBuffer;
use image::Rgb;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::Context;
use std::task::Poll;
use std::thread;
use tokio::time;

use crate::{Capture, WebCam};

pub struct MJpeg {
    capture: Arc<Mutex<WebCam>>,
}

impl MJpeg {
    pub fn new(capture: Arc<Mutex<WebCam>>) -> Self {
        Self { capture }
    }
}

impl Stream for MJpeg {
    type Item = Vec<u8>;

    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        thread::sleep(time::Duration::from_millis(50));

        let base_img: ImageBuffer<Rgb<u8>, Vec<u8>> = {
            let mut capture = self.capture.lock().unwrap();
            capture.frame().unwrap()
        };
        let base_img: DynamicImage = DynamicImage::ImageRgb8(base_img);
        let mut buf = vec![];
        base_img
            .write_to(&mut buf, image::ImageOutputFormat::Jpeg(60))
            .unwrap();
        let data = format_bytes!(b"\r\n--frame\r\nContent-Type: image/jpeg\r\n\r\n{}", &buf);
        Poll::Ready(Some(data))
    }
}
