/**
 * Live Match View — portrait pitch renderer consuming live_frame events
 * from the Rust match engine.
 *
 * Engine coordinate system:
 *   x: 0.0 (home goal line) → 1.0 (away goal line)   [pitch LENGTH]
 *   y: 0.0 (left touchline) → 1.0 (right touchline)  [pitch WIDTH]
 *
 * Portrait display mapping:
 *   engine x → screen Y  (home goal = top, away goal = bottom)
 *   engine y → screen X  (left touchline = left, right touchline = right)
 *
 * Direction remapping:
 *   engine dir_deg (0=right, 90=down, 180=left, 270=up) [landscape frame]
 *   +90° → screen dir_deg for portrait frame
 *
 * Sprite layer order (back → front):
 *   background → skin → hair → shorts → socks → shirt
 * GK has no shirt layer; background layer is coloured per team.
 */

import React, { useEffect, useRef } from 'react';
import { listen } from '@tauri-apps/api/event';
import './LiveMatchView.css';

// ─── Types mirroring the Rust FrameState ──────────────────────────────────────

type Side  = 'Home' | 'Away';
type Pose  = 'Stand' | 'Walk' | 'Run' | 'Kick' | 'Slide' | 'Fall' | 'Header';
type Phase = 'FirstHalf' | 'HalfTime' | 'SecondHalf' | 'FullTime';

// Rust unit enum serialises as a plain string e.g. "Goal", "Shot", "HalfTime"
type EventKind =
  | 'KickOff' | 'Pass' | 'Shot' | 'Goal' | 'Save' | 'Tackle' | 'Miss'
  | 'ThrowIn' | 'GoalKick' | 'Corner' | 'FreeKick' | 'HalfTime' | 'FullTime';

interface MatchEvent {
  tick: number;
  minute: number;
  second: number;
  kind: EventKind;
  side: Side;
  ball_x: number;
  ball_y: number;
  player_id: string | null;
  description: string;
}

interface PlayerSnap {
  id: string;
  side: Side;
  x: number;
  y: number;
  dir_deg: number;
  pose: Pose;
  pose_timer: number;
  has_ball: boolean;
  skin_tone: number;
  is_gk: boolean;
  name: string;
}

interface FrameState {
  tick: number;
  minute: number;
  second: number;
  ball_x: number;
  ball_y: number;
  ball_z: number;
  ball_speed: number;
  ball_frame: number;
  players: PlayerSnap[];
  home_score: number;
  away_score: number;
  possession: Side;
  home_poss_pct: number;
  phase: Phase;
  event: MatchEvent | null;
}

// ─── Props ────────────────────────────────────────────────────────────────────

interface LiveMatchViewProps {
  matchId: string;
  homeClubName: string;
  awayClubName: string;
  onFullTime: (homeScore: number, awayScore: number) => void;
}

// ─── Sprite sets ──────────────────────────────────────────────────────────────

interface SpriteSet {
  shirt:         HTMLImageElement[];
  shirtAway:     HTMLCanvasElement[];   // red↔blue swapped
  skin:          HTMLCanvasElement[][];  // [variant 0-3][frame]
  hair:          HTMLImageElement[];
  shorts:        HTMLImageElement[];
  socks:         HTMLImageElement[];
  background:    HTMLImageElement[];
}

interface GkSpriteSet {
  background:    HTMLImageElement[];
  backgroundAway: HTMLCanvasElement[];
  skin:          HTMLCanvasElement[][];
  hair:          HTMLImageElement[];
  shorts:        HTMLImageElement[];
  socks:         HTMLImageElement[];
}

interface BallSprites {
  frames: HTMLImageElement[];
  shadow: HTMLImageElement | null;
}

// ─── Pitch colours ────────────────────────────────────────────────────────────

const G_DARK  = '#3cb800';
const G_LIGHT = '#44cc00';
const LINE    = '#fcfcfc';
const GOAL_C  = '#e8e8e8';

