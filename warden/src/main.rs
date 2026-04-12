use std::{thread, time};
use std::path::PathBuf;
use std::sync::mpsc;
use notify::{recommended_watcher, Event, EventKind, Watcher};
use tungstenite::{connect, Utf8Bytes};

fn watcher(root: PathBuf, download_dir: PathBuf) {
    let (tx, rx) = mpsc::channel();

    let mut warden = recommended_watcher(move |res| {
        tx.send(res).unwrap();
    }).unwrap();

    warden.watch(&download_dir, notify::RecursiveMode::NonRecursive).unwrap();

    loop {
        match rx.recv() {
            Ok(Ok(Event { kind: EventKind::Create(_), paths, .. })) => {
                println!("{:?}", paths);
            }
            _ => {}
        }
    }
}

fn main() {

    let root = dirs::home_dir().unwrap().join("my-desktop-agent");
    let download_dir = dirs::download_dir().unwrap();

    {
        std::fs::create_dir_all(&root).unwrap();
    }

    thread::spawn(move || {
       let (mut socket, _) = connect("ws://localhost:8080/warden").expect("Failed to connect to warden");

        loop {
            socket.send(tungstenite::Message::Text(
                Utf8Bytes::from(r#"{"type":"heartbeat","from":"warden"}"#.to_string())
            )).unwrap();

            thread::sleep(time::Duration::from_secs(5));
        }
    });

    watcher(root, download_dir);
}
