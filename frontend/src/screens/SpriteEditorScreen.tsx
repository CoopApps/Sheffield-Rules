/**
 * SpriteEditorScreen — Victorian sprite layer editor.
 *
 * Lets you preview and edit the historical_1867 sprite pack layers.
 * Each frame is a stack of transparent 144×180 PNGs composited in order.
 *
 * Layer order (player):  background → shirt → trousers → socks → boots → hair
 * Layer order (gk):      background → waistcoat → trousers → socks → boots → hair
 *
 * Editing capabilities:
 *   - Browse all 106 player / 58 GK frames
 *   - Toggle individual layers on/off
 *   - Preview with team colour (home/away) and skin tone variant
 *   - Download the current composited frame as PNG
 *   - Regenerate sprites (calls backend endpoint to re-run the generator script)
 */

import React, { useRef, useEffect, useState, useCallback } from 'react';

// ─── Constants ────────────────────────────────────────────────────────────────

const PACK = 'historical_1867';
const W = 144;
const H = 180;
const DISPLAY_SCALE = 4; // show at 4× for comfortable editing

const PLAYER_LAYERS = ['background', 'shirt', 'trousers', 'socks', 'boots', 'hair'] as const;
const GK_LAYERS     = ['background', 'waistcoat', 'trousers', 'socks', 'boots', 'hair'] as const;

type PlayerLayer = typeof PLAYER_LAYERS[number];
type GKLayer     = typeof GK_LAYERS[number];
type AnyLayer    = PlayerLayer | GKLayer;

const LAYER_LABELS: Record<AnyLayer, string> = {
  background: 'Background',
  shirt:      'Shirt',
  waistcoat:  'Waistcoat (GK)',
  trousers:   'Trousers',
  socks:      'Socks',
  boots:      'Boots',
  hair:       'Hair & Cap',
};

// Skin tone tints (matching LiveMatchView runtime system)
const SKIN_TINTS: [number, number, number][] = [
  [255, 200, 160],
  [160, 100,  60],
  [210, 150, 100],
  [190, 155,  95],
];

// Pose labels for frame groups
const POSE_GROUPS = [
  { label: 'Walk/Stand (dir 0 ↓)',    frames: [0,  7]  },
  { label: 'Walk/Stand (dir 1 ↙)',    frames: [8,  15] },
  { label: 'Walk/Stand (dir 2 ←)',    frames: [16, 23] },
  { label: 'Walk/Stand (dir 3 ↖)',    frames: [24, 31] },
  { label: 'Walk/Stand (dir 4 ↑)',    frames: [32, 39] },
  { label: 'Walk/Stand (dir 5 ↗)',    frames: [40, 47] },
  { label: 'Walk/Stand (dir 6 →)',    frames: [48, 55] },
  { label: 'Header',                  frames: [56, 63] },
  { label: 'Slide',                   frames: [64, 71] },
  { label: 'Fall',                    frames: [72, 79] },
  { label: 'Kick',                    frames: [80, 95] },
  { label: 'Misc',                    frames: [96, 105]},
];

// ─── Sprite path helper ───────────────────────────────────────────────────────

function spritePath(type: 'player' | 'goalkeeper', layer: AnyLayer, frame: number): string {
  const fn = `spr${String(frame).padStart(4, '0')}.png`;
  // Served from the assets/packs directory via the backend asset API
  return `/api/assets/${PACK}/sprites/${type}/${layer}/${fn}`;
}

function ballSpritePath(frame: number): string {
  const fn = `spr${String(1179 + frame).padStart(4, '0')}.png`;
  return `/api/assets/${PACK}/sprites/ball/${fn}`;
}

// ─── Image loader ─────────────────────────────────────────────────────────────

function loadImg(src: string): Promise<HTMLImageElement | null> {
  return new Promise(resolve => {
    const img = new Image();
    img.onload  = () => resolve(img);
    img.onerror = () => resolve(null);
    img.src = src;
  });
}

// ─── Skin tone tinting (matches LiveMatchView) ────────────────────────────────

