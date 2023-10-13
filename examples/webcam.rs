use ornithology_pi::Capture;

#[cfg(feature = "webcam")]
fn main() {
    let mut capture = ornithology_pi::WebCam::default();
    let frame = capture.frame().unwrap();
    println!("{}, {}", frame.width(), frame.height());
    frame.save("frame.jpg").unwrap();
}

#[cfg(not(feature = "webcam"))]
fn main() {
    println!("Webcam feature not enabled");
}
