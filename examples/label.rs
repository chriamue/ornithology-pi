use lenna_birds_plugin::Birds;

fn main() {
    let birds = Birds::default();
    let img = Box::new(
        lenna_core::io::read::read_from_file("assets/Green_Kingfisher_0020_71155.jpg".into())
            .unwrap(),
    );

    let label = birds.detect_label(&img.image).unwrap();
    println!("{:?}", label);
}
