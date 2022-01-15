use ornithology_pi::Bluetooth;

#[tokio::main]
async fn main() {
    let bluetooth = Bluetooth::default();
    bluetooth.run().await;
}