// ─── Skin tone tints ──────────────────────────────────────────────────────────

const SKIN_TINTS: Array<[number, number, number]> = [
  [255, 200, 160],
  [160, 100,  60],
  [210, 150, 100],
  [190, 155,  95],
];

function buildSkinVariant(img: HTMLImageElement, v: number): HTMLCanvasElement {
  const oc = document.createElement('canvas');
  const nw = img.naturalWidth;
  const nh = img.naturalHeight;
  oc.width  = nw || 1;
  oc.height = nh || 1;
  if (!nw || !nh) return oc; // broken/missing image — return blank canvas
  const cx = oc.getContext('2d')!;
  cx.drawImage(img, 0, 0);
  if (v === 0) return oc;
  const id = cx.getImageData(0, 0, oc.width, oc.height);
  const d  = id.data;
  const [tr, tg, tb] = SKIN_TINTS[v];
  for (let i = 0; i < d.length; i += 4) {
    if (d[i + 3] < 10) continue;
    const r = d[i], g = d[i + 1], b = d[i + 2];
    if (r > 150 && r > g && g > b && r - b > 30) {
      d[i]     = Math.round(r * 0.4 + tr * 0.6);
      d[i + 1] = Math.round(g * 0.4 + tg * 0.6);
      d[i + 2] = Math.round(b * 0.4 + tb * 0.6);
    }
  }
  cx.putImageData(id, 0, 0);
  return oc;
}

// Red↔Blue swap for away team shirts / GK background
function buildSwappedCanvas(img: HTMLImageElement): HTMLCanvasElement {
  const oc = document.createElement('canvas');
  const nw = img.naturalWidth;
  const nh = img.naturalHeight;
  oc.width  = nw || 1;
  oc.height = nh || 1;
  if (!nw || !nh) return oc; // broken/missing image
  const cx = oc.getContext('2d')!;
  cx.drawImage(img, 0, 0);
  const id = cx.getImageData(0, 0, oc.width, oc.height);
  const d  = id.data;
  for (let i = 0; i < d.length; i += 4) {
    if (d[i + 3] < 10) continue;
    const r = d[i], g = d[i + 1], b = d[i + 2];
    if ((b > r && b > g + 20) || (r > b && r > g + 20)) {
      d[i] = b; d[i + 2] = r;
    }
  }
  cx.putImageData(id, 0, 0);
  return oc;
}

// ─── Direction mapping ────────────────────────────────────────────────────────

// Sprite dir8 index:
//   0=down, 1=down-left, 2=left, 3=up-left, 4=up, 5=up-right, 6=right, 7=down-right
// SWOS sprite sheets are laid out with frames grouped by dir8.

function degToDir8(deg: number): number {
  // Round to nearest 45° step (0..7) then map to sprite dir8
  const step = Math.round(((deg % 360) + 360) % 360 / 45) % 8;
  // step: 0=0°(right) 1=45° 2=90°(down) 3=135° 4=180°(left) 5=225° 6=270°(up) 7=315°
  const map = [6, 7, 0, 1, 2, 3, 4, 5];
  return map[step];
}

// ─── Frame index ──────────────────────────────────────────────────────────────

function spriteFrame(pose: Pose, dir8: number, walkStep: number, poseTimer: number): number {
  switch (pose) {
    case 'Header': return 56 + Math.min(7, Math.floor(poseTimer / 3));
    case 'Slide':  return 64 + Math.min(7, Math.floor(poseTimer / 4));
    case 'Fall':   return 72 + Math.min(7, Math.floor(poseTimer / 5));
    case 'Kick':   return 80 + Math.min(15, Math.floor(poseTimer / 2));
    case 'Stand':  return dir8 * 8;
    default:       return dir8 * 8 + (walkStep % 8); // Walk / Run
  }
}

// ─── Draw a sprite layer anchored at feet (bottom-centre) ────────────────────

