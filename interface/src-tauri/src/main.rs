// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use std::{thread, time};
use tungstenite::{connect, Utf8Bytes};

fn main() {

    thread::spawn(|| {
        let (mut socket, _) = connect("ws://localhost:8080/window").unwrap();

        loop {
            socket.send(tungstenite::Message::Text(
                Utf8Bytes::from(r#"{"type":"heartbeat","from":"window"}"#.to_string())
            )).unwrap();

            thread::sleep(time::Duration::from_secs(5));
        }
    });

    interface_lib::run()
}