function tintSkin(img: HTMLImageElement, tintIdx: number): HTMLCanvasElement {
  const c = document.createElement('canvas');
  c.width = img.naturalWidth;
  c.height = img.naturalHeight;
  const ctx = c.getContext('2d')!;
  ctx.drawImage(img, 0, 0);
  const d = ctx.getImageData(0, 0, c.width, c.height);
  const [tr, tg, tb] = SKIN_TINTS[tintIdx];
  for (let i = 0; i < d.data.length; i += 4) {
    if (d.data[i+3] > 0) {
      d.data[i]   = Math.round(d.data[i]   * 0.4 + tr * 0.6);
      d.data[i+1] = Math.round(d.data[i+1] * 0.4 + tg * 0.6);
      d.data[i+2] = Math.round(d.data[i+2] * 0.4 + tb * 0.6);
    }
  }
  ctx.putImageData(d, 0, 0);
  return c;
}

// Away team colour swap: red ↔ blue (matching LiveMatchView)
function swapTeamColor(img: HTMLImageElement): HTMLCanvasElement {
  const c = document.createElement('canvas');
  c.width = img.naturalWidth;
  c.height = img.naturalHeight;
  const ctx = c.getContext('2d')!;
  ctx.drawImage(img, 0, 0);
  const d = ctx.getImageData(0, 0, c.width, c.height);
  for (let i = 0; i < d.data.length; i += 4) {
    if (d.data[i+3] > 0) {
      const r = d.data[i], b = d.data[i+2];
      if (r > b + 30) { d.data[i] = b; d.data[i+2] = r; }
      else if (b > r + 30) { d.data[i] = b; d.data[i+2] = r; }
    }
  }
  ctx.putImageData(d, 0, 0);
  return c;
}

// ─── Component ────────────────────────────────────────────────────────────────

interface Props {
  onBack: () => void;
}

