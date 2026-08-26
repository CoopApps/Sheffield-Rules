import React from 'react'
import { AssetImage } from './AssetImage'
import { Match } from '../types/GameState'

interface MatchPanelProps {
  match: Match
  theme: string
  backgroundAsset?: string
  homeTeamName?: string
  awayTeamName?: string
}

export const MatchPanel: React.FC<MatchPanelProps> = ({
  match,
  theme,
  backgroundAsset,
  homeTeamName = 'Home Team',
  awayTeamName = 'Away Team',
}) => {
  return (
    <div className="match-panel">
      {backgroundAsset && (
        <div className="match-background">
          <AssetImage
            pack={theme}
            category="backgrounds"
            assetId={backgroundAsset}
            alt={`${homeTeamName} vs ${awayTeamName}`}
            className="background-image"
          />
        </div>
      )}

      <div className="match-content">
        <div className="match-header">
          <div className="match-date">{new Date(match.date).toLocaleDateString()}</div>
        </div>

        <div className="match-score">
          <div className="team-section home-team">
            <h2 className="team-name">{homeTeamName}</h2>
            <div className="score">{match.homeScore ?? '-'}</div>
          </div>

          <div className="match-divider">vs</div>

          <div className="team-section away-team">
            <div className="score">{match.awayScore ?? '-'}</div>
            <h2 className="team-name">{awayTeamName}</h2>
          </div>
        </div>

        <div className="match-stats">
          <div className="stat">
            <span className="stat-label">Result:</span>
            <span className="stat-value">
              {match.homeScore !== null && match.awayScore !== null
                ? match.homeScore > match.awayScore
                  ? 'Home Win'
                  : match.homeScore < match.awayScore
                    ? 'Away Win'
                    : 'Draw'
                : 'Not played'}
            </span>
          </div>
        </div>
      </div>
    </div>
  )
}
