use ornithology_pi::{Capture, WebCam};

fn main() {
    let mut capture = WebCam::default();
    let frame = capture.frame().unwrap();
    println!("{}, {}", frame.width(), frame.height());
    frame.save("frame.jpg").unwrap();
}