export function SpriteEditorScreen({ onBack }: Props) {
  const canvasRef    = useRef<HTMLCanvasElement>(null);
  const [type, setType]           = useState<'player' | 'goalkeeper'>('player');
  const [frame, setFrame]         = useState(0);
  const [skinTone, setSkinTone]   = useState(0);
  const [team, setTeam]           = useState<'home' | 'away'>('home');
  const [animating, setAnimating] = useState(false);
  const [animFrame, setAnimFrame] = useState(0);
  const animRef = useRef<ReturnType<typeof setInterval> | null>(null);

  const maxFrame = type === 'player' ? 105 : 57;
  const layers: readonly AnyLayer[] = type === 'player' ? PLAYER_LAYERS : GK_LAYERS;

  const [visible, setVisible] = useState<Record<AnyLayer, boolean>>(() => {
    const v: Partial<Record<AnyLayer, boolean>> = {};
    for (const l of [...PLAYER_LAYERS, ...GK_LAYERS]) v[l] = true;
    return v as Record<AnyLayer, boolean>;
  });

  // ── Composite and render to canvas ──────────────────────────────────────────

  const render = useCallback(async () => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    ctx.clearRect(0, 0, canvas.width, canvas.height);

    // Dark pitch-green background for contrast
    ctx.fillStyle = '#2d6e2d';
    ctx.fillRect(0, 0, canvas.width, canvas.height);

    ctx.imageSmoothingEnabled = false;

    const displayFrame = animating ? animFrame : frame;

    for (const layer of layers) {
      if (!visible[layer]) continue;

      const src = spritePath(type, layer, displayFrame);
      const img = await loadImg(src);
      if (!img) continue;

      let source: HTMLImageElement | HTMLCanvasElement = img;

      // Apply skin tint to hair/skin layer
      if (layer === 'hair') {
        source = tintSkin(img, skinTone);
      }

      // Apply team colour swap for away team
      if (team === 'away' && (layer === 'shirt' || layer === 'waistcoat' || layer === 'background')) {
        source = swapTeamColor(source instanceof HTMLImageElement ? source : img);
      }

      ctx.drawImage(source, 0, 0, canvas.width, canvas.height);
    }

    // Frame label
    ctx.fillStyle = 'rgba(0,0,0,0.6)';
    ctx.fillRect(0, canvas.height - 20, canvas.width, 20);
    ctx.fillStyle = '#e8d5a0';
    ctx.font = '11px "Courier New"';
    ctx.textAlign = 'center';
    ctx.fillText(`frame ${displayFrame}`, canvas.width / 2, canvas.height - 6);
  }, [frame, animFrame, animating, type, layers, visible, skinTone, team]);

  useEffect(() => { render(); }, [render]);

  // ── Animation playback ───────────────────────────────────────────────────────

  useEffect(() => {
    if (animRef.current) clearInterval(animRef.current);
    if (animating) {
      // Animate the current direction group (first 8 frames of dir 0 by default)
      const poseGroup = POSE_GROUPS.find(g => frame >= g.frames[0] && frame <= g.frames[1])
        ?? POSE_GROUPS[0];
      let f = poseGroup.frames[0];
      animRef.current = setInterval(() => {
        setAnimFrame(f);
        f = f >= poseGroup.frames[1] ? poseGroup.frames[0] : f + 1;
      }, 100);
    }
    return () => { if (animRef.current) clearInterval(animRef.current); };
  }, [animating, frame]);

  // ── Download current composite ───────────────────────────────────────────────

  function downloadFrame() {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const url = canvas.toDataURL('image/png');
    const a = document.createElement('a');
    a.href = url;
    a.download = `victorian_${type}_frame${frame}_${team}_skin${skinTone}.png`;
    a.click();
  }

  // ── Pose group navigation ────────────────────────────────────────────────────

  function jumpToPose(groupIdx: number) {
    const g = POSE_GROUPS[groupIdx];
    if (!g) return;
    const f = Math.min(g.frames[1], g.frames[0]);
    setFrame(f);
  }

  // ── Render ───────────────────────────────────────────────────────────────────

  return (
    <div style={styles.root}>
      {/* Header */}
      <div style={styles.header}>
        <button style={styles.backBtn} onClick={onBack}>← Back</button>
        <h1 style={styles.title}>Sprite Editor — Victorian 1867</h1>
        <span style={styles.packLabel}>Pack: {PACK}</span>
      </div>

      <div style={styles.body}>

        {/* Left panel: controls */}
        <div style={styles.leftPanel}>

          {/* Type selector */}
          <section style={styles.section}>
            <h3 style={styles.sectionTitle}>Type</h3>
            <div style={styles.btnGroup}>
              {(['player', 'goalkeeper'] as const).map(t => (
                <button
                  key={t}
                  style={{ ...styles.toggleBtn, ...(type === t ? styles.toggleBtnActive : {}) }}
                  onClick={() => { setType(t); setFrame(0); }}
                >
                  {t === 'player' ? 'Outfield' : 'Goalkeeper'}
                </button>
              ))}
            </div>
          </section>

          {/* Team colour */}
          <section style={styles.section}>
            <h3 style={styles.sectionTitle}>Team Colour</h3>
            <div style={styles.btnGroup}>
              {(['home', 'away'] as const).map(t => (
                <button
                  key={t}
                  style={{ ...styles.toggleBtn, ...(team === t ? styles.toggleBtnActive : {}) }}
                  onClick={() => setTeam(t)}
                >
                  {t.charAt(0).toUpperCase() + t.slice(1)}
                </button>
              ))}
            </div>
          </section>

          {/* Skin tone */}
          <section style={styles.section}>
            <h3 style={styles.sectionTitle}>Skin Tone</h3>
            <div style={styles.btnGroup}>
              {SKIN_TINTS.map((tint, i) => (
                <button
                  key={i}
                  style={{
                    ...styles.skinBtn,
                    backgroundColor: `rgb(${tint.join(',')})`,
                    border: skinTone === i ? '2px solid #e8d5a0' : '2px solid #555',
                  }}
                  onClick={() => setSkinTone(i)}
                  title={`Skin tone ${i + 1}`}
                />
              ))}
            </div>
          </section>

          {/* Layer visibility */}
          <section style={styles.section}>
            <h3 style={styles.sectionTitle}>Layers</h3>
            {layers.map(layer => (
              <label key={layer} style={styles.layerRow}>
                <input
                  type="checkbox"
                  checked={visible[layer]}
                  onChange={e => setVisible(v => ({ ...v, [layer]: e.target.checked }))}
                  style={styles.checkbox}
                />
                <span style={{ color: visible[layer] ? '#e8d5a0' : '#666' }}>
                  {LAYER_LABELS[layer]}
                </span>
              </label>
            ))}
          </section>

          {/* Pose groups */}
          <section style={styles.section}>
            <h3 style={styles.sectionTitle}>Jump to Pose</h3>
            <div style={styles.poseList}>
              {POSE_GROUPS.filter(g => g.frames[0] <= maxFrame).map((g, i) => (
                <button
                  key={i}
                  style={{
                    ...styles.poseBtn,
                    ...(frame >= g.frames[0] && frame <= g.frames[1] ? styles.poseBtnActive : {}),
                  }}
                  onClick={() => jumpToPose(i)}
                >
                  {g.label}
                </button>
              ))}
            </div>
          </section>

          {/* Actions */}
          <section style={styles.section}>
            <h3 style={styles.sectionTitle}>Actions</h3>
            <button
              style={{ ...styles.actionBtn, ...(animating ? styles.actionBtnActive : {}) }}
              onClick={() => setAnimating(a => !a)}
            >
              {animating ? '⏸ Stop' : '▶ Animate Pose'}
            </button>
            <button style={styles.actionBtn} onClick={downloadFrame}>
              ↓ Download Frame
            </button>
          </section>
        </div>

        {/* Centre: canvas preview */}
        <div style={styles.centrePanel}>
          <canvas
            ref={canvasRef}
            width={W * DISPLAY_SCALE}
            height={H * DISPLAY_SCALE}
            style={styles.canvas}
          />

          {/* Frame scrubber */}
          <div style={styles.scrubber}>
            <button
              style={styles.scrubBtn}
              onClick={() => setFrame(f => Math.max(0, f - 1))}
              disabled={animating}
            >◀</button>
            <input
              type="range"
              min={0}
              max={maxFrame}
              value={animating ? animFrame : frame}
              onChange={e => { setFrame(Number(e.target.value)); setAnimating(false); }}
              style={styles.slider}
            />
            <button
              style={styles.scrubBtn}
              onClick={() => setFrame(f => Math.min(maxFrame, f + 1))}
              disabled={animating}
            >▶</button>
            <span style={styles.frameLabel}>
              {animating ? animFrame : frame} / {maxFrame}
            </span>
          </div>
        </div>

        {/* Right panel: frame strip */}
        <div style={styles.rightPanel}>
          <h3 style={styles.sectionTitle}>Frames</h3>
          <div style={styles.frameStrip}>
            {Array.from({ length: maxFrame + 1 }, (_, i) => (
              <FrameThumb
                key={i}
                frameIdx={i}
                type={type}
                layer="hair"
                active={(!animating && frame === i) || (animating && animFrame === i)}
                onClick={() => { setFrame(i); setAnimating(false); }}
              />
            ))}
          </div>
        </div>
      </div>
    </div>
  );
}

