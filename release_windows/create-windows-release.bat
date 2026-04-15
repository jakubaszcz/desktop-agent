@echo off
echo Building all components...

cd ..

:: Server (Go)
echo Building server...
cd server
go build -ldflags="-H windowsgui" -o ../release_windows/server.exe .
cd ..

:: Machine (Rust)
echo Building machine...
cd machine
cargo build --release
copy target\release\machine.exe ..\release_windows\machine.exe
cd ..

:: Warden (Rust)
echo Building warden...
cd warden
cargo build --release
copy target\release\warden.exe ..\release_windows\warden.exe
cd ..

:: Interface (Tauri)
echo Building interface...
cd interface
npm run tauri build
copy src-tauri\target\release\interface.exe ..\release_windows\interface.exe
cd ..

echo Done! Check the release folder.