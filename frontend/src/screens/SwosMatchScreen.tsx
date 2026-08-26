/**
 * SwosMatchScreen — Sheffield FC vs Hallam FC, 1867 Sheffield Rules.
 * Full-canvas SWOS-style match renderer with live streaming from the Rust backend.
 * The match plays out automatically; the screen is purely the visual experience.
 */

import React, { useEffect, useState, useCallback } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { LiveMatchView } from '../components/match/LiveMatchView';

interface SwosMatchScreenProps {
  onBack: () => void;
}

type Phase = 'kickoff' | 'playing' | 'fulltime' | 'error';

export function SwosMatchScreen({ onBack }: SwosMatchScreenProps) {
  const [phase, setPhase] = useState<Phase>('kickoff');
  const [errorMsg, setErrorMsg] = useState('');
  const [matchId, setMatchId] = useState<string | null>(null);
  const [finalScore, setFinalScore] = useState<{ home: number; away: number } | null>(null);

  const startMatch = useCallback(async () => {
    try {
      setPhase('playing');
      setFinalScore(null);
      const id = await invoke<string>('start_quick_match');
      setMatchId(id);
    } catch (e) {
      setPhase('error');
      setErrorMsg(String(e));
    }
  }, []);

  const handleFullTime = useCallback((homeScore: number, awayScore: number) => {
    setFinalScore({ home: homeScore, away: awayScore });
    setPhase('fulltime');
  }, []);

  // Auto-start match on mount
  useEffect(() => {
    startMatch();
  }, []);

  // Keyboard: Escape = back
  useEffect(() => {
    const handler = (e: KeyboardEvent) => {
      if (e.key === 'Escape') onBack();
    };
    window.addEventListener('keydown', handler);
    return () => window.removeEventListener('keydown', handler);
  }, [onBack]);

  if (phase === 'kickoff') {
    return (
      <div style={{
        position: 'fixed', inset: 0, background: '#111',
        display: 'flex', flexDirection: 'column', alignItems: 'center', justifyContent: 'center',
        color: '#fff', fontFamily: 'monospace',
      }}>
        <div style={{ fontSize: 28, fontWeight: 'bold', color: '#ffdd00', marginBottom: 16 }}>
          SHEFFIELD RULES FOOTBALL
        </div>
        <div style={{ fontSize: 18, marginBottom: 8 }}>Sheffield FC vs Hallam FC</div>
        <div style={{ fontSize: 13, color: '#aaa', marginBottom: 32 }}>October 1867 — East Bank, Sheffield</div>
        <div style={{ fontSize: 14, color: '#888' }}>Loading squads from database…</div>
      </div>
    );
  }

  if (phase === 'error') {
    return (
      <div style={{
        position: 'fixed', inset: 0, background: '#111',
        display: 'flex', flexDirection: 'column', alignItems: 'center', justifyContent: 'center',
        color: '#fff', fontFamily: 'monospace', gap: 16,
      }}>
        <div style={{ color: '#ff4444', fontSize: 20 }}>Failed to start match</div>
        <div style={{ color: '#aaa', fontSize: 13, maxWidth: 500, textAlign: 'center' }}>{errorMsg}</div>
        <button onClick={onBack} style={{
          marginTop: 16, padding: '8px 24px', background: '#333', color: '#fff',
          border: '1px solid #666', borderRadius: 4, cursor: 'pointer', fontFamily: 'monospace',
        }}>Back</button>
      </div>
    );
  }

  return (
    <div style={{ position: 'fixed', inset: 0, background: '#111' }}>
      {/* Full-canvas match renderer */}
      {matchId && (
        <div style={{ position: 'absolute', inset: 0 }}>
          <LiveMatchView
            matchId={matchId}
            homeClubName="Sheffield FC"
            awayClubName="Hallam FC"
            onFullTime={handleFullTime}
          />
        </div>
      )}

      {/* Back button (top-left, unobtrusive) */}
      <button
        onClick={onBack}
        style={{
          position: 'absolute', top: 8, right: 8, zIndex: 100,
          background: 'rgba(0,0,0,0.7)', color: '#aaa', border: '1px solid #444',
          borderRadius: 4, padding: '4px 10px', fontFamily: 'monospace', fontSize: 11,
          cursor: 'pointer',
        }}
      >
        ESC / Exit
      </button>

      {/* Full-time overlay */}
      {phase === 'fulltime' && (
        <div style={{
          position: 'absolute', inset: 0, background: 'rgba(0,0,0,0.6)',
          display: 'flex', flexDirection: 'column', alignItems: 'center', justifyContent: 'center',
          fontFamily: 'monospace',
        }}>
          <div style={{
            background: '#111', border: '2px solid #c8a000', padding: '32px 48px',
            display: 'flex', flexDirection: 'column', alignItems: 'center', gap: 12,
          }}>
            <div style={{ color: '#c8a000', fontSize: 28, fontWeight: 'bold' }}>FULL TIME</div>
            <div style={{ color: '#fff', fontSize: 36, fontWeight: 'bold' }}>
              {finalScore?.home ?? 0} – {finalScore?.away ?? 0}
            </div>
            <div style={{ color: '#aaa', fontSize: 14 }}>Sheffield FC vs Hallam FC</div>
            <div style={{ color: '#888', fontSize: 12, marginTop: 4 }}>Sheffield Rules, October 1867</div>
            <div style={{ display: 'flex', gap: 16, marginTop: 16 }}>
              <button
                onClick={() => { setPhase('kickoff'); startMatch(); }}
                style={{
                  padding: '8px 20px', background: '#1a4a1a', color: '#88ff88',
                  border: '1px solid #44aa44', borderRadius: 4, cursor: 'pointer',
                  fontFamily: 'monospace', fontSize: 13,
                }}
              >
                Play Again
              </button>
              <button
                onClick={onBack}
                style={{
                  padding: '8px 20px', background: '#333', color: '#fff',
                  border: '1px solid #666', borderRadius: 4, cursor: 'pointer',
                  fontFamily: 'monospace', fontSize: 13,
                }}
              >
                Main Menu
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
