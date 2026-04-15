mod watcher;
mod manager;

use std::{thread, time};
use tungstenite::{connect, Utf8Bytes};

fn close_program() {
    std::process::exit(0);
}

fn main() {

    let root = dirs::home_dir().unwrap().join("my-desktop-agent");
    let download_dir = dirs::download_dir().unwrap();

    let sort_dir = dirs::home_dir().unwrap().join("my-desktop-agent-sort");
    
    {
        std::fs::create_dir_all(&root).unwrap();
        std::fs::create_dir_all(&sort_dir).unwrap();
    }

    thread::spawn(move || {
       let (mut socket, _) = connect("ws://localhost:8080/warden").unwrap();
        loop {
            if socket.send(tungstenite::Message::Text(
                Utf8Bytes::from(r#"{"type":"heartbeat","from":"warden"}"#.to_string())
            )).is_err() {
                close_program();
            };

            thread::sleep(time::Duration::from_secs(5));
        }
    });

    // Start the watcher thread for the sort directory
    {
        let root_clone = root.clone();

        thread::spawn(|| {
            watcher::watch(root_clone, sort_dir);
        });
    }

    // Start the watcher thread for the download directory
    watcher::watch(root, download_dir);
}