function drawSpriteAt(
  ctx: CanvasRenderingContext2D,
  src: HTMLImageElement | HTMLCanvasElement | undefined | null,
  sx: number, sy: number,
  scale: number,
) {
  if (!src) return;
  const nw = src instanceof HTMLImageElement ? src.naturalWidth  : src.width;
  const nh = src instanceof HTMLImageElement ? src.naturalHeight : src.height;
  if (!nw || !nh) return;
  const dw = nw * scale;
  const dh = nh * scale;
  ctx.drawImage(src, sx - dw / 2, sy - dh, dw, dh);
}

// ─── Module-level sprite cache (filled after load) ───────────────────────────

let _sprites: SpriteSet | null   = null;
let _gkSprites: GkSpriteSet | null = null;
let _ball: BallSprites = { frames: [], shadow: null };

// ─── Component ────────────────────────────────────────────────────────────────

export function LiveMatchView({ matchId, homeClubName, awayClubName, onFullTime }: LiveMatchViewProps) {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const rafRef    = useRef<number>();

  const frameRef   = useRef<FrameState | null>(null);
  const prevRef    = useRef<FrameState | null>(null);
  const interpRef  = useRef(1.0);

  const timeMs  = useRef(0);
  const lastTs  = useRef(0);

  const lastEvRef    = useRef<MatchEvent | null>(null);
  const evShownUntil = useRef(0);
  const phaseBanner  = useRef<{ text: string; until: number } | null>(null);

  const spritesOk     = useRef(false);
  const fullTimeFired = useRef(false);

  // ── Load sprites ─────────────────────────────────────────────────────────
  useEffect(() => {
    const loadImg = (src: string): Promise<HTMLImageElement> =>
      new Promise(res => {
        const i = new Image();
        i.onload  = () => res(i);
        i.onerror = () => res(i);
        i.src = src;
      });

    const loadLayer = (folder: string, n: number): Promise<HTMLImageElement[]> =>
      Promise.all(
        Array.from({ length: n }, (_, i) =>
          loadImg(`/sprites/player/${folder}/spr${String(i).padStart(4, '0')}.png`)
        )
      );

    const loadGkLayer = (folder: string): Promise<HTMLImageElement[]> =>
      Promise.all(
        Array.from({ length: 58 }, (_, i) =>
          loadImg(`/sprites/goalkeeper/${folder}/spr${String(i).padStart(4, '0')}.png`)
        )
      );

    (async () => {
      const [shirt, skinRaw, hair, shorts, socks, bg,
             gkBg, gkSkinRaw, gkHair, gkShorts, gkSocks] = await Promise.all([
        loadLayer('shirt', 106),
        loadLayer('skin',  106),
        loadLayer('hair',  106),
        loadLayer('shorts', 106),
        loadLayer('socks',  106),
        loadLayer('background', 101),
        loadGkLayer('background'),
        loadGkLayer('skin'),
        loadGkLayer('hair'),
        loadGkLayer('shorts'),
        loadGkLayer('socks'),
      ]);

      const skin   = [0, 1, 2, 3].map(v => skinRaw.map(i => buildSkinVariant(i, v)));
      const gkSkin = [0, 1, 2, 3].map(v => gkSkinRaw.map(i => buildSkinVariant(i, v)));

      const shirtAway   = shirt.map(i => buildSwappedCanvas(i));
      const gkBgAway    = gkBg.map(i => buildSwappedCanvas(i));

      _sprites = { shirt, shirtAway, skin, hair, shorts, socks, background: bg };
      _gkSprites = { background: gkBg, backgroundAway: gkBgAway, skin: gkSkin, hair: gkHair, shorts: gkShorts, socks: gkSocks };

      const [bFrames, ballShadow] = await Promise.all([
        Promise.all([1179, 1180, 1181, 1182].map(i => loadImg(`/sprites/ball/spr${i}.png`))),
        loadImg('/sprites/ball/spr1183.png'),
      ]);
      _ball = { frames: bFrames, shadow: ballShadow };

      spritesOk.current = true;
    })();
  }, []);

  // ── Listen for engine frames ─────────────────────────────────────────────
  useEffect(() => {
    let unlisten: (() => void) | null = null;

    listen<FrameState>('live_frame', ev => {
      if (ev.payload.tick === undefined) return;
      prevRef.current  = frameRef.current;
      frameRef.current = ev.payload;
      interpRef.current = 0;

      if (ev.payload.event) {
        lastEvRef.current    = ev.payload.event;
        evShownUntil.current = timeMs.current + 3500;

        const k = ev.payload.event.kind;
        if (k === 'HalfTime' || k === 'FullTime') {
          const label = k === 'HalfTime' ? 'HALF TIME' : 'FULL TIME';
          phaseBanner.current = { text: label, until: timeMs.current + 4000 };
        }

        if (k === 'FullTime' && !fullTimeFired.current) {
          fullTimeFired.current = true;
          setTimeout(() => {
            const f = frameRef.current;
            if (f) onFullTime(f.home_score, f.away_score);
          }, 2500);
        }
      }
    }).then(u => { unlisten = u; });

    return () => { if (unlisten) unlisten(); };
  }, [matchId, onFullTime]);

  // ── Render loop ──────────────────────────────────────────────────────────
  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    const resize = () => {
      const p = canvas.parentElement;
      if (p) { canvas.width = p.clientWidth; canvas.height = p.clientHeight; }
    };
    resize();
    window.addEventListener('resize', resize);

    const render = (ts: number) => {
      const dt = lastTs.current ? ts - lastTs.current : 16;
      lastTs.current  = ts;
      timeMs.current += dt;

      interpRef.current = Math.min(1, interpRef.current + dt / 16);
      const t    = interpRef.current;
      const prev = prevRef.current;
      const next = frameRef.current;

      const W = canvas.width;
      const H = canvas.height;

      // Background
      ctx.fillStyle = '#1a1a2a';
      ctx.fillRect(0, 0, W, H);

      // Portrait pitch: narrower than tall (3:4 aspect)
      const pitchRatio = 3 / 4; // width:height
      let pW = W * 0.72;
      let pH = pW / pitchRatio;
      if (pH > H * 0.88) { pH = H * 0.88; pW = pH * pitchRatio; }
      const pL = (W - pW) / 2;
      const pT = (H - pH) / 2 + 18; // shift down slightly for HUD
      const pB = pT + pH;

      // Engine → screen coordinate helpers
      // engine x (0=home goal, 1=away goal) → screen Y
      // engine y (0=left touchline, 1=right touchline) → screen X
      const toSX = (ey: number) => pL + ey * pW;
      const toSY = (ex: number) => pT + ex * pH;

      // SWOS pitch is 672 game-units wide. The PNG sprites are upscaled ~6× from original
      // pixel art (~24px wide player). A player should occupy ~5% of pitch width on screen.
      // spriteScale = pW / (672 * 6) ≈ pW / 4032; tuned to ~pW/2600 for visual fit.
      const spriteScale = pW / 2600;

      drawPitch(ctx, pL, pT, pW, pH);

      if (!next) {
        rafRef.current = requestAnimationFrame(render);
        return;
      }

      // Interpolated ball position
      const bEngX = prev ? prev.ball_x + (next.ball_x - prev.ball_x) * t : next.ball_x;
      const bEngY = prev ? prev.ball_y + (next.ball_y - prev.ball_y) * t : next.ball_y;
      const bSX   = toSX(bEngY);
      const bSY   = toSY(bEngX);
      const bz    = next.ball_z;

      // Walk cycle
      const walkStep = Math.floor(timeMs.current / 80) % 8;

      // Y-sorted draw list
      type Entry = { sy: number; draw: () => void };
      const entries: Entry[] = [];

      for (const p of next.players) {
        const pp = prev?.players.find(q => q.id === p.id);
        const pEngX = pp ? pp.x + (p.x - pp.x) * t : p.x;
        const pEngY = pp ? pp.y + (p.y - pp.y) * t : p.y;
        const sx = toSX(pEngY);
        const sy = toSY(pEngX);

        // Direction: engine is landscape (0=right); portrait needs +90°
        const screenDeg = (p.dir_deg + 90) % 360;
        const dir8 = degToDir8(screenDeg);
        const fi   = spriteFrame(p.pose, dir8, walkStep, p.pose_timer);
        const sv   = Math.min(3, p.skin_tone);
        const isAway = p.side === 'Away';

        entries.push({ sy, draw: () => {
          drawShadow(ctx, sx, sy, spriteScale);

          if (!_sprites || !_gkSprites) return; // no fallback shapes

          if (p.is_gk) {
            const gk = _gkSprites;
            const sfr = Math.min(57, fi);
            drawSpriteAt(ctx, isAway ? gk.backgroundAway[sfr] : gk.background[sfr], sx, sy, spriteScale);
            drawSpriteAt(ctx, gk.skin[sv]?.[sfr], sx, sy, spriteScale);
            drawSpriteAt(ctx, gk.hair[sfr], sx, sy, spriteScale);
            drawSpriteAt(ctx, gk.shorts[sfr], sx, sy, spriteScale);
            drawSpriteAt(ctx, gk.socks[sfr], sx, sy, spriteScale);
          } else {
            const sp  = _sprites;
            const sfr = Math.min(105, fi);
            const bgFr = Math.min(100, sfr);
            drawSpriteAt(ctx, sp.background[bgFr], sx, sy, spriteScale);
            drawSpriteAt(ctx, sp.skin[sv]?.[sfr], sx, sy, spriteScale);
            drawSpriteAt(ctx, sp.hair[sfr], sx, sy, spriteScale);
            drawSpriteAt(ctx, sp.shorts[sfr], sx, sy, spriteScale);
            drawSpriteAt(ctx, sp.socks[sfr], sx, sy, spriteScale);
            if (isAway) {
              drawSpriteAt(ctx, sp.shirtAway[sfr], sx, sy, spriteScale);
            } else {
              drawSpriteAt(ctx, sp.shirt[sfr], sx, sy, spriteScale);
            }
          }

          // Name label on ball carrier
          if (p.has_ball) {
            const labelY = sy - 180 * spriteScale - 4; // 180 = sprite natural height
            ctx.fillStyle = isAway ? '#ffaaaa' : '#aaccff';
            ctx.font = `bold ${Math.max(8, Math.round(180 * spriteScale * 0.22))}px monospace`;
            ctx.textAlign = 'center';
            ctx.textBaseline = 'bottom';
            ctx.fillText(p.name, sx, labelY);
          }
        }});
      }

      // Ball always on top
      entries.push({ sy: Infinity, draw: () => {
        drawBall(ctx, bSX, bSY, bz, next.ball_frame);
      }});

      entries.sort((a, b) => a.sy - b.sy);
      for (const e of entries) e.draw();

      // ── HUD ─────────────────────────────────────────────────────────────
      drawScoreboard(ctx, W, homeClubName, awayClubName, next.home_score, next.away_score);
      drawClock(ctx, W, next.minute, next.second, next.phase);
      drawPossessionBar(ctx, W, pB, next.home_poss_pct);

      const ev = lastEvRef.current;
      if (ev && timeMs.current < evShownUntil.current) drawEventFlash(ctx, W, H, ev);

      const banner = phaseBanner.current;
      if (banner && timeMs.current < banner.until)
        drawPhaseBanner(ctx, W, H, banner.text, next.home_score, next.away_score, homeClubName, awayClubName);

      rafRef.current = requestAnimationFrame(render);
    };

    rafRef.current = requestAnimationFrame(render);
    return () => {
      window.removeEventListener('resize', resize);
      if (rafRef.current) cancelAnimationFrame(rafRef.current);
    };
  }, [homeClubName, awayClubName]);

  return (
    <div className="live-match-view">
      <canvas ref={canvasRef} className="live-match-canvas" />
    </div>
  );
}

