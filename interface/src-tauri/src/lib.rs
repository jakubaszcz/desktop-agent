use tauri::{Emitter, Manager};
use tungstenite::{connect, Utf8Bytes};
use std::{thread, time};
use sysinfo::{System};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
fn close_program() {
    std::process::exit(0);
}

pub fn run() {
    tauri::Builder::default()
        .setup(|app| {

            let window = app.get_window("main").unwrap();
            let handle = app.handle();

            // Window settings
            {
                // Focus on opening
                window.set_focus().unwrap();

                // Hide the window title bar
                window.set_decorations(false).unwrap();

                // Handle window size
                {
                    window.set_resizable(false).unwrap();

                    window.set_size(tauri::Size::Logical(tauri::LogicalSize::new(1280.0, 720.0))).unwrap();
                }

                // Window position
                {
                    let window_size = window.outer_size().unwrap();

                    let current_screen = window.current_monitor().unwrap().unwrap();
                    let screen_size = current_screen.size();

                    let position_x = (screen_size.width - window_size.width) / 2;
                    let position_y = (screen_size.height - window_size.height) / 2;

                    window.set_position(tauri::Position::Physical(tauri::PhysicalPosition {
                        x: position_x as i32,
                        y: position_y as i32,
                    })).unwrap();
                }
            }

            let handle_clone = handle.clone();
            thread::spawn(move || {
                let mut sys = System::new_all();

                loop {
                    sys.refresh_all ();
                    handle_clone.emit("memory-used", sys.used_memory()).unwrap();
                    handle_clone.emit("total-memory", sys.total_memory()).unwrap();
                    thread::sleep(time::Duration::from_secs(1));
                }
            });

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
