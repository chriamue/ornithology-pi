use format_bytes::format_bytes;
use futures::Stream;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::Context;
use std::task::Poll;
use std::thread;
use tokio::time;

const FRAME_MILLIS: u32 = 1000 / 2;

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
        let start = time::Instant::now();

        let buf: Vec<u8> = {
            let mut capture = self.capture.lock().unwrap();
            capture.bytes_jpeg().unwrap()
        };
        let data = format_bytes!(b"\r\n--frame\r\nContent-Type: image/jpeg\r\n\r\n{}", &buf);
        let duration = time::Instant::now() - start;
        thread::sleep(time::Duration::from_millis(
            (FRAME_MILLIS as i32 - duration.as_millis() as i32).max(0) as u64,
        ));
        Poll::Ready(Some(data))
    }
}
