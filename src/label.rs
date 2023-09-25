use image::DynamicImage;
use lenna_birds_plugin::Birds;

#[derive(Clone)]
pub struct Label {
    model: Birds,
}

impl Label {
    pub fn detect(&self, image: &DynamicImage) -> Option<String> {
        if image.width() < 1 || image.height() < 1 {
            return None;
        }

        self.model
            .detect_label(&Box::new(image.clone()))
            .unwrap_or(None)
    }
}

impl Default for Label {
    fn default() -> Self {
        Self {
            model: Birds::default(),
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
        let labeler = Label::default();
        let detection = labeler.detect(&img.image);
        assert!(detection.is_some());
    }
}
