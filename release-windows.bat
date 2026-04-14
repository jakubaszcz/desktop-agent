@echo off
echo Building all components...

:: Create output folder
mkdir release 2>nul

:: Server (Go)
echo Building server...
cd server
go build -o ../release/server.exe .
cd ..

:: Machine (Rust)
echo Building machine...
cd machine
cargo build --release
copy target\release\machine.exe ..\release\machine.exe
cd ..

:: Warden (Rust)
echo Building warden...
cd warden
cargo build --release
copy target\release\warden.exe ..\release\warden.exe
cd ..

:: Interface (Tauri)
echo Building interface...
cd interface
npm run tauri build
copy src-tauri\target\release\interface.exe ..\release\interface.exe
cd ..

echo Done! Check the release folder.