// ─── Shadow ───────────────────────────────────────────────────────────────────

function drawShadow(ctx: CanvasRenderingContext2D, sx: number, sy: number, scale: number) {
  ctx.fillStyle = 'rgba(0,0,0,0.25)';
  ctx.beginPath();
  ctx.ellipse(sx + 1, sy - 1, 22 * scale, 8 * scale, 0, 0, Math.PI * 2);
  ctx.fill();
}

// ─── Ball ─────────────────────────────────────────────────────────────────────

function drawBall(ctx: CanvasRenderingContext2D, sx: number, sy: number, bz: number, bFrame: number) {
  const BALL_NAT = 48;
  const ballScale = 0.22; // ball display scale relative to natural size
  const bw = BALL_NAT * ballScale;
  const bh = BALL_NAT * ballScale;
  const lift = bz * bh * 2.5;

  // Shadow
  if (_ball.shadow?.complete && _ball.shadow.naturalWidth > 0) {
    const sa = 0.5 * (1 - bz * 0.65);
    const ss = 1 - bz * 0.35;
    ctx.globalAlpha = sa;
    ctx.drawImage(_ball.shadow, sx - bw * ss / 2 + 2, sy - bh * ss / 2 + 3, bw * ss, bh * ss);
    ctx.globalAlpha = 1;
  }

  const fr = _ball.frames[Math.max(0, Math.min(3, bFrame))];
  if (fr?.complete && fr.naturalWidth > 0) {
    ctx.drawImage(fr, sx - bw / 2, sy - bh / 2 - lift, bw, bh);
  }
  // No fallback shape per user instruction
}

