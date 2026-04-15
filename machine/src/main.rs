use std::{thread, time};
use std::sync::mpsc;
use std::time::{Duration, Instant};
use global_hotkey::{GlobalHotKeyEvent, GlobalHotKeyManager, HotKeyState};
use global_hotkey::hotkey::{Code, HotKey, Modifiers};
use tungstenite::{connect, Utf8Bytes};
use winit::{
    application::ApplicationHandler,
    event_loop::{ActiveEventLoop, ControlFlow, EventLoop},
};
use winit::event::WindowEvent;
use winit::window::WindowId;

struct App {
    manager: GlobalHotKeyManager,
    id: u32,
    tx: mpsc::Sender<String>
}

fn close_program() {
    std::process::exit(0);
}

impl App {
    fn new(tx: mpsc::Sender<String>) -> Self {

        // Hotkey
        let manager = GlobalHotKeyManager::new().unwrap();

        let custom_copy_hotkey = HotKey::new(
            Some(Modifiers::ALT),
            Code::KeyW,
        );

        let id = custom_copy_hotkey.id;

        manager.register(custom_copy_hotkey).unwrap();

        Self {
            manager,
            id,
            tx
        }
    }
}

impl ApplicationHandler for App {
    fn resumed(&mut self, event_loop: &dyn ActiveEventLoop) {}

    fn can_create_surfaces(&mut self, event_loop: &dyn ActiveEventLoop) {}

    fn window_event(&mut self, event_loop: &dyn ActiveEventLoop, window_id: WindowId, event: WindowEvent) {}

    fn about_to_wait(&mut self, event_loop: &dyn ActiveEventLoop) {
        event_loop.set_control_flow(ControlFlow::Poll);

        let receiver = GlobalHotKeyEvent::receiver();

        while let Ok(event) = receiver.try_recv() {
            if event.state != HotKeyState::Released {
                continue;
            }

            if event.id == self.id {
                self.tx.send(format!(r#"{{"type":"keybind", "action":"interface"}}"#)).unwrap();
            }
        }
    }
}

fn main() {
    let (tx, rx) = mpsc::channel();
    let event_loop = EventLoop::new().unwrap();
    let app = App::new(tx);


    thread::spawn(move || {
        let (mut socket, _) = connect("ws://localhost:8080/machine").unwrap();

        let mut last_heartbeat = Instant::now();

        let heartbeat_interval = Duration::from_secs(5);
        let thread_interval = Duration::from_millis(50);

        loop {

            if let Ok(keybind) = rx.try_recv() {
                if socket.send(tungstenite::Message::Text(Utf8Bytes::from(keybind))).is_err() { close_program(); };
            }

            if last_heartbeat.elapsed() >= heartbeat_interval {
                if socket.send(tungstenite::Message::Text(
                    Utf8Bytes::from(r#"{"type":"heartbeat","from":"machine"}"#.to_string())
                )).is_err() { close_program(); }
                last_heartbeat = Instant::now();
            }

            thread::sleep(thread_interval);
        }
    });
    event_loop.run_app(app).unwrap();
}
