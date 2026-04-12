use std::{thread, time};
use tungstenite::{connect, Utf8Bytes};

fn main() {
    thread::spawn(move || {
       let (mut socket, _) = connect("ws://localhost:8080/warden").expect("Failed to connect to warden");

        loop {
            socket.send(tungstenite::Message::Text(
                Utf8Bytes::from(r#"{"type":"heartbeat", "from":"warden"}"#.to_string())
            )).unwrap();

            thread::sleep(time::Duration::from_secs(5));
        }
    });

    loop {}
}