// ─── Pitch ────────────────────────────────────────────────────────────────────

function drawPitch(ctx: CanvasRenderingContext2D, pL: number, pT: number, pW: number, pH: number) {
  // Horizontal stripes (portrait: stripes run left-right)
  const STRIPES = 8;
  const sh = pH / STRIPES;
  for (let i = 0; i < STRIPES; i++) {
    ctx.fillStyle = i % 2 === 0 ? G_DARK : G_LIGHT;
    ctx.fillRect(pL, pT + i * sh, pW, sh);
  }

  ctx.strokeStyle = LINE; ctx.lineWidth = 2;
  ctx.strokeRect(pL, pT, pW, pH);

  // Halfway line (horizontal, since pitch is portrait)
  ctx.beginPath();
  ctx.moveTo(pL, pT + pH / 2);
  ctx.lineTo(pL + pW, pT + pH / 2);
  ctx.stroke();

  // Centre circle
  const cR = pW * 0.17;
  ctx.beginPath(); ctx.arc(pL + pW / 2, pT + pH / 2, cR, 0, Math.PI * 2); ctx.stroke();
  ctx.fillStyle = LINE;
  ctx.beginPath(); ctx.arc(pL + pW / 2, pT + pH / 2, 3, 0, Math.PI * 2); ctx.fill();

  // Penalty areas (top = home, bottom = away)
  const paH_frac = 0.18;  // depth along pitch length
  const paW_frac = 0.58;  // width across pitch
  const paDepth  = pH * paH_frac;
  const paWidth  = pW * paW_frac;
  const paLeft   = pL + (pW - paWidth) / 2;

  // Home (top)
  ctx.strokeRect(paLeft, pT, paWidth, paDepth);
  // Away (bottom)
  ctx.strokeRect(paLeft, pT + pH - paDepth, paWidth, paDepth);

  // Goal areas
  const gaH_frac = 0.08;
  const gaW_frac = 0.32;
  const gaDepth  = pH * gaH_frac;
  const gaWidth  = pW * gaW_frac;
  const gaLeft   = pL + (pW - gaWidth) / 2;

  ctx.strokeRect(gaLeft, pT, gaWidth, gaDepth);
  ctx.strokeRect(gaLeft, pT + pH - gaDepth, gaWidth, gaDepth);

  // Penalty spots
  ctx.fillStyle = LINE;
  ctx.beginPath(); ctx.arc(pL + pW / 2, pT + pH * 0.13, 2.5, 0, Math.PI * 2); ctx.fill();
  ctx.beginPath(); ctx.arc(pL + pW / 2, pT + pH * 0.87, 2.5, 0, Math.PI * 2); ctx.fill();

  // Penalty arcs
  const arcR = pW * 0.17;
  // Home arc (bottom edge of home penalty area)
  ctx.beginPath();
  ctx.arc(pL + pW / 2, pT + pH * 0.13, arcR, Math.PI * 0.15, Math.PI * 0.85);
  ctx.stroke();
  // Away arc (top edge of away penalty area)
  ctx.beginPath();
  ctx.arc(pL + pW / 2, pT + pH * 0.87, arcR, -Math.PI * 0.85, -Math.PI * 0.15);
  ctx.stroke();

  // Goals (above/below pitch)
  const goalW_frac = 0.27; // 7.32m / 68m * pitch_width proportion
  const goalW  = pW * goalW_frac;
  const goalLeft = pL + (pW - goalW) / 2;
  const goalDepth = 12;

  ctx.fillStyle = 'rgba(255,255,255,0.08)';
  ctx.fillRect(goalLeft, pT - goalDepth, goalW, goalDepth);   // home goal (top)
  ctx.fillRect(goalLeft, pT + pH, goalW, goalDepth);           // away goal (bottom)
  ctx.strokeStyle = GOAL_C; ctx.lineWidth = 1.5;
  ctx.strokeRect(goalLeft, pT - goalDepth, goalW, goalDepth);
  ctx.strokeRect(goalLeft, pT + pH, goalW, goalDepth);

  // Corner arcs
  const cr = 8;
  ctx.strokeStyle = LINE; ctx.lineWidth = 1.5;
  [
    [pL,      pT,      0,              Math.PI / 2],
    [pL + pW, pT,      Math.PI / 2,   Math.PI],
    [pL + pW, pT + pH, Math.PI,       Math.PI * 1.5],
    [pL,      pT + pH, Math.PI * 1.5, Math.PI * 2],
  ].forEach(([x, y, s, e]) => {
    ctx.beginPath(); ctx.arc(x as number, y as number, cr, s as number, e as number); ctx.stroke();
  });
}

