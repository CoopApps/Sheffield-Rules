# Saturday at Three - Windows Quick Start

## Step 1: Install Frontend Dependencies

Double-click this file:
```
install-frontend.bat
```

This will download and install all npm packages. Takes 2-5 minutes.

## Step 2: Start the Game

Open **TWO command prompts** (or PowerShell windows) in the `Saturday at Three` folder.

### Window 1 - Backend Server
Double-click:
```
start-backend.bat
```

You should see:
```
Listening on 127.0.0.1:8080
```

**Keep this window open!**

### Window 2 - Frontend Dev Server
Double-click:
```
start-frontend.bat
```

You should see:
```
Local:   http://127.0.0.1:5173/
```

**Keep this window open!**

## Step 3: Open the Game

In your browser, go to:
```
http://127.0.0.1:5173
```

You should see the game UI!

## Troubleshooting

### "npm: command not found"
That's OK! The batch files use the full path. Just double-click the batch files.

### Port 8080 or 5173 already in use
1. Find what's using the port: `netstat -ano | findstr :8080`
2. Kill it: `taskkill /PID {number} /F`
3. Try again

### npm install fails
1. Delete the `frontend/node_modules` folder
2. Delete `frontend/package-lock.json`
3. Try `install-frontend.bat` again

### Frontend shows blank page or errors
1. Make sure backend is running (check Window 1)
2. Open browser DevTools: Press F12
3. Check the Console tab for error messages

## Files in the Root Directory

| File | Purpose |
|------|---------|
| `install-frontend.bat` | Install npm dependencies (run once) |
| `start-backend.bat` | Start backend server |
| `start-frontend.bat` | Start frontend dev server |
| `START_HERE.md` | Overview of the project |
| `QUICKSTART.md` | 5-minute guide |

## Next Steps After Starting

1. **Read:** `START_HERE.md`
2. **Explore:** Click around the game UI
3. **Add Assets:** Put player photos in `assets/packs/historical_1888/player_portraits/`
4. **Restart Backend:** Backend will auto-discover new images

## Keep Both Windows Running

The game won't work if either window closes:
- **Window 1 (Backend):** Serves assets and handles logic
- **Window 2 (Frontend):** Serves the UI

## Stop the Game

In each window, press: **Ctrl+C**

## Questions?

Read the documentation files:
- `START_HERE.md` - Quick intro
- `QUICKSTART.md` - 5-minute overview
- `README.md` - Full documentation
- `ARCHITECTURE.md` - Technical details

Have fun building! ⚽🎮
