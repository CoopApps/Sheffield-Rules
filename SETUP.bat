@echo off
cls
echo.
echo ================================================
echo    SATURDAY AT THREE - Setup
echo ================================================
echo.

REM Add Node.js to PATH
set PATH=C:\Program Files\nodejs;%PATH%

echo Checking prerequisites...
echo.

REM Check Rust
cargo --version >nul 2>&1
if %errorlevel% neq 0 (
    echo ERROR: Rust not found. Please install from https://rustup.rs/
    pause
    exit /b 1
)
echo [OK] Rust installed

REM Check Node.js
node --version >nul 2>&1
if %errorlevel% neq 0 (
    echo ERROR: Node.js not found. Please install from https://nodejs.org/
    pause
    exit /b 1
)
echo [OK] Node.js installed

REM Check npm
npm --version >nul 2>&1
if %errorlevel% neq 0 (
    echo ERROR: npm not found
    pause
    exit /b 1
)
echo [OK] npm installed

echo.
echo All prerequisites found!
echo.
echo ================================================
echo Setup options:
echo ================================================
echo.
echo   1. Install frontend dependencies (npm install)
echo   2. Start backend server (cargo run)
echo   3. Start frontend dev server (npm run dev)
echo   4. Open documentation
echo.
echo ================================================
echo.

set /p choice="Enter your choice (1-4): "

if "%choice%"=="1" (
    cls
    echo Installing frontend dependencies...
    cd frontend
    if exist node_modules (
        echo Cleaning old installation...
        rmdir /s /q node_modules
    )
    if exist package-lock.json del package-lock.json
    call npm install
    if %errorlevel% equ 0 (
        echo.
        echo Installation complete!
        echo.
        echo Next, run: npm run dev
        echo Then open: http://127.0.0.1:5173
        echo.
    )
    cd ..
) else if "%choice%"=="2" (
    cls
    cd backend
    echo Starting backend server...
    echo Available at: http://127.0.0.1:8080
    echo.
    call cargo run
    cd ..
) else if "%choice%"=="3" (
    cls
    cd frontend
    echo Starting frontend dev server...
    echo Available at: http://127.0.0.1:5173
    echo.
    call npm run dev
    cd ..
) else if "%choice%"=="4" (
    start START_HERE.md
) else (
    echo Invalid choice. Please run again and enter 1, 2, 3, or 4.
    pause
)
