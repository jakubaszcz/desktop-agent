use std::{thread, time};
use std::io::Read;
use std::path::{Path, PathBuf};
use std::sync::mpsc;
use notify::{recommended_watcher, Event, EventKind, Watcher};
use tungstenite::{connect, Utf8Bytes};

enum FileType {
    Image,
    Video,
    Audio,
    Document,
    Archive,
    Code,
    Unknown,
}

fn detect_type(path: &Path) -> FileType {
    let mut file = match std::fs::File::open(path) {
        Ok(file) => file,
        Err(_) => return FileType::Unknown,
    };

    let mut header = [0u8; 8];
    if file.read(&mut header).is_err() {
        return FileType::Unknown
    }

    match header {
        // Images
        [0x89, 0x50, 0x4E, 0x47, ..] => FileType::Image, // PNG
        [0xFF, 0xD8, 0xFF, ..]        => FileType::Image, // JPEG
        [0x47, 0x49, 0x46, ..]        => FileType::Image, // GIF
        [0x42, 0x4D, ..]              => FileType::Image, // BMP

        // Videos
        [0x66, 0x74, 0x79, 0x70, ..] => FileType::Video, // MP4
        [0x1A, 0x45, 0xDF, 0xA3, ..] => FileType::Video, // MKV

        // Audio
        [0x49, 0x44, 0x33, ..]       => FileType::Audio, // MP3
        [0x66, 0x4C, 0x61, 0x43, ..] => FileType::Audio, // FLAC

        // Documents
        [0x25, 0x50, 0x44, 0x46, ..] => FileType::Document, // PDF
        [0x50, 0x4B, 0x03, 0x04, ..] => FileType::Document, // DOCX/XLSX (ZIP based)

        // Archives
        [0x52, 0x61, 0x72, 0x21, ..] => FileType::Archive, // RAR
        [0x37, 0x7A, 0xBC, 0xAF, ..] => FileType::Archive, // 7Z

        _ => FileType::Unknown,
    }
}

fn get_destination(file_type: &FileType, root: &Path) -> Option<PathBuf> {
    match file_type {
        FileType::Image    => Some(root.join("Pictures")),
        FileType::Video    => Some(root.join("Videos")),
        FileType::Audio    => Some(root.join("Music")),
        FileType::Document => Some(root.join("Documents")),
        FileType::Archive  => Some(root.join("Archives")),
        FileType::Code     => Some(root.join("Code")),
        FileType::Unknown  => None,
    }
}

fn is_temp_file(path: &Path) -> bool {
    match path.extension().and_then(|e| e.to_str()) {
        Some("tmp") | Some("part") | Some("crdownload") => true,
        _ => false,
    }
}

fn move_file(path: &Path, root: &Path) {

    if is_temp_file(path) {
        return;
    }

    let file_type = detect_type(path);

    if let Some(destination) = get_destination(&file_type, root) {
        std::fs::create_dir_all(&destination).unwrap();

        let filename = path.file_name().unwrap();
        let dest_path = destination.join(filename);

        match std::fs::rename(path, &dest_path) {
            Ok(_)  => println!("Déplacé: {:?} → {:?}", path, dest_path),
            Err(e) => println!("Erreur: {}", e),
        }
    }
}

fn watcher(root: PathBuf, download_dir: PathBuf) {
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
                    move_file(&path, &root);
                }
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
