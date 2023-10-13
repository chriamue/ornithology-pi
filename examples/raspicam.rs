#[cfg(feature = "webcam")]
use nokhwa::{
    pixel_format::RgbFormat,
    utils::{CameraFormat, CameraIndex, FrameFormat, RequestedFormat, RequestedFormatType},
    Camera,
};

#[cfg(feature = "webcam")]
fn main() {
    let index = CameraIndex::Index(0);
    // request the absolute highest resolution CameraFormat that can be decoded to RGB.
    let requested = RequestedFormat::new::<RgbFormat>(RequestedFormatType::Exact(
        CameraFormat::new_from(1920, 1080, FrameFormat::MJPEG, 30),
    ));

    let mut camera = Camera::new(index, requested).unwrap();
    //println!("{:?}", camera.supported_camera_controls().unwrap());
    // open stream
    let res = camera.open_stream().unwrap();
    println!("{:?}", res);
    // loop range 10
    for _ in 0..10 {
        let frame = camera.frame().unwrap();
        let decoded = frame.decode_image::<RgbFormat>().unwrap();
        println!("{}, {}", decoded.width(), decoded.height());
        decoded.save("frame.jpg").unwrap();
    }
    let frame = camera.frame().unwrap();
    let decoded = frame.decode_image::<RgbFormat>().unwrap();
    println!("{}, {}", decoded.width(), decoded.height());
    decoded.save("frame.jpg").unwrap();
}

#[cfg(not(feature = "webcam"))]
fn main() {
    println!("Webcam feature not enabled");
}
