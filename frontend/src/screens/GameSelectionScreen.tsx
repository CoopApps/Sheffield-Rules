import React from 'react'
import '../styles/game-selection.css'

// SVG Icons
const TrophyIcon = () => <svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor"><path d="M6 9c0-1 1-2 2-2h8c1 0 2 1 2 2v3H6V9zm8-5c0-1-1-2-2-2s-2 1-2 2v1h4V4zm6 4c1 0 2 1 2 2v8c0 1-1 2-2 2h-1v2h-2v-2H9v2H7v-2H6c-1 0-2-1-2-2v-8c0-1 1-2 2-2h1V4c0-1 1-2 2-2s2 1 2 2v1h4V4c0-1 1-2 2-2s2 1 2 2v1h1z"/></svg>
const CoinIcon = () => <svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor"><circle cx="12" cy="12" r="10" stroke="currentColor" strokeWidth="2" fill="none"/><text x="12" y="15" textAnchor="middle" fontSize="10" fill="currentColor">$</text></svg>
const ChartIcon = () => <svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor"><path d="M5 9.2h3v9.6H5zm5.6 0h3v9.6h-3zm5.6 0h3v9.6h-3z"/><path d="M5 19h14" stroke="currentColor" fill="none" strokeWidth="2"/></svg>
const PeopleIcon = () => <svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor"><circle cx="9" cy="8" r="4"/><path d="M9 14c-4.418 0-8 1.79-8 4v2h16v-2c0-2.21-3.582-4-8-4z"/><circle cx="17" cy="9" r="3"/><path d="M17 15c-2.757 0-5 1.343-5 3v1h10v-1c0-1.657-2.243-3-5-3z"/></svg>
const CalendarIcon = () => <svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor"><rect x="3" y="4" width="18" height="16" rx="2" stroke="currentColor" fill="none" strokeWidth="2"/><path d="M3 10h18M9 3v4M15 3v4" stroke="currentColor" strokeWidth="2" fill="none"/></svg>
const BallIcon = () => <svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor"><circle cx="12" cy="12" r="10" stroke="currentColor" fill="none" strokeWidth="2"/><path d="M12 2v3M12 19v3M22 12h-3M5 12H2M8.5 3.5l2 2M13.5 18.5l2 2M15.5 3.5l-2 2M10.5 18.5l-2 2" stroke="currentColor" strokeWidth="1.5" fill="none"/></svg>
const TheaterIcon = () => <svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor"><path d="M18 3v2h1v14h-1v2h2V3h-2zM4 3H2v18h2v-2h1V5H4V3zm-1 4h1v10H3V7z"/><circle cx="12" cy="12" r="2" fill="currentColor"/></svg>
const TrendIcon = () => <svg viewBox="0 0 24 24" width="20" height="20" fill="currentColor"><polyline points="3 21 9 13 14 19 21 10" stroke="currentColor" strokeWidth="2" fill="none"/><polyline points="21 10 21 3 14 3" stroke="currentColor" strokeWidth="2" fill="none"/></svg>

interface GameSelectionScreenProps {
  onSelectGame: (game: 'sheffield-rules' | 'saturday-at-three' | 'database-editor' | 'quick-match' | 'sprite-editor') => void
}

