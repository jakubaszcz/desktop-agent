use std::{thread, time};
use std::sync::mpsc;
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

impl App {
    fn new(tx: mpsc::Sender<String>) -> Self {

        // Hotkey
        let manager = GlobalHotKeyManager::new().unwrap();

        let custom_copy_hotkey = HotKey::new(
            Some(Modifiers::CONTROL | Modifiers::ALT),
            Code::KeyA,
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
        let (mut socket, _) = connect("ws://localhost:8080/machine")
            .expect("Failed to connect");

        loop {
            socket.send(tungstenite::Message::Text(
                Utf8Bytes::from(r#"{"type":"heartbeat"}"#.to_string())
            )).unwrap();

            if let Ok(keybind) = rx.try_recv() {
                socket.send(tungstenite::Message::Text(Utf8Bytes::from(keybind))).unwrap();
            }

            thread::sleep(time::Duration::from_secs(5));
        }
    });
    event_loop.run_app(app).unwrap();
}
