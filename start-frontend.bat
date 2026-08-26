@echo off
set PATH=C:\Program Files\nodejs;%PATH%

echo.
echo ================================================
echo  SATURDAY AT THREE - Frontend Dev Server
echo ================================================
echo.
echo Starting frontend on http://127.0.0.1:5173
echo.
echo Make sure backend is running on port 8080!
echo.
echo Keep this window open while developing.
echo Press Ctrl+C to stop.
echo.
echo ================================================
echo.

cd frontend
"C:\Program Files\nodejs\npm" run dev

cd ..
