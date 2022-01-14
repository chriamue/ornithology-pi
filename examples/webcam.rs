use ornithology_pi::Capture;

fn main() {
    let mut capture = Capture::default();
    let frame = capture.frame().unwrap();
    println!("{}, {}", frame.width(), frame.height());
    frame.save("frame.jpg").unwrap();
}