// ─── Frame thumbnail ──────────────────────────────────────────────────────────

interface ThumbProps {
  frameIdx: number;
  type: 'player' | 'goalkeeper';
  layer: AnyLayer;
  active: boolean;
  onClick: () => void;
}

function FrameThumb({ frameIdx, type, layer, active, onClick }: ThumbProps) {
  const canvasRef = useRef<HTMLCanvasElement>(null);

  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;
    const ctx = canvas.getContext('2d');
    if (!ctx) return;
    ctx.clearRect(0, 0, canvas.width, canvas.height);
    ctx.imageSmoothingEnabled = false;

    const src = spritePath(type, layer, frameIdx);
    const img = new Image();
    img.onload = () => ctx.drawImage(img, 0, 0, canvas.width, canvas.height);
    img.src = src;
  }, [frameIdx, type, layer]);

  return (
    <canvas
      ref={canvasRef}
      width={W}
      height={H}
      onClick={onClick}
      style={{
        width: 36,
        height: 45,
        cursor: 'pointer',
        border: active ? '2px solid #e8d5a0' : '1px solid #333',
        imageRendering: 'pixelated',
        flexShrink: 0,
        display: 'block',
      }}
      title={`Frame ${frameIdx}`}
    />
  );
}

// ─── Styles ───────────────────────────────────────────────────────────────────

