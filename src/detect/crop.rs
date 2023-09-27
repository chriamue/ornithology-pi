use image::{imageops, DynamicImage, GenericImageView};
use lenna_yolo_plugin::{detection::Detection, Yolo};

const THREASHOLD: f32 = 0.5;

#[cfg(feature = "yolo")]
const BIRD_CLASS: usize = 2;
#[cfg(feature = "yolov8")]
const BIRD_CLASS: usize = 14;

const BORDER: u32 = 50;

#[derive(Clone)]
pub struct Crop {
    threashold: f32,
    class: usize,
    border: u32,
    yolo: Yolo,
}

impl Crop {
    pub fn crop(&self, image: DynamicImage) -> Vec<(Detection, DynamicImage)> {
        let (width, height) = image.dimensions();
        if let (0, 0) = (width, height) {
            return Vec::new();
        }
        let detections = self.detect(&image);
        let detections: Vec<(Detection, DynamicImage)> = detections
            .iter()
            .clone()
            .map(|detection| {
                let bbox = Yolo::scale(width, height, &detection.bbox);
                let mut crop_img = image.clone();
                let x = 0.max(bbox.left() - self.border as i32) as u32;
                let y = 0.max(bbox.top() - self.border as i32) as u32;
                let cropped = imageops::crop(
                    &mut crop_img,
                    x,
                    y,
                    bbox.width() as u32 + 2 * self.border,
                    bbox.height() as u32 + 2 * self.border,
                );
                (*detection, DynamicImage::ImageRgba8(cropped.to_image()))
            })
            .collect();
        detections
    }

    pub fn detect(&self, image: &DynamicImage) -> Vec<Detection> {
        let detections = self.yolo.detect_objects(&Box::new(image.clone())).unwrap();

        let class_detections: Vec<Detection> = detections
            .iter()
            .filter(|&detection| {
                detection.class == self.class && detection.confidence > self.threashold
            })
            .clone()
            .copied()
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
            yolo: Yolo::default(),
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
        assert!(!detections.is_empty());
        assert!(detections[0].1.width() < width);
        assert!(detections[0].1.height() < height);
    }
}
