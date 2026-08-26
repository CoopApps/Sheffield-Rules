@echo off
REM Add Node.js to PATH
set PATH=C:\Program Files\nodejs;%PATH%

REM Clean old installation
if exist node_modules rmdir /s /q node_modules
if exist package-lock.json del package-lock.json

REM Install dependencies
echo Installing npm dependencies...
call npm install

if %errorlevel% equ 0 (
    echo.
    echo ✓ Installation complete!
    echo.
    echo To start the frontend development server, run:
    echo   npm run dev
    echo.
) else (
    echo.
    echo ✗ Installation failed. Check the error above.
    echo.
)
