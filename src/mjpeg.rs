use crate::{Capture, WebCam};
use format_bytes::format_bytes;
use futures::stream::Stream;
use std::error::Error as StdError;
use std::fmt;
use std::pin::Pin;
use std::sync::{Arc, Mutex};
use std::task::Context;
use std::task::Poll;
use std::thread;
use tokio::time;

const FRAME_MILLIS: u32 = 1000 / 2;

#[derive(Debug)]
pub struct MJpegError;

impl fmt::Display for MJpegError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "An error occurred in MJpeg")
    }
}

impl StdError for MJpegError {}

pub struct MJpeg {
    capture: Arc<Mutex<WebCam>>,
}

impl MJpeg {
    pub fn new(capture: Arc<Mutex<WebCam>>) -> Self {
        Self { capture }
    }
}

impl Stream for MJpeg {
    type Item = std::result::Result<Vec<u8>, Box<dyn StdError + Send + Sync>>;

    fn poll_next(self: Pin<&mut Self>, _cx: &mut Context<'_>) -> Poll<Option<Self::Item>> {
        let start = time::Instant::now();

        let buf: Vec<u8> = match self.capture.lock() {
            Ok(mut capture) => match capture.bytes_jpeg() {
                Ok(buf) => buf,
                Err(_) => return Poll::Ready(Some(Err(Box::new(MJpegError)))),
            },
            Err(_) => return Poll::Ready(Some(Err(Box::new(MJpegError)))),
        };

        let data = format_bytes!(b"\r\n--frame\r\nContent-Type: image/jpeg\r\n\r\n{}", &buf);
        let duration = time::Instant::now() - start;
        thread::sleep(time::Duration::from_millis(
            (FRAME_MILLIS as i32 - duration.as_millis() as i32).max(0) as u64,
        ));
        Poll::Ready(Some(Ok(data)))
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (0, None)
    }
}
