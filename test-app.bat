@echo off
setlocal enabledelayedexpansion

REM Set Node.js path
set "PATH=C:\Program Files\nodejs;!PATH!"

REM Change to project directory
cd /d "D:\projects\Saturday at Three"

echo ===================================
echo Saturday at Three - Test Launch
echo ===================================
echo.

REM First initialize database if needed
echo Checking database...
if exist "saturday_at_three.db" (
    echo Database found.
) else (
    echo Initializing database...
    call src-tauri\target\release\init_database.exe
)

echo.
echo Starting application with: npm run tauri dev
echo.
echo The app will open in your default browser at:
echo http://localhost:5173/
echo.
echo Press Ctrl+C in the terminal to stop the dev server.
echo.

npm run tauri dev

pause
