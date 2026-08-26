@echo off
REM Add Node.js to PATH
set PATH=C:\Program Files\nodejs;%PATH%

echo Starting Saturday at Three Frontend Development Server...
echo.
echo Available at: http://127.0.0.1:5173
echo.
echo Make sure the backend is running on port 8080!
echo.

call npm run dev
