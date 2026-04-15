use tauri::Emitter;
use tungstenite::{connect, Utf8Bytes};
use std::{thread, time};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
fn close_program() {
    std::process::exit(0);
}

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {

            let handle = app.handle().clone();

            thread::spawn(move || {
                let (mut socket, _) = connect("ws://localhost:8080/window").unwrap();

                let mut last_heartbeat = time::Instant::now();

                let heartbeat_interval = time::Duration::from_secs(5);
                let thread_interval = time::Duration::from_millis(50);

                loop {

                    if last_heartbeat.elapsed() >= heartbeat_interval {
                        if socket.send(tungstenite::Message::Text(
                            Utf8Bytes::from(r#"{"type":"heartbeat","from":"window"}"#.to_string())
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

                            if let Ok(_parsed) = serde_json::from_str::<serde_json::Value>(&raw) {
                                // Emit the message to the frontend
                            }
                        }
                        Err(_) => {}
                    }

                    thread::sleep(thread_interval);
                }
            });

            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
