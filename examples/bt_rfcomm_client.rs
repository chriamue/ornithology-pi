use bluer::{
    rfcomm::{Socket, SocketAddr},
    Address,
};

use ornithology_pi::bluetooth::{
    rfcomm_srv::{CHANNEL, MTU},
    Message,
};
use std::time::{Duration};
use tokio::io::{AsyncReadExt, AsyncWriteExt};
use tokio::time::sleep;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub async fn perform(address: Address, channel: u8) -> Result<()> {
    let socket = Socket::new()?;
    let local_sa = SocketAddr::any();
    socket.bind(local_sa)?;

    let peer_sa = SocketAddr::new(address, channel);
    let stream = socket.connect(peer_sa).await?;

    let conn_info = stream.as_ref().conn_info()?;
    println!("Connected with {:?}", &conn_info);

    let mut buf = vec![0; MTU.into()];
    let (mut rh, mut wh) = stream.into_split();

    let request = serde_json::to_vec(&Message::LastRequest).unwrap();

    if let Err(err) = wh.write_all(&request).await {
        println!("Write failed: {}", &err);
    }

    loop {
        let n = match rh.read(&mut buf).await {
            Ok(0) => {
                println!("Stream ended");
                break;
            }
            Ok(n) => n,
            Err(err) => {
                println!("Read failed: {}", &err);
                continue;
            }
        };
        let buf = &buf[..n];

        let message = serde_json::from_slice::<Message>(buf);
        match message {
            Ok(Message::Ping) => {
                println!("{:?}", Message::Ping);
            }
            Ok(Message::Pong) => {
                println!("{:?}", Message::Pong);

                sleep(Duration::from_secs(1)).await;
                let request = serde_json::to_vec(&Message::Ping).unwrap();

                if let Err(err) = wh.write_all(&request).await {
                    println!("Write failed: {}", &err);
                    continue;
                }
            }
            Ok(Message::CountResponse { count }) => {
                println!("Counted {}", count);
                let request = serde_json::to_vec(&Message::LastRequest).unwrap();

                if let Err(err) = wh.write_all(&request).await {
                    println!("Write failed: {}", &err);
                }
            }
            Ok(Message::LastResponse { last }) => {
                println!("{:?}", last);
            }
            _ => {
                println!("Read {:?} bytes", buf);
            }
        }
    }

    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let target_address: Address = "00:24:D6:A1:9A:BD".parse().expect("invalid address");
    let target_address: Address = "B8:27:EB:4C:40:D5".parse().expect("invalid address");
    let target_address: Address = "00:15:83:0C:BF:EB".parse().expect("invalid address");

    perform(target_address, CHANNEL).await.unwrap();
}
