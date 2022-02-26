use futures::Stream;
use image::{DynamicImage, ImageBuffer, Rgb};
use std::error::Error;

pub trait Capture: Stream {
    fn frame(&mut self) -> Result<ImageBuffer<Rgb<u8>, Vec<u8>>, Box<dyn Error>>;
    fn bytes_jpeg(&mut self) -> Result<Vec<u8>, Box<dyn Error>> {
        let base_img = self.frame()?;
        let base_img: DynamicImage = DynamicImage::ImageRgb8(base_img);
        let mut buf = vec![];
        base_img
            .write_to(&mut buf, image::ImageOutputFormat::Jpeg(70))
            .unwrap();
        Ok(buf)
    }
}

mod webcam;
pub use webcam::WebCam;
