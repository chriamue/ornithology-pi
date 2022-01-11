use nokhwa::Camera;
use nokhwa::CameraFormat;
use nokhwa::FrameFormat;

fn main() {
    let mut camera = Camera::new(
        0,
        Some(CameraFormat::new_from(640, 480, FrameFormat::MJPEG, 30)), // format
    )
    .unwrap();
    camera.open_stream().unwrap();
    let frame = camera.frame().unwrap();
    println!("{}, {}", frame.width(), frame.height());
    frame.save("frame.jpg").unwrap();
}
