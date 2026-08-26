@echo off
echo Installing Saturday at Three Frontend Dependencies...
echo.

cd frontend

if exist node_modules (
    echo Removing old installation...
    rmdir /s /q node_modules
)

if exist package-lock.json del package-lock.json

echo Running npm install...
echo.

"C:\Program Files\nodejs\npm" install

if %errorlevel% equ 0 (
    echo.
    echo SUCCESS! Dependencies installed.
    echo.
    echo Next steps:
    echo   1. Open two command prompts
    echo   2. In one, run: start-backend.bat
    echo   3. In the other, run: start-frontend.bat
    echo   4. Open browser to: http://127.0.0.1:5173
    echo.
    pause
) else (
    echo.
    echo FAILED! Check the error above.
    echo.
    pause
)

cd ..
