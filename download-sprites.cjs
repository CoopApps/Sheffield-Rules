// Download all match-day sprites from swos-port repo
const https = require('https');
const fs = require('fs');
const path = require('path');

const BASE = 'https://raw.githubusercontent.com/zlatkok/swos-port/master/assets/sprites/game';
const OUT  = 'D:/projects/Saturday at Three/frontend/public/sprites';

function download(url, dest) {
  return new Promise((resolve, reject) => {
    const file = fs.createWriteStream(dest);
    https.get(url, res => {
      if (res.statusCode !== 200) { file.close(); fs.unlink(dest, () => {}); return resolve(false); }
      res.pipe(file);
      file.on('finish', () => { file.close(); resolve(true); });
    }).on('error', err => { fs.unlink(dest, () => {}); resolve(false); });
  });
}

async function downloadLayer(repoPath, localPath, count) {
  let ok = 0;
  for (let i = 0; i < count; i++) {
    const n = String(i).padStart(4, '0');
    const url  = `${BASE}/${repoPath}/spr${n}.png`;
    const dest = `${localPath}/spr${n}.png`;
    if (fs.existsSync(dest)) { ok++; continue; }
    const success = await download(url, dest);
    if (success) ok++;
  }
  return ok;
}

async function main() {
  const tasks = [
    // player/background (101 frames, 0-100)
    { repo: 'player/background', local: `${OUT}/player/background`, count: 101 },
    // goalkeeper layers (58 frames each, 0-57)
    { repo: 'goalkeeper/background', local: `${OUT}/goalkeeper/background`, count: 58 },
    { repo: 'goalkeeper/hair',       local: `${OUT}/goalkeeper/hair`,       count: 58 },
    { repo: 'goalkeeper/shorts',     local: `${OUT}/goalkeeper/shorts`,     count: 58 },
    { repo: 'goalkeeper/skin',       local: `${OUT}/goalkeeper/skin`,       count: 58 },
    { repo: 'goalkeeper/socks',      local: `${OUT}/goalkeeper/socks`,      count: 58 },
    // stadium (9 frames, 0-8)
    { repo: 'stadium', local: `${OUT}/stadium`, count: 9 },
    // bench layers (12 frames each, 0-11)
    { repo: 'bench/background', local: `${OUT}/bench/background`, count: 12 },
    { repo: 'bench/shirt',      local: `${OUT}/bench/shirt`,      count: 12 },
  ];

  for (const t of tasks) {
    const n = await downloadLayer(t.repo, t.local, t.count);
    console.log(`${t.repo}: ${n}/${t.count} sprites`);
  }
  console.log('Done.');
}

main().catch(console.error);
