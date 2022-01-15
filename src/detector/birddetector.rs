use crate::observer::{Observable, Observer};
use crate::{Capture, Crop, DataSighting, Label, Sighting, WebCam};
use image::DynamicImage;

use super::Detector;

pub struct BirdDetector {
    pub capture: Box<dyn Capture>,
    pub crop: Crop,
    pub label: Label,
    pub subscribers: Vec<Box<dyn Observer>>,
}

unsafe impl Send for BirdDetector {}
unsafe impl Sync for BirdDetector {}

impl Default for BirdDetector {
    fn default() -> Self {
        Self {
            capture: Box::new(WebCam::default()),
            crop: Crop::default(),
            label: Label::default(),
            subscribers: Vec::new(),
        }
    }
}

impl Observable for BirdDetector {
    fn register(&mut self, observer: Box<dyn Observer>) {
        self.subscribers.push(observer);
    }

    fn observers(&self) -> &Vec<Box<dyn Observer>> {
        &self.subscribers
    }
}

impl Detector for BirdDetector {
    fn detect_next(&mut self) {
        let frame = self.capture.frame().unwrap();
        let crop_img = DynamicImage::ImageRgb8(frame.clone());
        let detections = self.crop.crop(crop_img);
        if detections.len() > 0 {
            let detection_frame = detections[0].1.clone();

            let detection = self.label.detect(&detection_frame);
            match detection {
                Some(detection) => {
                    let sighting = Sighting::new(detection);
                    self.subscribers.iter().for_each(|subscriber| {
                        let data_sighting: DataSighting = (sighting.clone(), &detection_frame);
                        subscriber.notify(data_sighting);
                    });
                }
                _ => {}
            }
        }
    }
}
