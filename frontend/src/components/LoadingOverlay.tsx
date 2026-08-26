import React from 'react'
import '../styles/LoadingOverlay.css'

interface LoadingOverlayProps {
  isVisible: boolean
  progress: number  // 0-100
  message: string
}

export const LoadingOverlay: React.FC<LoadingOverlayProps> = ({ isVisible, progress, message }) => {
  if (!isVisible) return null

  return (
    <div className="loading-overlay">
      <div className="loading-container">
        <div className="loading-content">
          <h2>Initializing Game</h2>
          <p className="loading-message">{message}</p>

          <div className="progress-bar-container">
            <div
              className="progress-bar-fill"
              style={{ width: `${progress}%` }}
            />
          </div>

          <div className="progress-text">
            {progress}%
          </div>
        </div>
      </div>
    </div>
  )
}
