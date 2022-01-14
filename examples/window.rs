#[cfg(feature = "window")]
use image::DynamicImage;
#[cfg(feature = "window")]
use ornithology_pi::{Capture, Crop, Label};
#[cfg(feature = "window")]
use show_image::{create_window, event, ImageInfo, ImageView};

#[cfg(feature = "window")]
#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut capture = Capture::default();
    let frame = capture.frame().unwrap();
    println!("{}, {}", frame.width(), frame.height());
    frame.save("frame.jpg").unwrap();

    let image = ImageView::new(ImageInfo::rgb8(640, 480), &frame);

    let crop = Crop::default();
    let labeler = Label::default();

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
                    let frame = detections[0].1.clone();

                    let detection = labeler.detect(&frame);
                    print!("{:?}", detection);
                    let frame = frame.to_rgb8();
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

#[cfg(not(feature = "window"))]
fn main() {}
