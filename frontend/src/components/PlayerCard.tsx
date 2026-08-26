import React from 'react'
import { AssetImage } from './AssetImage'

interface PlayerCardProps {
  playerId: string
  playerName: string
  position: string
  rating: number
  theme: string
  portraitAsset?: string
}

export const PlayerCard: React.FC<PlayerCardProps> = ({
  playerId,
  playerName,
  position,
  rating,
  theme,
  portraitAsset,
}) => {
  return (
    <div className="player-card">
      {portraitAsset && (
        <AssetImage
          pack={theme}
          category="player_portraits"
          assetId={portraitAsset}
          alt={playerName}
          className="player-portrait"
        />
      )}
      <div className="player-info">
        <h3 className="player-name">{playerName}</h3>
        <p className="player-position">{position}</p>
        <div className="player-rating">
          <span className="rating-label">Rating:</span>
          <span className="rating-value">{rating.toFixed(1)}</span>
        </div>
      </div>
    </div>
  )
}
