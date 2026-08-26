/**
 * LiveMatchScreen — Team selection → live match → full-time summary.
 * Uses start_live_match backend command and live_frame event stream.
 */

import React, { useEffect, useState, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { LiveMatchView } from '../components/match/LiveMatchView';

interface Club {
  id: string;
  name: string;
}

// Hardcoded classic clubs as fallback; real clubs loaded from DB
const CLASSIC_CLUBS: Club[] = [
  { id: 'sheffield_fc', name: 'Sheffield FC' },
  { id: 'hallam_fc',    name: 'Hallam FC' },
  { id: 'norton',       name: 'Norton' },
  { id: 'heeley',       name: 'Heeley' },
  { id: 'ecclesall',    name: 'Ecclesall' },
  { id: 'broomhall',    name: 'Broomhall' },
];

interface LiveMatchScreenProps {
  onBack: () => void;
}

type Phase = 'select' | 'loading' | 'playing' | 'fulltime' | 'error';

export function LiveMatchScreen({ onBack }: LiveMatchScreenProps) {
  const [phase, setPhase]           = useState<Phase>('select');
  const [clubs, setClubs]           = useState<Club[]>(CLASSIC_CLUBS);
  const [homeIdx, setHomeIdx]       = useState(0);
  const [awayIdx, setAwayIdx]       = useState(1);
  const [matchId, setMatchId]       = useState('');
  const [errorMsg, setErrorMsg]     = useState('');
  const [finalHome, setFinalHome]   = useState(0);
  const [finalAway, setFinalAway]   = useState(0);

  // Try to load clubs from DB on mount; fall back silently to CLASSIC_CLUBS
  useEffect(() => {
    invoke<{ id: string; name: string }[]>('db_get_all_clubs')
      .then(rows => {
        if (rows && rows.length > 0) {
          setClubs(rows.map(r => ({ id: String(r.id), name: r.name })));
        }
      })
      .catch(() => { /* use default CLASSIC_CLUBS */ });
  }, []);

  // Keyboard: Escape exits
  useEffect(() => {
    const handler = (e: KeyboardEvent) => { if (e.key === 'Escape') onBack(); };
    window.addEventListener('keydown', handler);
    return () => window.removeEventListener('keydown', handler);
  }, [onBack]);

  const startMatch = useCallback(async () => {
    const home = clubs[homeIdx];
    const away = clubs[awayIdx];
    if (!home || !away) return;
    if (home.id === away.id) {
      setErrorMsg('Home and away teams must be different.');
      setPhase('error');
      return;
    }

    try {
      setPhase('loading');
      const mid = await invoke<string>('start_live_match', {
        homeClubId: home.id,
        awayClubId: away.id,
      });
      setMatchId(mid);
      setPhase('playing');
    } catch (e) {
      setErrorMsg(String(e));
      setPhase('error');
    }
  }, [clubs, homeIdx, awayIdx]);

  const handleFullTime = useCallback((hs: number, as_: number) => {
    setFinalHome(hs);
    setFinalAway(as_);
    setPhase('fulltime');
  }, []);

  const homeClub = clubs[homeIdx];
  const awayClub = clubs[awayIdx];

  // ── Team selection ───────────────────────────────────────────────────────────
  if (phase === 'select') {
    return (
      <div style={styles.outer}>
        <div style={styles.card}>
          <div style={styles.title}>SELECT TEAMS</div>
          <div style={styles.teamsRow}>
            <TeamPicker
              label="HOME"
              clubs={clubs}
              selected={homeIdx}
              onChange={setHomeIdx}
              exclude={awayIdx}
            />
            <div style={styles.vsLabel}>VS</div>
            <TeamPicker
              label="AWAY"
              clubs={clubs}
              selected={awayIdx}
              onChange={setAwayIdx}
              exclude={homeIdx}
            />
          </div>
          <div style={styles.matchupLine}>
            {homeClub?.name ?? '—'} vs {awayClub?.name ?? '—'}
          </div>
          <div style={styles.btnRow}>
            <button style={styles.btnPrimary} onClick={startMatch}>
              KICK OFF
            </button>
            <button style={styles.btnSecondary} onClick={onBack}>
              BACK
            </button>
          </div>
          <div style={styles.hint}>ESC — back to menu</div>
        </div>
      </div>
    );
  }

  // ── Loading ──────────────────────────────────────────────────────────────────
  if (phase === 'loading') {
    return (
      <div style={styles.outer}>
        <div style={styles.card}>
          <div style={styles.title}>LOADING</div>
          <div style={{ color: '#aaa', fontSize: 15, marginTop: 12 }}>
            {homeClub?.name} vs {awayClub?.name}
          </div>
          <div style={{ color: '#666', fontSize: 12, marginTop: 24 }}>
            Preparing squads…
          </div>
        </div>
      </div>
    );
  }

  // ── Error ────────────────────────────────────────────────────────────────────
  if (phase === 'error') {
    return (
      <div style={styles.outer}>
        <div style={styles.card}>
          <div style={{ ...styles.title, color: '#ff4444' }}>ERROR</div>
          <div style={{ color: '#aaa', fontSize: 13, marginTop: 12, maxWidth: 420, textAlign: 'center' }}>
            {errorMsg}
          </div>
          <div style={styles.btnRow}>
            <button style={styles.btnSecondary} onClick={() => setPhase('select')}>Try Again</button>
            <button style={styles.btnSecondary} onClick={onBack}>Main Menu</button>
          </div>
        </div>
      </div>
    );
  }

  // ── Full-time overlay ────────────────────────────────────────────────────────
  if (phase === 'fulltime') {
    return (
      <div style={styles.outer}>
        <div style={styles.card}>
          <div style={{ ...styles.title, color: '#c8a000', fontSize: 32 }}>FULL TIME</div>
          <div style={{ color: '#fff', fontSize: 48, fontWeight: 'bold', margin: '12px 0' }}>
            {finalHome} – {finalAway}
          </div>
          <div style={{ color: '#aaa', fontSize: 15 }}>
            {homeClub?.name} vs {awayClub?.name}
          </div>
          <div style={styles.btnRow}>
            <button
              style={styles.btnPrimary}
              onClick={() => { setPhase('select'); }}
            >
              Play Again
            </button>
            <button style={styles.btnSecondary} onClick={onBack}>
              Main Menu
            </button>
          </div>
        </div>
      </div>
    );
  }

  // ── Playing ──────────────────────────────────────────────────────────────────
  return (
    <div style={{ position: 'fixed', inset: 0, background: '#000' }}>
      <LiveMatchView
        matchId={matchId}
        homeClubName={homeClub?.name ?? 'Home'}
        awayClubName={awayClub?.name ?? 'Away'}
        onFullTime={handleFullTime}
      />
      <button
        onClick={onBack}
        style={{
          position: 'absolute', top: 8, right: 8, zIndex: 200,
          background: 'rgba(0,0,0,0.7)', color: '#888',
          border: '1px solid #444', borderRadius: 3,
          padding: '3px 9px', fontFamily: 'monospace', fontSize: 11,
          cursor: 'pointer',
        }}
      >
        ESC / Exit
      </button>
    </div>
  );
}

// ── TeamPicker sub-component ─────────────────────────────────────────────────

interface TeamPickerProps {
  label: string;
  clubs: Club[];
  selected: number;
  onChange: (idx: number) => void;
  exclude: number;
}

function TeamPicker({ label, clubs, selected, onChange, exclude }: TeamPickerProps) {
  const prev = () => {
    let i = (selected - 1 + clubs.length) % clubs.length;
    if (i === exclude) i = (i - 1 + clubs.length) % clubs.length;
    onChange(i);
  };
  const next = () => {
    let i = (selected + 1) % clubs.length;
    if (i === exclude) i = (i + 1) % clubs.length;
    onChange(i);
  };

  return (
    <div style={styles.picker}>
      <div style={styles.pickerLabel}>{label}</div>
      <div style={styles.pickerRow}>
        <button style={styles.arrow} onClick={prev}>◀</button>
        <div style={styles.clubName}>{clubs[selected]?.name ?? '—'}</div>
        <button style={styles.arrow} onClick={next}>▶</button>
      </div>
    </div>
  );
}

// ── Styles ───────────────────────────────────────────────────────────────────

const styles: Record<string, React.CSSProperties> = {
  outer: {
    position: 'fixed', inset: 0, background: '#0a0a0a',
    display: 'flex', alignItems: 'center', justifyContent: 'center',
    fontFamily: 'monospace',
  },
  card: {
    background: '#111', border: '2px solid #333',
    padding: '40px 56px',
    display: 'flex', flexDirection: 'column', alignItems: 'center', gap: 16,
    minWidth: 560,
  },
  title: {
    color: '#ffdd00', fontSize: 22, fontWeight: 'bold', letterSpacing: 4,
  },
  teamsRow: {
    display: 'flex', alignItems: 'center', gap: 32, marginTop: 12,
  },
  vsLabel: {
    color: '#555', fontSize: 18, fontWeight: 'bold',
  },
  matchupLine: {
    color: '#888', fontSize: 13, marginTop: 4,
  },
  btnRow: {
    display: 'flex', gap: 16, marginTop: 20,
  },
  btnPrimary: {
    padding: '10px 28px', background: '#1a4a1a', color: '#88ff88',
    border: '1px solid #44aa44', borderRadius: 3, cursor: 'pointer',
    fontFamily: 'monospace', fontSize: 14, letterSpacing: 1,
  },
  btnSecondary: {
    padding: '10px 28px', background: '#222', color: '#aaa',
    border: '1px solid #444', borderRadius: 3, cursor: 'pointer',
    fontFamily: 'monospace', fontSize: 14,
  },
  hint: {
    color: '#444', fontSize: 11, marginTop: 8,
  },
  picker: {
    display: 'flex', flexDirection: 'column', alignItems: 'center', gap: 8,
  },
  pickerLabel: {
    color: '#666', fontSize: 11, letterSpacing: 2,
  },
  pickerRow: {
    display: 'flex', alignItems: 'center', gap: 10,
  },
  clubName: {
    color: '#fff', fontSize: 16, fontWeight: 'bold',
    minWidth: 160, textAlign: 'center',
  },
  arrow: {
    background: 'none', border: '1px solid #444', color: '#888',
    borderRadius: 3, padding: '4px 10px', cursor: 'pointer',
    fontFamily: 'monospace', fontSize: 14,
  },
};
