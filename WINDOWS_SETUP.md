# Windows Setup Guide - Saturday at Three

## Prerequisites

You need:
- ✅ Rust 1.70+ (already have it)
- ✅ Node.js 18+ (already have it)
- ✅ npm (comes with Node.js)

## Quick Setup (2 minutes)

### Option A: Automated Setup (Easiest)

Just run one file:

```
D:\projects\Saturday at Three\SETUP.bat
```

This will:
1. Verify you have Rust and Node.js
2. Show you a menu of options
3. Install npm dependencies
4. Get you running

### Option B: Manual Setup

**Terminal 1 - Backend:**
```cmd
cd D:\projects\Saturday at Three\backend
cargo run
```

**Terminal 2 - Frontend:**
```cmd
cd D:\projects\Saturday at Three\frontend
npm install
npm run dev
```

**Browser:**
```
http://127.0.0.1:5173
```

## Batch Files Provided

The project now includes helpful batch files:

| File | Location | Does What |
|------|----------|-----------|
| `SETUP.bat` | Root | Main setup menu |
| `setup.bat` | frontend/ | Installs npm dependencies |
| `start.bat` | backend/ | Runs backend server |
| `start.bat` | frontend/ | Runs dev server |

## If npm Still Doesn't Work

If you get `'npm' is not recognized...` error:

### Solution 1: Use Full Path
```cmd
"C:\Program Files\nodejs\npm" install
```

### Solution 2: Add to PATH (Permanent)

1. Press **Win + R**
2. Type: `sysdm.cpl`
3. Click "Environment Variables" button
4. Under "User variables", click "New..."
5. Variable name: `PATH`
6. Variable value: `C:\Program Files\nodejs`
7. Click OK, OK, OK
8. Close all command prompts and reopen

Then `npm` will work globally.

### Solution 3: Use PowerShell

Open PowerShell instead of Command Prompt:
```powershell
cd "D:\projects\Saturday at Three\frontend"
npm install
npm run dev
```

## Step-by-Step Manual Setup

### 1. Backend Setup

```cmd
cd "D:\projects\Saturday at Three\backend"
cargo build
```

This will compile the Rust backend. First time takes ~60 seconds.

Verify it worked:
```cmd
cargo run
```

You should see:
```
Starting Saturday at Three server on http://127.0.0.1:8080
```

Stop with Ctrl+C.

### 2. Frontend Setup

```cmd
cd "D:\projects\Saturday at Three\frontend"
"C:\Program Files\nodejs\npm" install
```

Or if you added Node.js to PATH:
```cmd
npm install
```

This downloads React, TypeScript, Vite, and other dependencies.

Verify it worked:
```cmd
npm run dev
```

You should see:
```
  VITE v5.0.0  ready in 234 ms

  ➜  Local:   http://127.0.0.1:5173/
  ➜  press h to show help
```

## Running the Game

**You need TWO terminals open simultaneously:**

### Terminal 1: Backend Server
```cmd
cd "D:\projects\Saturday at Three\backend"
cargo run
```

Wait for: `Listening on 127.0.0.1:8080`

### Terminal 2: Frontend Dev Server
```cmd
cd "D:\projects\Saturday at Three\frontend"
npm run dev
```

Wait for: `Local: http://127.0.0.1:5173`

### Terminal 3 (or Browser)
Open: **http://127.0.0.1:5173**

You should see the game UI with:
- Sidebar navigation
- Dashboard screen
- Theme selector

## Common Issues & Solutions

### Issue: npm not recognized

**Solution A:** Use full path
```cmd
"C:\Program Files\nodejs\npm" install
```

**Solution B:** Use PowerShell
```powershell
npm install
```

**Solution C:** Add to PATH (see above)

### Issue: Backend port 8080 already in use

**Solution:** Kill the process using port 8080
```cmd
netstat -ano | findstr :8080
taskkill /PID {PID_NUMBER} /F
```

Then try again.

### Issue: npm install fails with permission errors

**Solution:** Clean and retry
```cmd
cd "D:\projects\Saturday at Three\frontend"
rmdir /s /q node_modules
del package-lock.json
npm install
```

### Issue: Frontend shows "connection refused" or blank page

**Solution:**
1. Make sure backend is running on :8080
2. Check browser console (F12) for errors
3. Try: `curl http://127.0.0.1:8080/health`
4. Restart both servers

### Issue: Cargo build fails

**Solution:**
```cmd
cd backend
cargo clean
cargo build
```

## File Locations

```
D:\projects\Saturday at Three\
├── backend/           ← Backend source code (Rust)
├── frontend/          ← Frontend source code (React)
├── assets/            ← Game images (add here)
├── SETUP.bat          ← Run this first!
├── START_HERE.md      ← Read this next
└── *.md              ← Documentation
```

## Ports

- **Backend:** http://127.0.0.1:8080
- **Frontend:** http://127.0.0.1:5173

If these ports are in use, you'll need to change them:

**Backend:** Edit `backend/src/main.rs` line with `.bind("127.0.0.1:8080")`
**Frontend:** Edit `frontend/vite.config.ts` and change port

## Next Steps After Setup

1. Open `START_HERE.md` to understand what you have
2. Explore the game UI (click around)
3. Read `QUICKSTART.md` for next tasks
4. Add player photos to `assets/packs/historical_1888/player_portraits/`

## Building for Release

### Backend
```cmd
cd backend
cargo build --release
```
Output: `backend\target\release\saturday_at_three.exe`

### Frontend
```cmd
cd frontend
npm run build
```
Output: `frontend\dist\` (ready to deploy)

## Getting Help

- Check browser console: **F12** → Console tab
- Look at `*.md` files in project root for documentation
- All code is type-safe (Rust + TypeScript), so many errors are caught at compile time

## Tips

- Keep both servers running (backend + frontend)
- React DevTools browser extension is helpful
- VS Code is great for editing this project
- Git is configured with `.gitignore`

## You're Ready!

If you got this far, everything is set up. Just:

1. Open two terminals
2. Run `cargo run` in backend/
3. Run `npm run dev` in frontend/
4. Open browser to http://127.0.0.1:5173

Enjoy building! ⚽🎮
