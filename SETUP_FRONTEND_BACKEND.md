# Complete Setup Guide - Frontend + Backend

## 🎯 Goal
Run the full Saturday at Three game with both frontend (React) and backend (Rust) to see the new initialization logging.

## ✅ What You Have
- **Node.js:** v25.8.0 installed at `d:\nodejs`
- **NPM:** v11.11.0
- **Rust/Cargo:** Already working
- **Project:** D:\projects\Saturday at Three

## 🚨 Issue
Node.js is installed but not in your system PATH, so Windows can't find `npm` command.

## 🔧 Solution: Two Options

### Option A: Add to PATH Permanently (Recommended)

**Windows 11/10:**
1. Press `Win + X`, select "System"
2. Click "Advanced system settings"
3. Click "Environment Variables"
4. Under "System variables", find "Path"
5. Click "Edit"
6. Click "New"
7. Add: `d:\nodejs`
8. Click "OK" on all dialogs
9. **Restart your terminal/PowerShell**

**Result:** `npm` command will work everywhere permanently.

### Option B: Add to PATH Temporarily (Quick Test)

**PowerShell:**
```powershell
$env:PATH = "d:\nodejs;$env:PATH"
```

**Command Prompt:**
```cmd
set PATH=d:\nodejs;%PATH%
```

**Result:** `npm` works only in current terminal session.

---

## 📋 Step-by-Step: Run the Game

### Step 1: Open Terminal
- PowerShell (recommended)
- Or Command Prompt

### Step 2: Add Node.js to PATH
```powershell
$env:PATH = "d:\nodejs;$env:PATH"
```

### Step 3: Verify Setup
```bash
node --version
# Expected: v25.8.0

npm --version
# Expected: 11.11.0

cargo --version
# Should show Rust cargo version
```

### Step 4: Navigate to Frontend
```bash
cd "D:\projects\Saturday at Three\frontend"
```

### Step 5: Install Dependencies (First Time Only)
```bash
npm install
```

**What this does:**
- Downloads all React packages
- Downloads Vite (dev server)
- Downloads Tauri packages
- Takes 2-5 minutes
- Creates `node_modules/` folder

**You'll see output like:**
```
npm install
added 234 packages in 2m

12 packages are looking for funding
  run `npm fund` for details
```

### Step 6: Run the Game!
```bash
npm run tauri:dev
```

**What happens:**
1. ⚡ Vite starts the React dev server (port 5173)
2. 🦀 Cargo builds the Rust backend
3. 🪟 Game window opens
4. 📋 **Console shows all initialization logging!**

---

## 🎮 What You'll See

### In the Terminal:
```
╔══════════════════════════════════════════════════════════════════════════╗
║  INITIALIZING SHEFFIELD & HALLAMSHIRE FANTASY LEAGUE - 1867             ║
╚══════════════════════════════════════════════════════════════════════════╝

⚙️  Mode: sheffield-hallamshire-league
📅 Year: 1867
🏟️  Club: sheffield-fc-1857

━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
📋 STEP 1/7: Copying Master Database
━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━━
   Source: Sheffield1867.db
   Target: Sheffield1867_temp.db
   Purpose: Preserve master data during gameplay

   ✅ Database copied successfully

[... continues through all 7 steps ...]

🏆 Loading Cup Competitions
   ✅ Found 2 cup competition(s):
      • Youdan Cup (Divisions 1-3)
      • Cromwell Cup (Divisions 4-7)

╔══════════════════════════════════════════════════════════════════════════╗
║  ✅ INITIALIZATION COMPLETE                                              ║
╚══════════════════════════════════════════════════════════════════════════╝

🎮 Game ready to start!
```

### In the Game Window:
- Club selection screen
- Select Sheffield FC (or any club)
- Click "Initialize Game"
- **Watch the console** to see all the logging!

---

## 🛠️ Troubleshooting

### Problem: "npm: command not found"
**Solution:** Node.js not in PATH. Run:
```powershell
$env:PATH = "d:\nodejs;$env:PATH"
```

### Problem: "Failed to copy database"
**Cause:** Sheffield1867.db is locked (game already running)
**Solution:**
1. Close any running game windows
2. Delete `Sheffield1867_temp.db` if it exists
3. Try again

### Problem: "Cargo build failed"
**Cause:** Rust backend compilation error
**Solution:**
```bash
cd "D:\projects\Saturday at Three\src-tauri"
cargo clean
cargo build
```

### Problem: Frontend won't start
**Solution:**
```bash
cd frontend
rm -rf node_modules
npm install
npm run tauri:dev
```

### Problem: Port 5173 already in use
**Cause:** Another dev server running
**Solution:**
1. Find and close other Vite/React dev servers
2. Or kill the process:
```bash
# PowerShell
Get-Process -Name node | Stop-Process -Force
```

