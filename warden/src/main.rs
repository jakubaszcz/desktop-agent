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

        let mut last_heartbeat = time::Instant::now();

        let heartbeat_interval = time::Duration::from_secs(5);
        let thread_interval = time::Duration::from_millis(50);

        loop {
            if last_heartbeat.elapsed() >= heartbeat_interval {
                if socket.send(tungstenite::Message::Text(
                    Utf8Bytes::from(r#"{"type":"heartbeat","from":"warden"}"#.to_string())
                )).is_err() {
                    close_program();
                };
                last_heartbeat = time::Instant::now();
            }


            match socket.get_mut() {
                tungstenite::stream::MaybeTlsStream::Plain(stream) => {
                    stream.set_nonblocking(true).unwrap();
                }
                _ => {}
            }
            match socket.read() {
                Ok(msg) => {
                    let raw = msg.to_string();

                    if let Ok(parsed) = serde_json::from_str::<serde_json::Value>(&raw) {
                        let msg_type = parsed["type"].as_str().unwrap_or("unknown");
                        let msg_from = parsed["from"].as_str().unwrap_or("unknown");
                        let msg_data = parsed["data"].as_str().unwrap_or("");

                        println!("{} from {}: {}", msg_type, msg_from, msg_data);

                        if msg_type.eq("command") && !msg_from.eq("unknown"){
                            let json_msg = format!(
                                r#"{{"type":"response","action":"{}:{}:{}"}}"#,
                                msg_from,
                                msg_data,
                                "success"
                            );

                            println!("{}", json_msg);

                            if socket
                                .send(tungstenite::Message::Text(Utf8Bytes::from(json_msg)))
                                .is_err()
                            {
                                close_program();
                            }
                        }
                    }
                }
                Err(_) => {}
            }

            thread::sleep(thread_interval);
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
