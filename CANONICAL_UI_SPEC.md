# Canonical Fixed-Resolution UI Specification

## Overview

The entire UI is rendered at a **fixed virtual resolution (1280×800)** and uniformly scaled to fit the window. The window acts like a camera zoom — resizing NEVER causes layout reflow.

## Key Principles

✅ **Single Scale Factor** — Only the `scale` value changes when the window is resized
✅ **No Layout Reflow** — The internal layout is fixed at 1280×800 forever
✅ **Letterboxing** — Empty space appears as gray letterbox (not a bug)
✅ **Pixel-Perfect Mode** — Integer scaling only (1×, 2×, 3×) for sharp retro look
✅ **Window Never Scrolls** — Only internal panels scroll
✅ **No Media Queries** — Zero responsive CSS

## Implementation Architecture

### HTML Structure

```html
<body>
  <div id="viewport">           <!-- Center and letterbox container -->
    <div id="ui-root">          <!-- Fixed 1280×800 canvas, scaled by CSS transform -->
      <div id="root">           <!-- React app mounts here -->
        <!-- App content -->
      </div>
    </div>
  </div>
</body>
```

### CSS Foundation (index.css)

```css
html, body, #root {
  width: 100%;
  height: 100%;
  margin: 0;
  overflow: hidden;            /* Window never scrolls */
}

#viewport {
  width: 100%;
  height: 100%;
  display: flex;
  align-items: center;
  justify-content: center;
  background: #eaeaea;         /* Letterbox color */
  overflow: hidden;
}

#ui-root {
  width: 1280px;
  height: 800px;
  transform-origin: top left;
  will-change: transform;
  background: white;
}

#ui-root.pixel-perfect {
  image-rendering: pixelated;
  image-rendering: crisp-edges;
}
```

### React Scaling Logic (App.tsx)

```typescript
const BASE_WIDTH = 1280
const BASE_HEIGHT = 800

useEffect(() => {
  const scaleUi = () => {
    const scaleX = window.innerWidth / BASE_WIDTH
    const scaleY = window.innerHeight / BASE_HEIGHT

    let nextScale = Math.min(scaleX, scaleY)

    // Pixel-perfect mode: integer scaling only
    if (pixelPerfect) {
      nextScale = Math.floor(nextScale)
      if (nextScale < 1) nextScale = 1
    }

    setScale(nextScale)

    const ui = document.getElementById('ui-root')
    if (ui) {
      ui.style.transform = `scale(${nextScale})`
    }
  }

  scaleUi()
  window.addEventListener('resize', scaleUi)
  return () => window.removeEventListener('resize', scaleUi)
}, [pixelPerfect])
```

## Scaling Behavior

| Window Size | Scale | Result |
|---|---|---|
| 1280×800 | 1.0× | No scaling, pixel-perfect fit |
| 1920×1080 | 1.0× | Letterboxed (leaves empty space) |
| 2560×1600 | 1.6× | Would be 1.6×, or 1× in pixel-perfect mode |
| 640×480 | 0.64× | Clamped to 1.0× (never smaller) |

> **Why letterboxing?** Preserving aspect ratio ensures UI is never stretched/distorted. The letterbox is the correct behavior.

## Internal Layout (Fixed)

The 1280×800 canvas contains:

```
┌─────────────────────────────────────────────┐
│  Sidebar (200px)  │  Main Content (1080px) │  Height: 800px
│                   │                        │
│   • Navigation    │   Dashboard/Matchday/  │
│   • Game Info     │   Squad/Standings      │
│   • Next Day Btn  │                        │
│   • Toolbar       │                        │
└─────────────────────────────────────────────┘
```

### Scrolling Rules

- **Sidebar** — Can scroll vertically if content overflows
- **Main Content** — Can scroll vertically if content overflows
- **Window** — NEVER scrolls (overflow: hidden on body)

## Debug Overlay

When running in development, a debug overlay appears in the bottom-right corner:

```
Scale: 1.50×
DPR: 1.00
Pixel-perfect: OFF

[Toggle Pixel-Perfect]
```

**Remove for production:** Delete the debug overlay from App.tsx return statement.

## Windows DPI Awareness (Tauri)

**REQUIRED** for proper scaling. In `tauri.conf.json`:

```json
{
  "tauri": {
    "bundle": {
      "windows": {
        "dpiAware": true
      }
    }
  }
}
```

Without this, Windows silently scales the app, causing blur and inconsistent behavior.

## Pixel-Perfect Mode

### When Enabled

- Scale only in integer multiples (1×, 2×, 3×)
- Never fractional (1.5× becomes 1×)
- Images render crisp (no interpolation)
- Looks like retro game emulator

### When Disabled

- Smooth scaling (fractional allowed)
- Slightly blurry on non-integer scales
- Better for ultrawide or unusual aspect ratios

### Toggle

Click "Toggle Pixel-Perfect" button in debug overlay.

## Migration Checklist

- ✅ `#viewport` wrapper added to HTML
- ✅ `#ui-root` with fixed 1280×800 dimensions
- ✅ Global `overflow: hidden` on html/body
- ✅ Scaling logic in App.tsx useEffect
- ✅ State: `scale` and `pixelPerfect`
- ✅ CSS: `transform: scale(...)` applied to `#ui-root`
- ✅ Debug overlay rendered
- ✅ Pixel-perfect mode toggle functional

## One-Sentence Summary

**"The app renders a fixed 1280×800 UI and scales it uniformly like a game framebuffer; resizing the window only zooms the interface and never alters layout."**

---

## Related Files

- **App.tsx** — Scaling logic, debug overlay, pixel-perfect toggle
- **index.css** — Global styles, viewport/ui-root definitions
- **index.html** — HTML structure with viewport/ui-root wrappers
- **tauri.conf.json** — `dpiAware: true` for Windows

---

## Questions to Ask When Debugging

1. **Is the window scrolling?** → Check `overflow: hidden` on body/html
2. **Is layout reflowing on resize?** → Check that internal layout CSS uses fixed `px`, not `%`
3. **Is the UI blurry?** → Check `dpiAware: true` in Tauri config
4. **Does integer scaling work?** → Check `Math.floor()` logic in pixelPerfect mode
5. **Is the debug overlay visible?** → It should appear in bottom-right corner always

---

Version: 1.0
Created: 2026-01-29
Status: ✅ Implementation Complete