const styles: Record<string, React.CSSProperties> = {
  root: {
    display: 'flex',
    flexDirection: 'column',
    height: '100vh',
    background: '#0f1208',
    color: '#e8d5a0',
    fontFamily: 'Georgia, serif',
    overflow: 'hidden',
  },
  header: {
    display: 'flex',
    alignItems: 'center',
    gap: 16,
    padding: '10px 20px',
    background: '#1a1208',
    borderBottom: '1px solid #3d2b0e',
    flexShrink: 0,
  },
  backBtn: {
    background: 'transparent',
    border: '1px solid #3d2b0e',
    color: '#e8d5a0',
    padding: '4px 12px',
    cursor: 'pointer',
    fontFamily: 'Georgia, serif',
    fontSize: 13,
  },
  title: {
    margin: 0,
    fontSize: 18,
    fontWeight: 'bold',
    color: '#e8d5a0',
    flex: 1,
  },
  packLabel: {
    fontSize: 12,
    color: '#8b6914',
    fontFamily: 'Courier New, monospace',
  },
  body: {
    display: 'flex',
    flex: 1,
    overflow: 'hidden',
  },
  leftPanel: {
    width: 220,
    flexShrink: 0,
    overflowY: 'auto',
    padding: '12px 14px',
    background: '#12100a',
    borderRight: '1px solid #3d2b0e',
  },
  centrePanel: {
    flex: 1,
    display: 'flex',
    flexDirection: 'column',
    alignItems: 'center',
    justifyContent: 'center',
    padding: 24,
    gap: 16,
  },
  rightPanel: {
    width: 80,
    flexShrink: 0,
    overflowY: 'auto',
    padding: '12px 8px',
    background: '#12100a',
    borderLeft: '1px solid #3d2b0e',
  },
  section: {
    marginBottom: 20,
  },
  sectionTitle: {
    margin: '0 0 8px 0',
    fontSize: 11,
    textTransform: 'uppercase',
    letterSpacing: '0.1em',
    color: '#8b6914',
    fontFamily: 'Courier New, monospace',
  },
  btnGroup: {
    display: 'flex',
    gap: 6,
    flexWrap: 'wrap',
  },
  toggleBtn: {
    background: '#1a1208',
    border: '1px solid #3d2b0e',
    color: '#8b6914',
    padding: '4px 10px',
    cursor: 'pointer',
    fontFamily: 'Georgia, serif',
    fontSize: 12,
  },
  toggleBtnActive: {
    background: '#3d2b0e',
    color: '#e8d5a0',
    border: '1px solid #8b6914',
  },
  skinBtn: {
    width: 28,
    height: 28,
    borderRadius: '50%',
    cursor: 'pointer',
  },
  layerRow: {
    display: 'flex',
    alignItems: 'center',
    gap: 8,
    marginBottom: 6,
    cursor: 'pointer',
    fontSize: 13,
  },
  checkbox: {
    accentColor: '#8b6914',
    width: 14,
    height: 14,
  },
  poseList: {
    display: 'flex',
    flexDirection: 'column',
    gap: 4,
  },
  poseBtn: {
    background: 'transparent',
    border: '1px solid #3d2b0e',
    color: '#8b6914',
    padding: '3px 6px',
    cursor: 'pointer',
    fontFamily: 'Courier New, monospace',
    fontSize: 11,
    textAlign: 'left',
  },
  poseBtnActive: {
    background: '#3d2b0e',
    color: '#e8d5a0',
    border: '1px solid #8b6914',
  },
  actionBtn: {
    display: 'block',
    width: '100%',
    background: '#1a1208',
    border: '1px solid #3d2b0e',
    color: '#8b6914',
    padding: '6px 10px',
    cursor: 'pointer',
    fontFamily: 'Georgia, serif',
    fontSize: 12,
    marginBottom: 6,
    textAlign: 'left',
  },
  actionBtnActive: {
    background: '#3d2b0e',
    color: '#e8d5a0',
    border: '1px solid #8b6914',
  },
  canvas: {
    imageRendering: 'pixelated',
    border: '2px solid #3d2b0e',
    boxShadow: '0 0 40px rgba(139,105,20,0.15)',
  },
  scrubber: {
    display: 'flex',
    alignItems: 'center',
    gap: 10,
    width: W * DISPLAY_SCALE,
  },
  scrubBtn: {
    background: '#1a1208',
    border: '1px solid #3d2b0e',
    color: '#e8d5a0',
    width: 28,
    height: 28,
    cursor: 'pointer',
    fontFamily: 'monospace',
    flexShrink: 0,
  },
  slider: {
    flex: 1,
    accentColor: '#8b6914',
  },
  frameLabel: {
    fontFamily: 'Courier New, monospace',
    fontSize: 12,
    color: '#8b6914',
    minWidth: 60,
    textAlign: 'right',
    flexShrink: 0,
  },
  frameStrip: {
    display: 'flex',
    flexDirection: 'column',
    gap: 3,
    alignItems: 'center',
  },
};
