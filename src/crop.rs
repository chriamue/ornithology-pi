use image::{imageops, DynamicImage, GenericImageView};
use lenna_yolo_plugin::{detection::Detection, Yolo};

const THREASHOLD: f32 = 0.5;
const BIRD_CLASS: usize = 2;
const BORDER: u32 = 50;

pub struct Crop {
    threashold: f32,
    class: usize,
    border: u32,
}

impl Crop {
    pub fn crop(&self, image: DynamicImage) -> Vec<(Detection, DynamicImage)> {
        let (width, height) = image.dimensions();
        let detections = self.detect(&image);
        let detections: Vec<(Detection, DynamicImage)> = detections
            .iter()
            .clone()
            .map(|detection| {
                let bbox = Yolo::scale(width, height, &detection.bbox);
                let mut crop_img = image.clone();
                let cropped = imageops::crop(
                    &mut crop_img,
                    bbox.left() as u32 - self.border,
                    bbox.top() as u32 - self.border,
                    bbox.width() as u32 + 2 * self.border,
                    bbox.height() as u32 + 2 * self.border,
                );
                (
                    detection.clone(),
                    DynamicImage::ImageRgba8(cropped.to_image()),
                )
            })
            .collect();
        detections
    }

    pub fn detect(&self, image: &DynamicImage) -> Vec<Detection> {
        let yolo = Yolo::default();
        let detections = yolo.detect_objects(&Box::new(image.clone())).unwrap();

        let class_detections: Vec<Detection> = detections
            .iter()
            .filter(|&detection| {
                detection.class == self.class && detection.confidence > self.threashold
            })
            .clone()
            .map(|d| d.clone())
            .collect();
        class_detections
    }
}

impl Default for Crop {
    fn default() -> Self {
        Crop {
            threashold: THREASHOLD,
            class: BIRD_CLASS,
            border: BORDER,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default() {
        let img = Box::new(
            lenna_core::io::read::read_from_file("assets/Green_Kingfisher_0020_71155.jpg".into())
                .unwrap(),
        );
        let crop_img = DynamicImage::ImageRgba8(img.image.to_rgba8());
        let (width, height) = crop_img.dimensions();

        let crop = Crop::default();
        let detections = crop.crop(crop_img);
        assert!(detections.len() > 0);
        assert!(detections[0].1.width() < width);
        assert!(detections[0].1.height() < height);
    }
}
