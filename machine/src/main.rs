use std::{thread, time};
use std::sync::mpsc;
use tungstenite::{connect, Utf8Bytes};

fn main() {
    let (tx, rx) = mpsc::channel();

    thread::spawn(move || {
        let (mut socket, _) = connect("ws://localhost:8080/machine")
            .expect("Failed to connect");

        loop {
            socket.send(tungstenite::Message::Text(
                Utf8Bytes::from(r#"{"type":"heartbeat"}"#.to_string())
            )).unwrap();

            if let Ok(keybind) = rx.try_recv() {
                socket.send(tungstenite::Message::Text(keybind)).unwrap();
            }

            thread::sleep(time::Duration::from_secs(5));
        }
    });

    loop {
        // tx.send("message").unwrap();
    }
}
