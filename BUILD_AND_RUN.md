# Build and Run Instructions

The compilation is running but may take 3-5 minutes due to Rust's compilation time and the new census importer module.

## If Compilation is Stuck

If `cargo build` appears to hang, try these steps:

### Option 1: Simple Build
```bash
cd "D:\projects\Saturday at Three\src-tauri"
cargo build
```

Wait 3-5 minutes. Rust compilation can be silent while working.

### Option 2: With Progress
```bash
cd "D:\projects\Saturday at Three\src-tauri"
cargo build --verbose
```

This will show each file being compiled.

### Option 3: If File Locks Exist
```bash
# Close any running instances of the app
# Then:
cd "D:\projects\Saturday at Three"
taskkill /F /IM saturday-at-three.exe
cd src-tauri
cargo build
```

### Option 4: Clean Build
```bash
cd "D:\projects\Saturday at Three\src-tauri"
cargo clean
cargo build --release
```

## Run the App

Once compilation completes successfully:

### Option A: Tauri Dev Mode
```bash
cd "D:\projects\Saturday at Three"
npm run tauri-dev
```

### Option B: Frontend Dev + Backend Separately
```bash
# Terminal 1 - Frontend
cd "D:\projects\Saturday at Three\frontend"
npm run dev

# Terminal 2 - Backend
cd "D:\projects\Saturday at Three\src-tauri"
cargo run
```

### Option C: Production Build
```bash
cd "D:\projects\Saturday at Three"
npm run tauri build
```

## Test the New Features

Once the app is running:

1. Open Database Editor from main menu
2. Go to "Club Management" tab
   - Click "💾 Create Backup Now"
   - Click "Show Backups" to see it listed
3. Go to "Create Players" tab
   - Click "🚀 Bulk Import ALL 73 Census Files" (orange button)
   - Confirm the dialog
   - Wait ~2 minutes
   - See success message with statistics
4. Go to "Player Database" tab
   - Search for players
   - Verify ~2000+ players imported
   - Check census data is populated

## Troubleshooting

### Compilation Errors
If you see errors about missing modules:
```bash
cd src-tauri
cargo update
cargo build
```

### Missing Dependencies
```bash
cd frontend
npm install
cd ../src-tauri
cargo fetch
```

### File Locks
Close all instances of the app, VSCode, and any terminals running the app.

### Node/NPM Issues
Make sure Node.js is in your PATH:
```bash
export PATH="/c/Program Files/nodejs:$PATH"
node --version
npm --version
```

## Current Build Status

The Rust backend is compiling in the background. This includes:
- ✅ backup.rs (backup system)
- ✅ census_importer.rs (bulk CSV import)
- ✅ All Tauri commands registered
- ✅ Frontend components ready

Once `cargo build` completes (you'll see "Finished" message), you can run the app!

## Quick Verification

After build completes, verify files exist:
```bash
# Check backend binary
ls -la "D:\projects\Saturday at Three\src-tauri\target\debug\saturday-at-three.exe"

# Check frontend components
ls -la "D:\projects\Saturday at Three\frontend\src\components\CsvImportButton.tsx"
ls -la "D:\projects\Saturday at Three\frontend\src\components\ConfirmationDialog.tsx"
```

## Expected Build Time

- First build: 5-10 minutes (downloading crates, compiling)
- Subsequent builds: 1-2 minutes (incremental)
- Release build: 10-15 minutes (optimizations)

## Success Indicators

You'll know compilation succeeded when you see:
```
    Finished dev [unoptimized + debuginfo] target(s) in X.XXs
```

Then you can run the app and test all the new features!
