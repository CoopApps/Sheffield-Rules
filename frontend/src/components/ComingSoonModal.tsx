import React from 'react'
import '../styles/ComingSoonModal.css'

interface ComingSoonModalProps {
  title: string
  description: string
  onClose: () => void
}

export function ComingSoonModal({ title, description, onClose }: ComingSoonModalProps) {
  return (
    <div className="modal-overlay" onClick={onClose}>
      <div className="modal-content coming-soon-modal" onClick={(e) => e.stopPropagation()}>
        <div className="coming-soon-icon">🚀</div>
        <h2>{title}</h2>
        <p>{description}</p>
        <p className="coming-soon-text">Coming Soon</p>
        <button className="btn btn-primary" onClick={onClose}>
          Got It
        </button>
      </div>
    </div>
  )
}