---

## 📁 Project Structure

```
D:\projects\Saturday at Three\
├── frontend/                  ← React UI (NPM)
│   ├── src/
│   │   ├── screens/          ← Game screens
│   │   ├── components/       ← UI components
│   │   └── styles/           ← CSS files
│   ├── package.json          ← NPM dependencies
│   └── node_modules/         ← Downloaded packages
│
├── src-tauri/                ← Rust backend (Cargo)
│   ├── src/
│   │   ├── commands.rs       ← Game commands (LOGGING HERE!)
│   │   └── database/         ← Database logic
│   ├── Cargo.toml           ← Rust dependencies
│   └── target/              ← Compiled binaries
│
├── dbviewer/                 ← Database tools (Cargo)
│   ├── compare_schema.rs
│   ├── add_cup_competitions.rs
│   └── redistribute_all_clubs.rs
│
└── Sheffield1867.db          ← Master database
```

---

## 🎯 Quick Commands Reference

### Frontend (NPM)
```bash
cd frontend

npm install              # Install dependencies (first time)
npm run dev              # Frontend only (browser)
npm run tauri:dev        # Full app (frontend + backend)
npm run build            # Build for production
```

### Backend (Cargo)
```bash
cd src-tauri

cargo build              # Build backend
cargo build --release    # Optimized build
cargo clean              # Clean build artifacts
```

### Database Tools (Cargo)
```bash
cd dbviewer

cargo run --bin compare_schema              # Compare DB with schema
cargo run --bin add_cup_competitions        # Add Youdan + Cromwell cups
cargo run --bin redistribute_all_clubs      # Redistribute 372 clubs
cargo run --bin list_sheffield_tables       # List all tables
```

---

## 🔍 Checking Your Setup

Run this script to verify everything is ready:

```bash
# Check Node.js
node --version

# Check NPM
npm --version

# Check Cargo
cargo --version

# Check if frontend dependencies are installed
cd "D:\projects\Saturday at Three\frontend"
ls node_modules         # Should show many folders

# Check if Rust backend compiles
cd "D:\projects\Saturday at Three\src-tauri"
cargo check
```

**Expected output:**
```
✅ Node.js: v25.8.0
✅ NPM: v11.11.0
✅ Cargo: 1.xx.x
✅ node_modules exists with 200+ packages
✅ Cargo check: Finished without errors
```

---

## 🚀 First-Time Setup Checklist

- [ ] Add `d:\nodejs` to PATH (permanently or temporarily)
- [ ] Open terminal and verify `npm --version` works
- [ ] Navigate to `frontend/` folder
- [ ] Run `npm install` (takes 2-5 minutes)
- [ ] Run `npm run tauri:dev`
- [ ] Game window opens
- [ ] Select a club
- [ ] Click "Initialize Game"
- [ ] **Watch console for initialization logging!**

---

## 💡 Tips

1. **Keep the terminal visible** when running the game to see all logging
2. **First compilation is slow** (Rust backend takes 2-3 minutes first time)
3. **Subsequent runs are faster** (incremental compilation)
4. **Hot reload works** - frontend changes show immediately
5. **Backend changes require restart** - close and run `npm run tauri:dev` again

---

## 🎉 Success Criteria

You'll know it's working when:
- ✅ Game window opens
- ✅ You can select a club
- ✅ Console shows:
  ```
  ╔══════════════════════════════════════════════════════════════════════════╗
  ║  INITIALIZING SHEFFIELD & HALLAMSHIRE FANTASY LEAGUE - 1867             ║
  ╚══════════════════════════════════════════════════════════════════════════╝
  ```
- ✅ All 7 steps show with progress
- ✅ Cup competitions load (Youdan + Cromwell)
- ✅ Final summary shows 372 clubs
- ✅ Game starts successfully

---

## 📞 Need Help?

If something doesn't work:
1. Check the terminal output for error messages
2. Make sure Node.js is in PATH: `npm --version` should work
3. Make sure in correct folder: `pwd` should show `D:\projects\Saturday at Three\frontend`
4. Try clean reinstall: `rm -rf node_modules && npm install`
5. Check Rust compilation: `cd ../src-tauri && cargo build`

---

## 🔗 Related Documentation

- `INITIALIZATION_LOGGING_EXAMPLE.md` - Shows what the new logging looks like
- `SHEFFIELD_GAME_INITIALIZATION_FIXED.md` - Technical details of fixes
- `SHEFFIELD1867_DB_REQUIREMENTS.md` - Database requirements

---

**Ready? Let's run it!**

```bash
$env:PATH = "d:\nodejs;$env:PATH"
cd "D:\projects\Saturday at Three\frontend"
npm install
npm run tauri:dev
```

🎮 Enjoy the game!
