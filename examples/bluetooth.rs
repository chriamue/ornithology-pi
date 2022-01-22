use ornithology_pi::Bluetooth;

#[tokio::main]
async fn main() {
    let mut bluetooth = Bluetooth::default();
    bluetooth.run().await.unwrap();
}
