use image::{imageops::crop, DynamicImage, GenericImageView};
use lenna_yolo_plugin::{detection::Detection, Yolo};

const THREASHOLD: f32 = 0.5;
const BIRD_CLASS: usize = 2;
const BORDER: u32 = 50;

fn main() {
    let yolo = Yolo::default();
    let mut img = Box::new(
        lenna_core::io::read::read_from_file("assets/Green_Kingfisher_0020_71155.jpg".into())
            .unwrap(),
    );

    let detections = yolo.detect_objects(&img.image).unwrap();

    let bird_detections: Vec<&Detection> = detections
        .iter()
        .filter(|&detection| detection.class == BIRD_CLASS && detection.confidence > THREASHOLD)
        .collect();
    let mut crop_img = DynamicImage::ImageRgba8(img.image.to_rgba8());
    let (width, height) = crop_img.dimensions();
    let bbox = Yolo::scale(width, height, &bird_detections[0].bbox);
    let cropped = crop(
        &mut crop_img,
        bbox.left() as u32 - BORDER,
        bbox.top() as u32 - BORDER,
        bbox.width() as u32 + 2 * BORDER,
        bbox.height() as u32 + 2 * BORDER,
    );
    *img.image = DynamicImage::ImageRgba8(cropped.to_image());
    img.name = "crop".to_string();
    lenna_core::io::write::write_to_file(&img, image::ImageOutputFormat::Jpeg(80)).unwrap();
}
