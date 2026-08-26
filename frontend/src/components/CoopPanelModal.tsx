import React, { useEffect, useState } from 'react'
import { invoke } from '../utils/tauriInvoke'
import '../styles/FunctionalModal.css'

interface CoopStatus {
  founded: boolean
  membership: number | null
  dividendRatePct: number | null
}

const GOODS: { key: string; label: string; shillings: number }[] = [
  { key: 'kit', label: 'A set of kit', shillings: 25 },
  { key: 'ball', label: 'A leather football', shillings: 12 },
  { key: 'goalposts', label: 'Goal posts', shillings: 60 },
  { key: 'shin_guards', label: 'Shin guards', shillings: 6 },
  { key: 'hall', label: 'The meeting hall', shillings: 10 },
  { key: 'refreshments', label: 'Refreshments', shillings: 8 },
]

interface CoopPanelModalProps {
  gameState: any
  onClose: () => void
}

// The Co-operative Society: join, buy goods at a fair price, and earn the divi
// back each season on what you buy.
export function CoopPanelModal({ gameState, onClose }: CoopPanelModalProps) {
  const [status, setStatus] = useState<CoopStatus | null>(null)
  const [error, setError] = useState<string | null>(null)
  const [lastBought, setLastBought] = useState<string | null>(null)
  const [busy, setBusy] = useState(false)

  const load = async () => {
    try {
      setStatus(await invoke<CoopStatus>('coop_status', { game: gameState }))
    } catch (e) {
      setError(String(e))
    }
  }

  useEffect(() => { load() }, [gameState])

  const buy = async (item: string, label: string) => {
    setBusy(true)
    setLastBought(null)
    try {
      const res = await invoke<{ boughtShillings: number }>('coop_buy', { game: gameState, item })
      setLastBought(`Bought ${label.toLowerCase()} for ${res.boughtShillings}s.`)
    } catch (e) {
      setError(String(e))
    } finally {
      setBusy(false)
    }
  }

  return (
    <div className="functional-modal-overlay" onClick={onClose}>
      <div className="functional-modal" onClick={(e) => e.stopPropagation()}>
        <div className="functional-modal-header">
          <h2>The Co-operative Society</h2>
          <button className="functional-modal-close" onClick={onClose}>✕</button>
        </div>
        <div className="functional-modal-body">
          {error && <p className="functional-modal-error">{error}</p>}
          {!status ? (
            <p>Reading the ledger…</p>
          ) : !status.founded ? (
            <p>The Society has not yet been founded — that comes in 1868.</p>
          ) : (
            <>
              <p>
                {status.membership?.toLocaleString()} members, paying a divi of {status.dividendRatePct?.toFixed(1)}%.
              </p>
              <div>
                {GOODS.map((g) => (
                  <div key={g.key} className="functional-modal-row">
                    <span>{g.label} — {g.shillings}s</span>
                    <button className="functional-modal-btn" disabled={busy} onClick={() => buy(g.key, g.label)}>
                      Buy
                    </button>
                  </div>
                ))}
              </div>
              {lastBought && <p style={{ marginTop: 10 }}>{lastBought}</p>}
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
