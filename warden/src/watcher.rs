use std::path::PathBuf;
use std::sync::mpsc;
use notify::{recommended_watcher, Event, EventKind, Watcher};
use crate::manager;

pub(crate) fn watch(root: PathBuf, download_dir: PathBuf) {
    let (tx, rx) = mpsc::channel();

    let mut warden = recommended_watcher(move |res| {
        tx.send(res).unwrap();
    }).unwrap();

    warden.watch(&download_dir, notify::RecursiveMode::NonRecursive).unwrap();

    loop {
        match rx.recv() {
            Ok(Ok(Event { kind: EventKind::Create(_), paths, .. })) => {
                for path in paths {
                    std::thread::sleep(std::time::Duration::from_millis(500));
                    manager::manage(&path, &root);
                }
            }
            _ => {}
        }
    }
}