import React, { useState } from 'react'
import { invoke } from '../utils/tauriInvoke'
import '../styles/FunctionalModal.css'

interface TrainingReport {
  present: string[]
  absent: string[]
  turnout: number
  invited: number
}

interface TrainingPanelModalProps {
  gameState: any
  onClose: () => void
}

// Arrange a training session for the user's club and show who turned up. Every
// man's odds are composed from his census life (trade, household, temperament),
// so this is a real read on the squad's shape, not a fixed roll.
export function TrainingPanelModal({ gameState, onClose }: TrainingPanelModalProps) {
  const [loading, setLoading] = useState(false)
  const [report, setReport] = useState<TrainingReport | null>(null)
  const [error, setError] = useState<string | null>(null)

  const arrange = async () => {
    setLoading(true)
    setError(null)
    try {
      const r = await invoke<TrainingReport>('arrange_training', { game: gameState })
      setReport(r)
    } catch (e) {
      setError(String(e))
    } finally {
      setLoading(false)
    }
  }

  return (
    <div className="functional-modal-overlay" onClick={onClose}>
      <div className="functional-modal" onClick={(e) => e.stopPropagation()}>
        <div className="functional-modal-header">
          <h2>Training</h2>
          <button className="functional-modal-close" onClick={onClose}>✕</button>
        </div>
        <div className="functional-modal-body">
          {!report && (
            <p>
              Arrange a session for this evening and see who turns up. There is no
              contract binding a man to attend — his trade, his household and his
              own temper decide it.
            </p>
          )}
          {error && <p className="functional-modal-error">{error}</p>}

          {report && (
            <>
              <p>
                <strong>{report.turnout}</strong> of {report.invited} turned out this evening.
              </p>
              {report.present.length > 0 && (
                <>
                  <span className="functional-modal-section-title">Present</span>
                  <ul>{report.present.map((n) => <li key={n}>{n}</li>)}</ul>
                </>
              )}
              {report.absent.length > 0 && (
                <>
                  <span className="functional-modal-section-title">Did not come</span>
                  <ul>{report.absent.map((n) => <li key={n}>{n}</li>)}</ul>
                </>
              )}
            </>
          )}
        </div>
        <div className="functional-modal-footer">
          {!report && (
            <button className="functional-modal-btn" onClick={arrange} disabled={loading} style={{ marginRight: 8 }}>
              {loading ? 'Sending word round…' : 'Arrange Session'}
            </button>
          )}
          <button className="functional-modal-btn" onClick={onClose}>Close</button>
        </div>
      </div>
    </div>
  )
}
