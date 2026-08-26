/**
 * Match Viewer Screen - Dual-mode match viewing (2D canvas + text commentary)
 * Supports live streaming for user matches and replay viewing for completed matches
 */

import React, { useEffect, useState } from 'react';
import { GameState, Match } from '../types/GameState';
import { useMatchStore } from '../stores/matchStore';
import { TextCommentaryView } from '../components/match/TextCommentaryView';
import { FullscreenMatchView } from '../components/match/FullscreenMatchView';
import '../styles/match-viewer.css';

interface MatchViewerScreenProps {
  gameState: GameState;
  setGameState: (state: GameState | ((prev: GameState) => GameState)) => void;
  theme: string;
}

export function MatchViewerScreen({
  gameState,
  setGameState,
  theme,
}: MatchViewerScreenProps) {
  const {
    viewMode,
    setViewMode,
    matchType,
    matchData,
    liveEvents,
    liveVisualStates,
    currentMinute,
    isPaused,
    togglePause,
    currentStatistics,
    loadReplayMatch,
    startLiveMatch,
    reset,
  } = useMatchStore();

  const [selectedMatch, setSelectedMatch] = useState<Match | null>(null);
  const [loading, setLoading] = useState(false);
  const [error, setError] = useState<string | null>(null);

  // Generate test matches if none exist
  const allMatches = gameState.matches.length > 0 ? gameState.matches : [
    {
      id: 'test-match-1',
      gameweek: 1,
      homeClubId: gameState.userClubId,
      awayClubId: 'hallam',
      homeScore: 0,
      awayScore: 0,
      date: gameState.currentDate || '1867-01-01',
      played: false,
    },
    {
      id: 'test-match-2',
      gameweek: 1,
      homeClubId: 'cromwell',
      awayClubId: 'pitsmoor',
      homeScore: 2,
      awayScore: 1,
      date: gameState.currentDate || '1867-01-01',
      played: true,
    },
  ];

  // Get available matches
  const userMatches = allMatches.filter(
    (m) =>
      m.homeClubId === gameState.userClubId ||
      m.awayClubId === gameState.userClubId
  );

  const otherMatches = allMatches.filter(
    (m) =>
      m.homeClubId !== gameState.userClubId &&
      m.awayClubId !== gameState.userClubId &&
      m.played
  );

  // Handle match selection
  const handleMatchSelect = async (match: Match) => {
    setSelectedMatch(match);
    setError(null);
    setLoading(true);

    try {
      const isUserMatch =
        match.homeClubId === gameState.userClubId ||
        match.awayClubId === gameState.userClubId;

      if (isUserMatch && !match.played) {
        // Live match - simulate and stream
        await startLiveMatch(
          match.id,
          match.homeClubId,
          match.awayClubId,
          gameState.season || 1867
        );
      } else {
        // Replay match - load from database
        await loadReplayMatch(match.id);
      }
    } catch (err) {
      console.error('Failed to load match:', err);
      setError(err instanceof Error ? err.message : 'Failed to load match');
    } finally {
      setLoading(false);
    }
  };

  // Calculate current score from events
  const getCurrentScore = () => {
    if (!selectedMatch) return { home: 0, away: 0 };

    if (matchType === 'live') {
      // For live matches, count goals from events
      let homeGoals = 0;
      let awayGoals = 0;

      liveEvents.forEach(event => {
        if (event.event_type === 'Goal') {
          if (event.team_side === 'Home') {
            homeGoals++;
          } else if (event.team_side === 'Away') {
            awayGoals++;
          }
        }
      });

      return { home: homeGoals, away: awayGoals };
    } else if (matchData) {
      // For replays, use the match data score
      return { home: matchData.home_score, away: matchData.away_score };
    }

    // Fallback to selected match scores
    return { home: selectedMatch.homeScore || 0, away: selectedMatch.awayScore || 0 };
  };

  const currentScore = getCurrentScore();

  // Get club names
  const getClubName = (clubId: string) => {
    const club = gameState.clubs.find((c) => c.id === clubId);
    if (club) return club.name;

    // Fallback test club names
    const testClubs: Record<string, string> = {
      'sheffield': 'Sheffield FC',
      'hallam': 'Hallam FC',
      'cromwell': 'Cromwell FC',
      'pitsmoor': 'Pitsmoor FC',
      'accrington': 'Accrington FC',
    };

    return testClubs[clubId] || 'Unknown Club';
  };

  return (
    <div className={`match-viewer-screen theme-${theme}`}>
      {/* Match Selector Bar */}
      <div className="match-selector-bar">
        <h2>Match Viewer</h2>

        {/* Match Selection */}
        {!selectedMatch && (
          <div className="match-selection">
            <div className="match-category">
              <h3>Your Team's Matches</h3>
              <div className="match-list">
                {userMatches.length === 0 && (
                  <p className="no-matches">No matches available</p>
                )}
                {userMatches.map((match) => (
                  <button
                    key={match.id}
                    className={`match-item ${!match.played ? 'unplayed' : ''}`}
                    onClick={() => handleMatchSelect(match)}
                  >
                    <span className="match-teams">
                      {getClubName(match.homeClubId)} vs{' '}
                      {getClubName(match.awayClubId)}
                    </span>
                    {match.played && (
                      <span className="match-score">
                        {match.homeScore} - {match.awayScore}
                      </span>
                    )}
                    {!match.played && (
                      <span className="match-status">Play Match</span>
                    )}
                  </button>
                ))}
              </div>
            </div>

            <div className="match-category">
              <h3>Other Matches (Replays)</h3>
              <div className="match-list">
                {otherMatches.length === 0 && (
                  <p className="no-matches">No completed matches available</p>
                )}
                {otherMatches.map((match) => (
                  <button
                    key={match.id}
                    className="match-item"
                    onClick={() => handleMatchSelect(match)}
                  >
                    <span className="match-teams">
                      {getClubName(match.homeClubId)} vs{' '}
                      {getClubName(match.awayClubId)}
                    </span>
                    <span className="match-score">
                      {match.homeScore} - {match.awayScore}
                    </span>
                  </button>
                ))}
              </div>
            </div>
          </div>
        )}

        {/* View Mode Toggle (only shown when match is active) */}
        {selectedMatch && !loading && !error && (
          <div className="view-controls">
            <button
              className="back-button"
              onClick={() => {
                setSelectedMatch(null);
                reset();
              }}
            >
              ← Back to Match Selection
            </button>

            <div className="view-mode-toggle">
              <button
                className={`toggle-btn ${
                  viewMode === 'fullscreen-2d' ? 'active' : ''
                }`}
                onClick={() => setViewMode('fullscreen-2d')}
              >
                2D View
              </button>
              <button
                className={`toggle-btn ${
                  viewMode === 'live-view' ? 'active' : ''
                }`}
                onClick={() => setViewMode('live-view')}
              >
                Live View
              </button>
              <button
                className={`toggle-btn ${
                  viewMode === 'text-commentary' ? 'active' : ''
                }`}
                onClick={() => setViewMode('text-commentary')}
              >
                Text Commentary
              </button>
            </div>

            <div className="match-info">
              <span className="match-type-badge">
                {matchType === 'live' ? '🔴 LIVE' : '📹 REPLAY'}
              </span>
              <span className="match-minute">
                {currentMinute}' {currentMinute >= 45 ? '(2nd Half)' : ''}
              </span>
            </div>
          </div>
        )}
      </div>

      {/* Loading State */}
      {loading && (
        <div className="match-loading">
          <div className="loading-spinner"></div>
          <p>Loading match...</p>
        </div>
      )}

      {/* Error State */}
      {error && (
        <div className="match-error">
          <p>Error: {error}</p>
          <button onClick={() => setSelectedMatch(null)}>
            Back to Selection
          </button>
        </div>
      )}

      {/* Match Viewer Content */}
      {selectedMatch && !loading && !error && (
        <div className="match-viewer-content">
          {/* 2D Canvas View */}
          {viewMode === 'fullscreen-2d' && (
            <FullscreenMatchView
              matchType={matchType}
              liveVisualStates={liveVisualStates}
              liveEvents={liveEvents}
              matchData={matchData}
              currentMinute={currentMinute}
              currentStatistics={currentStatistics}
              homeClubName={getClubName(selectedMatch.homeClubId)}
              awayClubName={getClubName(selectedMatch.awayClubId)}
              homeScore={currentScore.home}
              awayScore={currentScore.away}
            />
          )}

          {/* Live View */}
          {viewMode === 'live-view' && (
            <FullscreenMatchView
              matchType={matchType}
              liveVisualStates={liveVisualStates}
              liveEvents={liveEvents}
              matchData={matchData}
              currentMinute={currentMinute}
              currentStatistics={currentStatistics}
              homeClubName={getClubName(selectedMatch.homeClubId)}
              awayClubName={getClubName(selectedMatch.awayClubId)}
              homeScore={currentScore.home}
              awayScore={currentScore.away}
            />
          )}

          {/* Text Commentary View */}
          {viewMode === 'text-commentary' && (
            <TextCommentaryView
              matchType={matchType}
              liveEvents={liveEvents}
              matchData={matchData}
              currentStatistics={currentStatistics}
              homeClubName={getClubName(selectedMatch.homeClubId)}
              awayClubName={getClubName(selectedMatch.awayClubId)}
            />
          )}

          {/* Playback Controls */}
          <div className="playback-controls">
            <button className="control-btn" onClick={togglePause}>
              {isPaused ? '▶️ Play' : '⏸️ Pause'}
            </button>
            <div className="timeline">
              <span>Minute: {currentMinute}</span>
            </div>
          </div>
        </div>
      )}
    </div>
  );
}
