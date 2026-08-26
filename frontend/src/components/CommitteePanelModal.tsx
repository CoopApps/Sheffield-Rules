import React, { useEffect, useState } from 'react'
import { invoke } from '../utils/tauriInvoke'
import '../styles/FunctionalModal.css'

interface CommitteeStatus {
  standing: number
  verdict: string
  selectionInterference: number
  hasBenefactor: boolean
}

interface ClubIdentity {
  patronage: string
  patronageLabel: string
  constraint: string
  freeHand: boolean
  gateSharePct: number
  localAffinity: number
  secretary: { name: string; reputation: number; standing: string; honours: number; seasons: number }
  rivals: { club: string; intensity: number }[]
}

const VERDICT_WORDS: Record<string, string> = {
  Delighted: 'The committee are delighted with the club’s affairs.',
  Content: 'The committee are well content with how the club is run.',
  Watchful: 'The committee watch the season’s progress closely.',
  Restless: 'The committee grow restless; the members mutter at the meeting.',
  NoConfidence: 'The committee’s confidence has all but gone.',
}

interface CommitteePanelModalProps {
  gameState: any
  onClose: () => void
}

// Your standing with the club's members, what institution stands behind the
// side, and your own name in the wider game — read straight off the census-
// grounded governance/rivalry/secretary systems.
export function CommitteePanelModal({ gameState, onClose }: CommitteePanelModalProps) {
  const [committee, setCommittee] = useState<CommitteeStatus | null>(null)
  const [identity, setIdentity] = useState<ClubIdentity | null>(null)
  const [error, setError] = useState<string | null>(null)

  useEffect(() => {
    let cancelled = false
    async function load() {
      try {
        const [c, i] = await Promise.all([
          invoke<CommitteeStatus>('committee_status', { game: gameState }),
          invoke<ClubIdentity>('club_identity', { game: gameState }),
        ])
        if (!cancelled) { setCommittee(c); setIdentity(i) }
      } catch (e) {
        if (!cancelled) setError(String(e))
      }
    }
    load()
    return () => { cancelled = true }
  }, [gameState])

  return (
    <div className="functional-modal-overlay" onClick={onClose}>
      <div className="functional-modal" style={{ maxWidth: 560 }} onClick={(e) => e.stopPropagation()}>
        <div className="functional-modal-header">
          <h2>The Committee</h2>
          <button className="functional-modal-close" onClick={onClose}>✕</button>
        </div>
        <div className="functional-modal-body">
          {error && <p className="functional-modal-error">{error}</p>}
          {!committee || !identity ? (
            <p>Reading the minute book…</p>
          ) : (
            <>
              <span className="functional-modal-section-title">Standing</span>
              <p>{committee.standing} / 10000 — {VERDICT_WORDS[committee.verdict] ?? committee.verdict}</p>
              {committee.selectionInterference > 0 && (
                <p style={{ color: '#8a5a00' }}>
                  The committee insists on {committee.selectionInterference} man{committee.selectionInterference > 1 ? 'ren' : ''} of their own choosing on the team-sheet.
                </p>
              )}
              {committee.hasBenefactor && <p>A benefactor lends the club his patronage.</p>}

              <hr />

              <span className="functional-modal-section-title">The Club</span>
              <p>{identity.patronageLabel}</p>
              <p style={{ fontStyle: 'italic' }}>{identity.constraint}</p>
              <p>Gate kept: {identity.gateSharePct}%{identity.freeHand ? ' — a free hand in selection' : ''}</p>

              <hr />

              <span className="functional-modal-section-title">{identity.secretary.name}</span>
              <p>{identity.secretary.standing}</p>
              <p>
                {identity.secretary.reputation} reputation, {identity.secretary.honours} honour{identity.secretary.honours === 1 ? '' : 's'}, {identity.secretary.seasons} season{identity.secretary.seasons === 1 ? '' : 's'} served
              </p>

              {identity.rivals.length > 0 && (
                <>
                  <hr />
                  <span className="functional-modal-section-title">Rivals</span>
                  <ul>
                    {identity.rivals.map((r) => (
                      <li key={r.club}>{r.club} — intensity {r.intensity}</li>
                    ))}
                  </ul>
                </>
              )}
            </>
          )}
        </div>
        <div className="functional-modal-footer">
          <button className="functional-modal-btn" onClick={onClose}>Close</button>
        </div>
      </div>
    </div>
  )
}