// Everything below is kept, never deleted — just moved out of the primary
// launcher so "Sheffield Rules" (the shipping product) stands alone. These
// were earlier experiments (a SWOS-style matchday attempt, its sprite work)
// and dev tooling; toggle "Experiments & Tools" to reach them.
export function GameSelectionScreen({ onSelectGame }: GameSelectionScreenProps) {
  const [showExtras, setShowExtras] = React.useState(false)

  return (
    <div className="game-selection-screen">
      <div className="game-selection-container">
        <h1 className="game-title">Saturday at Three</h1>
        <p className="game-subtitle">Sheffield, 1867 — take the secretary's chair</p>

        <div className="game-grid game-grid-primary">
          {/* Sheffield Rules — the shipping product */}
          <div className="game-card sheffield-rules" onClick={() => onSelectGame('sheffield-rules')}>
            <div className="game-card-header">
              <h2>Sheffield Rules</h2>
              <span className="game-version">1867</span>
            </div>
            <div className="game-card-content">
              <p className="game-description">
                Manage a Sheffield club through the amateur era — rouges, the annual
                assembly, the census-drawn men of the town, and the letters that arrive
                each morning.
              </p>
              <div className="game-features">
                <div className="feature">
                  <span className="feature-icon">🏆</span>
                  <span>The Sheffield &amp; Hallamshire pyramid, 186 clubs</span>
                </div>
                <div className="feature">
                  <span className="feature-icon">📜</span>
                  <span>The laws evolve — rouges, the assembly, the FA</span>
                </div>
                <div className="feature">
                  <span className="feature-icon">👥</span>
                  <span>Real Sheffield census men, trades and households</span>
                </div>
                <div className="feature">
                  <span className="feature-icon">💌</span>
                  <span>A living post — challenges, results, the committee</span>
                </div>
              </div>
            </div>
            <button className="play-button">Play</button>
          </div>

          {/* A single door to everything else — not deleted, just not the front door. */}
          <div className="game-card experiments-toggle" onClick={() => setShowExtras(v => !v)}>
            <div className="game-card-header">
              <h2>Experiments &amp; Tools</h2>
              <span className="game-version">{showExtras ? 'Hide' : 'Show'}</span>
            </div>
            <div className="game-card-content">
              <p className="game-description">
                Earlier prototypes and development tools — a SWOS-style matchday
                attempt, its sprite work, and the database editor. Not part of the
                shipping game.
              </p>
            </div>
            <button className="play-button">{showExtras ? 'Hide' : 'Open'}</button>
          </div>
        </div>

        {showExtras && (
        <div className="game-grid game-grid-extras">
          {/* Saturday at Three - Demo */}
          <div className="game-card saturday-at-three" onClick={() => onSelectGame('saturday-at-three')}>
            <div className="game-card-header">
              <h2>Saturday at Three</h2>
              <span className="game-version">Demo - 1888-89</span>
            </div>
            <div className="game-card-content">
              <p className="game-description">
                Experience the inaugural 1888-89 Football League season in Britain.
              </p>
              <div className="game-features">
                <div className="feature">
                  <span className="feature-icon"><CalendarIcon /></span>
                  <span>Full 1888-89 season (22 rounds)</span>
                </div>
                <div className="feature">
                  <span className="feature-icon"><BallIcon /></span>
                  <span>12 founding Football League clubs</span>
                </div>
                <div className="feature">
                  <span className="feature-icon"><TheaterIcon /></span>
                  <span>Period-appropriate commentary</span>
                </div>
                <div className="feature">
                  <span className="feature-icon"><TrendIcon /></span>
                  <span>Real historical players & data</span>
                </div>
              </div>
            </div>
            <button className="play-button">Play Demo</button>
          </div>

          {/* Quick Match */}
          <div className="game-card quick-match" onClick={() => onSelectGame('quick-match')}>
            <div className="game-card-header">
              <h2>Quick Match</h2>
              <span className="game-version">Sheffield Rules 1867</span>
            </div>
            <div className="game-card-content">
              <p className="game-description">
                Sheffield FC vs Hallam FC — October 1867. The world's oldest football derby, played under the original Sheffield Rules.
              </p>
              <div className="game-features">
                <div className="feature">
                  <span className="feature-icon">⚽</span>
                  <span>Sheffield FC vs Hallam FC</span>
                </div>
                <div className="feature">
                  <span className="feature-icon">📜</span>
                  <span>1867 Sheffield Rules (rouge flags)</span>
                </div>
                <div className="feature">
                  <span className="feature-icon">🎮</span>
                  <span>SWOS-style match simulation</span>
                </div>
                <div className="feature">
                  <span className="feature-icon">🏟️</span>
                  <span>Historic East Bank ground</span>
                </div>
              </div>
            </div>
            <button className="play-button">Kick Off</button>
          </div>

          {/* Sprite Editor */}
          <div className="game-card database-editor" onClick={() => onSelectGame('sprite-editor')}>
            <div className="game-card-header">
              <h2>Sprite Editor</h2>
              <span className="game-version">Victorian 1867 Pack</span>
            </div>
            <div className="game-card-content">
              <p className="game-description">
                Preview and edit Victorian-era player sprites. Layer-by-layer composition with team colours and skin tone variants.
              </p>
              <div className="game-features">
                <div className="feature">
                  <span className="feature-icon">🎨</span>
                  <span>Layer-by-layer preview</span>
                </div>
                <div className="feature">
                  <span className="feature-icon">👔</span>
                  <span>106 player / 58 GK frames</span>
                </div>
                <div className="feature">
                  <span className="feature-icon">🎩</span>
                  <span>Victorian kit — flat caps & trousers</span>
                </div>
                <div className="feature">
                  <span className="feature-icon">💾</span>
                  <span>Export composited frames as PNG</span>
                </div>
              </div>
            </div>
            <button className="play-button">Open Sprite Editor</button>
          </div>

          {/* Database Editor */}
          <div className="game-card database-editor" onClick={() => onSelectGame('database-editor')}>
            <div className="game-card-header">
              <h2>Database Editor</h2>
              <span className="game-version">Development Tool</span>
            </div>
            <div className="game-card-content">
              <p className="game-description">
                Player generation with postcode filtering & multi-club assignment.
              </p>
              <div className="game-features">
                <div className="feature">
                  <span className="feature-icon"><PeopleIcon /></span>
                  <span>Custom player entry table</span>
                </div>
                <div className="feature">
                  <span className="feature-icon">📍</span>
                  <span>Postcode-based filtering</span>
                </div>
                <div className="feature">
                  <span className="feature-icon">⚽</span>
                  <span>Multi-club assignment</span>
                </div>
                <div className="feature">
                  <span className="feature-icon">📅</span>
                  <span>All Sheffield clubs & leagues</span>
                </div>
              </div>
            </div>
            <button className="play-button">Open Editor</button>
          </div>
        </div>
        )}

        <div className="game-selection-footer">
          <p>© 2026 Football Management Series</p>
        </div>
      </div>
    </div>
  )
}

export default GameSelectionScreen
