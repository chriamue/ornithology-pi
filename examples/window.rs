use image::DynamicImage;
use ornithology_pi::Capture;
use ornithology_pi::Crop;
use show_image::{create_window, event, ImageInfo, ImageView};

#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut capture = Capture::default();
    let frame = capture.frame().unwrap();
    println!("{}, {}", frame.width(), frame.height());
    frame.save("frame.jpg").unwrap();

    let image = ImageView::new(ImageInfo::rgb8(640, 480), &frame);

    let crop = Crop::default();

    let window = create_window("image", Default::default())?;
    window.set_image("image-001", image)?;

    for event in window.event_channel().unwrap() {
        if let event::WindowEvent::KeyboardInput(event) = event {
            if event.input.key_code == Some(event::VirtualKeyCode::Escape)
                && event.input.state.is_pressed()
            {
                break;
            }
            if event.input.key_code == Some(event::VirtualKeyCode::Space)
                && event.input.state.is_pressed()
            {
                let frame = capture.frame().unwrap();
                let crop_img = DynamicImage::ImageRgb8(frame.clone());
                let detections = crop.crop(crop_img);
                if detections.len() > 0 {
                    let frame = detections[0].1.clone().to_rgb8();
                    let image = ImageView::new(ImageInfo::rgb8(640, 480), &frame);
                    window.set_image("image-001", image)?;
                } else {
                    let image = ImageView::new(ImageInfo::rgb8(640, 480), &frame);
                    window.set_image("image-001", image)?;
                }
            }
        }
    }

    Ok(())
}
