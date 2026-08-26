import React, { useEffect, useState } from 'react'
import { invoke } from '../utils/tauriInvoke'

interface SponsoredCup {
  sponsorName: string
  cupName: string
  seasonProposed: number
  entrants: number
  prizeShillings: number
  minDivisionLevel: number
  maxDivisionLevel: number
  seasonsHeld: number
  stillBacking: boolean
}

interface PatronageInfo {
  patronageLabel: string
  constraint: string
}

interface SponsoredCupsPanelProps {
  gameState: any
}

// Sponsored cups the town has seen — a local man of means putting up a
// competition of his own — alongside what stands behind the user's own club.
export function SponsoredCupsPanel({ gameState }: SponsoredCupsPanelProps) {
  const [cups, setCups] = useState<SponsoredCup[]>([])
  const [patronage, setPatronage] = useState<PatronageInfo | null>(null)

  useEffect(() => {
    let cancelled = false
    async function load() {
      try {
        const [c, p] = await Promise.all([
          invoke<SponsoredCup[]>('sponsored_cups', { game: gameState }),
          invoke<PatronageInfo>('club_identity', { game: gameState }),
        ])
        if (!cancelled) { setCups(c); setPatronage(p) }
      } catch {
        // Quietly absent if the save has none yet — not an error state worth surfacing here.
      }
    }
    load()
    return () => { cancelled = true }
  }, [gameState])

  if (cups.length === 0 && !patronage) return null

  return (
    <div style={{ padding: '10px 14px', borderBottom: '1px solid rgba(255,255,255,0.12)', fontSize: '0.85em' }}>
      {patronage && (
        <div style={{ marginBottom: cups.length > 0 ? 8 : 0 }}>
          <strong>{patronage.patronageLabel}</strong> — <span style={{ opacity: 0.8 }}>{patronage.constraint}</span>
        </div>
      )}
      {cups.length > 0 && (
        <div>
          {cups.map((c) => (
            <div key={c.cupName + c.seasonProposed} style={{ opacity: c.stillBacking ? 1 : 0.5, marginBottom: 2 }}>
              {c.cupName} ({c.seasonProposed}) — {c.entrants} clubs, {c.prizeShillings}s purse
              {!c.stillBacking && ' — lapsed'}
            </div>
          ))}
        </div>
      )}
    </div>
  )
}
