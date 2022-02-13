use bluer::{
    rfcomm::{Listener, Socket, SocketAddr, Stream},
    Address,
};

use futures::{future, pin_mut, StreamExt};
use rand::RngCore;
use std::collections::VecDeque;
use std::time::{Duration, Instant};
use tokio::io::AsyncReadExt;
use tokio::time::sleep;

type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;

pub async fn perform(address: Address, channel: u8) -> Result<()> {
    let socket = Socket::new()?;
    let local_sa = SocketAddr::any();
    socket.bind(local_sa)?;

    let peer_sa = SocketAddr::new(address, channel);
    let mut conn = socket.connect(peer_sa).await?;

    let conn_info = conn.as_ref().conn_info()?;
    println!("Connected with {:?}", &conn_info);

    let done = async { future::pending().await };
    pin_mut!(done);

    let start = Instant::now();
    let mut total = 0;
    let mut received = VecDeque::new();
    let mut buf = vec![0; 4096];
    loop {
        tokio::select! {
            res = conn.read(&mut buf) => {
                match res? {
                    0 => break,
                    n => {
                        total += n;
                        received.push_back((Instant::now(), n));
                    }
                }
            }
            () = &mut done => break,
        }

        loop {
            match received.front() {
                Some((t, _)) if t.elapsed() > Duration::from_secs(1) => {
                    received.pop_front();
                }
                _ => break,
            }
        }
        let avg_data: usize = received.iter().map(|(_, n)| n).sum();
        if let Some(avg_start) = received.front().map(|(t, _)| t) {
            print!(
                "{:.1} kB/s             \r",
                avg_data as f32 / 1024.0 / avg_start.elapsed().as_secs_f32()
            );
        }
    }
    let dur = start.elapsed();

    println!("                              ");
    println!(
        "Received {} kBytes in {:.1} seconds, speed is {:.1} kB/s",
        total / 1024,
        dur.as_secs_f32(),
        total as f32 / 1024.0 / dur.as_secs_f32()
    );

    Ok(())
}

#[tokio::main(flavor = "current_thread")]
async fn main() {
    let target_address: Address = "B8:27:EB:4C:40:D5".parse().expect("invalid address");
    perform(target_address, 1).await.unwrap();
}
