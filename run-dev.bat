@echo off
REM Add Node.js to PATH
set PATH=C:\Program Files\nodejs;%PATH%

REM Navigate to project directory
cd /d "D:\projects\Saturday at Three"

REM Clean previous debug build to free memory
cargo clean --release 2>nul

REM Run Tauri dev with release mode for backend
"C:\Program Files\nodejs\npx" tauri dev --release