// ─── HUD ──────────────────────────────────────────────────────────────────────

function drawScoreboard(
  ctx: CanvasRenderingContext2D, W: number,
  home: string, away: string, hs: number, as_: number,
) {
  const bW = 300; const bH = 34;
  const bX = (W - bW) / 2; const bY = 4;
  ctx.fillStyle = 'rgba(0,0,0,0.85)';
  ctx.fillRect(bX, bY, bW, bH);
  ctx.strokeStyle = '#555'; ctx.lineWidth = 1;
  ctx.strokeRect(bX, bY, bW, bH);

  ctx.textBaseline = 'middle';
  const mid = bY + bH / 2;
  ctx.fillStyle = '#aaccff'; ctx.font = 'bold 11px monospace';
  ctx.textAlign = 'right';
  ctx.fillText(home.substring(0, 14), bX + bW * 0.43, mid);
  ctx.fillStyle = '#fff'; ctx.font = 'bold 18px monospace';
  ctx.textAlign = 'center';
  ctx.fillText(`${hs}  –  ${as_}`, bX + bW / 2, mid);
  ctx.fillStyle = '#ffaaaa'; ctx.font = 'bold 11px monospace';
  ctx.textAlign = 'left';
  ctx.fillText(away.substring(0, 14), bX + bW * 0.57, mid);
}

