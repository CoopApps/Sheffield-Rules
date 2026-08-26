import React from 'react'
import '../styles/LetterDisplayModal.css'

interface LetterDisplayModalProps {
  letterData: {
    senderClubName: string
    senderGroundName?: string
    senderCity?: string
    senderRegion?: string
    responseText: string
    responseDate?: string
    accepted: boolean
  }
  onClose: () => void
}

export function LetterDisplayModal({ letterData, onClose }: LetterDisplayModalProps) {
  const formatDate = (dateStr?: string) => {
    if (!dateStr) return ''
    try {
      const parts = dateStr.split('-')
      if (parts.length === 3) {
        const date = new Date(parseInt(parts[0]), parseInt(parts[1]) - 1, parseInt(parts[2]))
        if (!isNaN(date.getTime())) {
          const day = date.getDate()
          const month = date.toLocaleDateString('en-GB', { month: 'long' })
          const year = date.getFullYear()

          // Victorian-era date format: "3rd day of January, 1867"
          const suffix =
            day === 1 || day === 21 || day === 31 ? 'st' :
            day === 2 || day === 22 ? 'nd' :
            day === 3 || day === 23 ? 'rd' : 'th'

          return `${day}${suffix} day of ${month}, ${year}`
        }
      }
    } catch (e) {
      console.error('Error formatting date:', dateStr, e)
    }
    return dateStr
  }

  return (
    <div className="letter-display-overlay">
      <div className="letter-display-modal">
        {/* Close button */}
        <button className="letter-close-btn" onClick={onClose}>
          ×
        </button>

        {/* Letter content */}
        <div className="letter-content">
          {/* Wax seal decoration */}
          <div className="wax-seal" style={{
            background: letterData.accepted
              ? 'radial-gradient(circle, #2ecc71 0%, #27ae60 100%)'
              : 'radial-gradient(circle, #e74c3c 0%, #c0392b 100%)'
          }}>
            <div className="seal-text">
              {letterData.accepted ? '✓' : '✗'}
            </div>
          </div>

          {/* Sender's address */}
          <div className="letter-header">
            <p className="letter-address">{letterData.senderGroundName || 'Their Ground'}</p>
            <p className="letter-address">
              {letterData.senderCity || 'Unknown'}, {letterData.senderRegion || 'England'}
            </p>
          </div>

          {/* Date */}
          <p className="letter-date">{formatDate(letterData.responseDate)}</p>

          {/* Letter body */}
          <div className="letter-body">
            <p style={{ whiteSpace: 'pre-line' }}>
              {letterData.responseText}
            </p>
          </div>

          {/* Status badge */}
          <div className={`letter-status-badge ${letterData.accepted ? 'accepted' : 'declined'}`}>
            {letterData.accepted ? 'Challenge Accepted' : 'Challenge Declined'}
          </div>
        </div>

        {/* Action buttons */}
        <div className="letter-footer">
          <button className="letter-btn primary" onClick={onClose}>
            Close Letter
          </button>
        </div>
      </div>
    </div>
  )
}
