use image::DynamicImage;
use ornithology_pi::Crop;

fn main() {
    let mut img = Box::new(
        lenna_core::io::read::read_from_file("assets/Green_Kingfisher_0020_71155.jpg".into())
            .unwrap(),
    );

    let crop = Crop::default();
    let crop_img = DynamicImage::ImageRgba8(img.image.to_rgba8());
    let detections = crop.crop(crop_img);

    let cropped = detections[0].1.clone();
    *img.image = cropped;
    img.name = "crop".to_string();
    lenna_core::io::write::write_to_file(&img, image::ImageOutputFormat::Jpeg(80)).unwrap();
}
