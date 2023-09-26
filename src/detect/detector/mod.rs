pub trait Detector {
    fn detect_next(&mut self);
}

mod birddetector;
pub use birddetector::BirdDetector;
