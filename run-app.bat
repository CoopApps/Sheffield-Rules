@echo off
REM Set Node.js path - adjust if needed
set PATH=C:\Program Files\nodejs;%PATH%

echo.
echo ================================================
echo    SATURDAY AT THREE - Desktop App
echo ================================================
echo.

REM Check if node is available
where node >nul 2>nul
if errorlevel 1 (
    echo ERROR: Node.js not found in PATH
    echo Please install Node.js from https://nodejs.org/
    echo Or edit this file and set the correct path
    pause
    exit /b 1
)

echo Node version:
call node --version
echo NPM version:
call npm --version
echo.

echo Installing root dependencies...
call npm install

echo Installing frontend dependencies...
cd frontend
call npm install
cd ..

echo.
echo Building frontend...
cd frontend
call npm run build
if errorlevel 1 (
    echo ERROR: Frontend build failed
    pause
    exit /b 1
)
cd ..

echo.
echo Starting Tauri app in dev mode...
echo This may take a minute on first run as it compiles Rust...
echo.

call npx tauri dev

if errorlevel 1 (
    echo.
    echo ERROR: Tauri dev failed
    pause
    exit /b 1
)

pause