function drawClock(
  ctx: CanvasRenderingContext2D, W: number,
  minute: number, second: number, phase: Phase,
) {
  const mm = String(minute).padStart(2, '0');
  const ss = String(second).padStart(2, '0');
  const x = 10; const y = 4; const bW = 72; const bH = 34;
  ctx.fillStyle = 'rgba(0,0,0,0.85)';
  ctx.fillRect(x, y, bW, bH);
  ctx.fillStyle = '#fcfc00'; ctx.font = 'bold 18px monospace';
  ctx.textAlign = 'center'; ctx.textBaseline = 'middle';
  ctx.fillText(`${mm}:${ss}`, x + bW / 2, y + bH / 2);
  ctx.fillStyle = phase === 'SecondHalf' ? '#ff6644' : '#88ff88';
  ctx.beginPath(); ctx.arc(x + bW - 6, y + bH - 6, 4, 0, Math.PI * 2); ctx.fill();
}

function drawPossessionBar(
  ctx: CanvasRenderingContext2D, W: number,
  pitchBottom: number, homePct: number,
) {
  const bW = 220; const bH = 14;
  const bX = (W - bW) / 2; const bY = pitchBottom + 6;
  ctx.fillStyle = 'rgba(0,0,0,0.6)';
  ctx.fillRect(bX, bY, bW, bH);
  ctx.fillStyle = '#5599ff';
  ctx.fillRect(bX, bY, bW * (homePct / 100), bH);
  ctx.fillStyle = '#ff5544';
  ctx.fillRect(bX + bW * (homePct / 100), bY, bW * (1 - homePct / 100), bH);
  ctx.strokeStyle = '#333'; ctx.lineWidth = 1;
  ctx.strokeRect(bX, bY, bW, bH);
  ctx.fillStyle = '#fff'; ctx.font = '9px monospace';
  ctx.textAlign = 'center'; ctx.textBaseline = 'middle';
  ctx.fillText(`${Math.round(homePct)}% – ${Math.round(100 - homePct)}%`, bX + bW / 2, bY + bH / 2);
}

