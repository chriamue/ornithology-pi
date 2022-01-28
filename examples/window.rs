#[cfg(feature = "window")]
use image::imageops;
#[cfg(feature = "window")]
use ornithology_pi::{
    observer::{Observable, Observer},
    Capture, DataSighting,
};
use show_image::WindowProxy;
#[cfg(feature = "window")]
use show_image::{create_window, event, ImageInfo, ImageView};

struct BirdObserver {
    window: WindowProxy,
}

unsafe impl Send for BirdObserver {}
unsafe impl Sync for BirdObserver {}

impl Observer for BirdObserver {
    fn notify(&self, sighting: DataSighting) {
        println!("{:?}", sighting.0.species);
        let frame = sighting.1.to_rgb8();
        let frame = imageops::resize(&frame, 640, 480, imageops::FilterType::Triangle);
        let image = ImageView::new(ImageInfo::rgb8(640, 480), &frame);
        self.window.set_image("bird", image).unwrap();
    }
}

#[cfg(feature = "window")]
#[show_image::main]
fn main() -> Result<(), Box<dyn std::error::Error>> {
    use ornithology_pi::{detector::Detector, BirdDetector};

    let mut birddetector = BirdDetector::default();

    let window = create_window("image", Default::default())?;
    let detector_window = create_window("detection", Default::default()).unwrap();

    birddetector.register(Box::new(BirdObserver {
        window: detector_window,
    }));

    let frame = birddetector.capture.lock().unwrap().frame().unwrap();
    println!("{}, {}", frame.width(), frame.height());
    frame.save("frame.jpg").unwrap();

    let image = ImageView::new(ImageInfo::rgb8(640, 480), &frame);

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
                let frame = birddetector.capture.lock().unwrap().frame().unwrap();
                let frame = imageops::resize(&frame, 640, 480, imageops::FilterType::Triangle);
                let image = ImageView::new(ImageInfo::rgb8(640, 480), &frame);
                window.set_image("image-001", image)?;
                birddetector.detect_next();
            }
        }
    }

    Ok(())
}

#[cfg(not(feature = "window"))]
fn main() {}
