use bluer::{
    rfcomm::{Listener, SocketAddr, Stream},
    Address,
};
use rand::RngCore;
use tokio::io::AsyncWriteExt;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub async fn perform(channel: u8) -> Result<()> {
    let session = bluer::Session::new().await?;
    let adapter = session.default_adapter().await?;
    adapter.set_powered(true).await?;
    adapter.set_discoverable(true).await?;
    adapter.set_discoverable_timeout(0).await?;

    let local_sa = SocketAddr::new(Address::any(), channel);
    let listen = Listener::bind(local_sa).await?;

    let local_sa = listen.as_ref().local_addr()?;
    println!(
        "Advertising on Bluetooth adapter {} with address {}",
        &adapter.name(),
        adapter.address().await?
    );
    println!("Listening on channel {}", local_sa.channel);

    loop {
        match listen.accept().await {
            Ok((mut conn, peer_sa)) => {
                let conn_info = conn.as_ref().conn_info()?;
                println!("Connection from {} with {:?}", peer_sa.addr, &conn_info,);

                loop {
                    let mut rng = rand::thread_rng();
                    let mut buf = vec![0; 4096];
                    rng.fill_bytes(&mut buf);

                    if let Err(err) = conn.write_all(&buf).await {
                        println!("Disconnected: {}", err);
                        break;
                    }
                }
            }
            Err(err) => println!("Connection failed: {}", err),
        }
    }

    Ok(())
}

#[tokio::main]
async fn main() {
    perform(0).await.unwrap();
}