function drawEventFlash(ctx: CanvasRenderingContext2D, W: number, _H: number, ev: MatchEvent) {
  const txt = ev.description;
  if (!txt) return;
  const maxW = 320; const bH = 20;
  const bY = 42; // below the HUD bar (clock + scoreboard sit at y=4, height=34)
  ctx.fillStyle = 'rgba(0,0,0,0.72)';
  ctx.fillRect((W - maxW) / 2, bY, maxW, bH);
  const k = ev.kind;
  let col = '#fff';
  if (k === 'Goal')     col = '#55ff55';
  if (k === 'Save')     col = '#55aaff';
  if (k === 'Shot')     col = '#ffdd44';
  if (k === 'ThrowIn')  col = '#00ccff';
  if (k === 'Corner')   col = '#ffaa44';
  if (k === 'FreeKick') col = '#ffee00';
  ctx.fillStyle = col; ctx.font = 'bold 12px monospace';
  ctx.textAlign = 'center'; ctx.textBaseline = 'middle';
  ctx.fillText(txt, W / 2, bY + bH / 2);
}

function drawPhaseBanner(
  ctx: CanvasRenderingContext2D, W: number, H: number,
  label: string, hs: number, as_: number, home: string, away: string,
) {
  const bW = 280; const bH = 70;
  const bX = (W - bW) / 2; const bY = (H - bH) / 2;
  ctx.fillStyle = 'rgba(0,0,0,0.88)';
  ctx.fillRect(bX, bY, bW, bH);
  ctx.strokeStyle = '#c8a000'; ctx.lineWidth = 2;
  ctx.strokeRect(bX, bY, bW, bH);
  ctx.fillStyle = '#c8a000'; ctx.font = 'bold 22px monospace';
  ctx.textAlign = 'center'; ctx.textBaseline = 'top';
  ctx.fillText(label, W / 2, bY + 8);
  ctx.fillStyle = '#fff'; ctx.font = 'bold 20px monospace';
  ctx.fillText(`${hs}  –  ${as_}`, W / 2, bY + 38);
}
