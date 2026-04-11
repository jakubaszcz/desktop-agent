use std::{thread, time};
use tungstenite::connect;

fn main() {
    let (mut socket, _) = connect("ws://localhost:8080/heartbeat")
        .expect("Failed to connect");

    loop {
        socket.send(tungstenite::Message::text("heartbeat")).unwrap();
        let msg = socket.read().unwrap();
        println!("Go dit: {}", msg);
        thread::sleep(time::Duration::from_secs(5));
    }
}
