@echo off
setlocal

cd /d "%~dp0"

start "" "server.exe"
timeout /t 1 /nobreak >nul
start "" "machine.exe"
start "" "warden.exe"
