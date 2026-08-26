/**
 * Text Commentary View - Newspaper-style match commentary with auto-scroll
 * Displays match events in a scrolling feed with historical styling
 */

import React, { useEffect, useRef, useState } from 'react';
import { MatchEvent, MatchStatistics, MatchResult } from '../../stores/matchStore';
import './TextCommentaryView.css';

interface TextCommentaryViewProps {
  matchType: 'live' | 'replay';
  liveEvents: MatchEvent[];
  matchData: MatchResult | null;
  currentStatistics: MatchStatistics | null;
  homeClubName: string;
  awayClubName: string;
}

export function TextCommentaryView({
  matchType,
  liveEvents,
  matchData,
  currentStatistics,
  homeClubName,
  awayClubName,
}: TextCommentaryViewProps) {
  const feedRef = useRef<HTMLDivElement>(null);
  const [autoScroll, setAutoScroll] = useState(true);
  const [userScrolled, setUserScrolled] = useState(false);

  // Get events based on match type
  const events = matchType === 'live' ? liveEvents : (matchData?.events || []);

  // Auto-scroll to bottom when new events arrive
  useEffect(() => {
    if (autoScroll && feedRef.current && !userScrolled) {
      feedRef.current.scrollTop = feedRef.current.scrollHeight;
    }
  }, [events.length, autoScroll, userScrolled]);

  // Detect user scroll
  const handleScroll = () => {
    if (feedRef.current) {
      const { scrollTop, scrollHeight, clientHeight } = feedRef.current;
      const isAtBottom = scrollHeight - scrollTop - clientHeight < 50;

      if (!isAtBottom && !userScrolled) {
        setUserScrolled(true);
        setAutoScroll(false);
      } else if (isAtBottom && userScrolled) {
        setUserScrolled(false);
        setAutoScroll(true);
      }
    }
  };

  // Reset scroll state when match type changes
  useEffect(() => {
    setAutoScroll(true);
    setUserScrolled(false);
  }, [matchType]);

  // Format event for display
  const formatEventDescription = (event: MatchEvent): string => {
    // Add historical flair to descriptions
    const descriptions: { [key: string]: (e: MatchEvent) => string } = {
      KickOff: () => 'The match commences with the kick-off.',
      Goal: (e) => `⚽ GOAL! ${e.description}`,
      Rouge: (e) => `🎯 ROUGE! ${e.description} (1 point scored behind goal)`,
      Shot: (e) => `Shot attempted! ${e.description}`,
      Save: (e) => `Excellent save! ${e.description}`,
      Pass: (e) => e.description,
      Tackle: (e) => `Tackle made! ${e.description}`,
      Turnover: (e) => `Possession lost! ${e.description}`,
      HalfTime: () => '─── Half Time ───',
      FullTime: () => '─── Full Time ───',
    };

    const formatter = descriptions[event.event_type];
    return formatter ? formatter(event) : event.description;
  };

  // Get event style class based on type
  const getEventClass = (eventType: string): string => {
    const classes: { [key: string]: string } = {
      KickOff: 'event-kickoff',
      Goal: 'event-goal',
      Rouge: 'event-rouge',
      Shot: 'event-shot',
      Save: 'event-save',
      HalfTime: 'event-halftime',
      FullTime: 'event-fulltime',
      Turnover: 'event-turnover',
    };
    return classes[eventType] || 'event-normal';
  };

  return (
    <div className="text-commentary-view">
      {/* Commentary Header */}
      <div className="commentary-header">
        <h3 className="newspaper-title">Match Report</h3>
        <div className="match-teams-header">
          <span className="team-home">{homeClubName}</span>
          <span className="vs">vs</span>
          <span className="team-away">{awayClubName}</span>
        </div>
        {matchType === 'live' && (
          <div className="live-indicator">
            <span className="live-dot"></span>
            <span>LIVE COVERAGE</span>
          </div>
        )}
      </div>

      {/* Main Content Grid */}
      <div className="commentary-content">
        {/* Event Feed */}
        <div className="commentary-feed-container">
          <div
            className="commentary-feed"
            ref={feedRef}
            onScroll={handleScroll}
          >
            {events.length === 0 && (
              <div className="no-events">
                <p>Awaiting match events...</p>
              </div>
            )}

            {events.map((event, idx) => (
              <div
                key={idx}
                className={`commentary-event ${getEventClass(event.event_type)}`}
              >
                <div className="event-time">
                  <span className="minute">{event.minute}'</span>
                  {event.second > 0 && (
                    <span className="second">{event.second}s</span>
                  )}
                </div>
                <div className="event-content">
                  <div className="event-type-badge">{event.event_type}</div>
                  <div className="event-description">
                    {formatEventDescription(event)}
                  </div>
                  {event.players_involved.length > 0 && (
                    <div className="event-players">
                      Players: {event.players_involved.join(', ')}
                    </div>
                  )}
                </div>
              </div>
            ))}

            {/* Scroll indicator */}
            {!autoScroll && (
              <button
                className="scroll-to-bottom"
                onClick={() => {
                  setAutoScroll(true);
                  setUserScrolled(false);
                  if (feedRef.current) {
                    feedRef.current.scrollTop = feedRef.current.scrollHeight;
                  }
                }}
              >
                ↓ New Events Below
              </button>
            )}
          </div>
        </div>

        {/* Statistics Sidebar */}
        <div className="statistics-sidebar">
          <div className="statistics-panel">
            <h4 className="stats-title">Match Statistics</h4>

            {currentStatistics ? (
              <div className="stats-grid">
                {/* Possession */}
                <div className="stat-item">
                  <div className="stat-label">Possession</div>
                  <div className="stat-bar">
                    <div className="stat-bar-home" style={{ width: `${currentStatistics.home_possession || 0}%` }}>
                      <span>{currentStatistics.home_possession || 0}%</span>
                    </div>
                    <div className="stat-bar-away" style={{ width: `${currentStatistics.away_possession || 0}%` }}>
                      <span>{currentStatistics.away_possession || 0}%</span>
                    </div>
                  </div>
                </div>

                {/* Shots */}
                <div className="stat-item">
                  <div className="stat-label">Shots</div>
                  <div className="stat-values">
                    <span className="stat-home">{currentStatistics.home_shots || 0}</span>
                    <span className="stat-separator">-</span>
                    <span className="stat-away">{currentStatistics.away_shots || 0}</span>
                  </div>
                </div>

                {/* Shots on Target */}
                <div className="stat-item">
                  <div className="stat-label">Shots on Target</div>
                  <div className="stat-values">
                    <span className="stat-home">{currentStatistics.home_shots_on_target || 0}</span>
                    <span className="stat-separator">-</span>
                    <span className="stat-away">{currentStatistics.away_shots_on_target || 0}</span>
                  </div>
                </div>

                {/* Passes */}
                <div className="stat-item">
                  <div className="stat-label">Passes</div>
                  <div className="stat-values">
                    <span className="stat-home">{currentStatistics.home_passes || 0}</span>
                    <span className="stat-separator">-</span>
                    <span className="stat-away">{currentStatistics.away_passes || 0}</span>
                  </div>
                </div>

                {/* Pass Accuracy */}
                <div className="stat-item">
                  <div className="stat-label">Pass Accuracy</div>
                  <div className="stat-values">
                    <span className="stat-home">{currentStatistics.home_pass_accuracy?.toFixed(1) || '0.0'}%</span>
                    <span className="stat-separator">-</span>
                    <span className="stat-away">{currentStatistics.away_pass_accuracy?.toFixed(1) || '0.0'}%</span>
                  </div>
                </div>
              </div>
            ) : (
              <div className="no-stats">
                <p>Statistics will appear when the match begins</p>
              </div>
            )}

            {/* Scorers List */}
            {matchData?.scorers && matchData.scorers.length > 0 && (
              <div className="scorers-section">
                <h4 className="scorers-title">Scorers</h4>
                <div className="scorers-list">
                  {matchData.scorers.map((scorer, idx) => (
                    <div key={idx} className="scorer-item">
                      <span className="scorer-icon">
                        {scorer.score_type === 'Goal' ? '⚽' : '🎯'}
                      </span>
                      <span className="scorer-name">{scorer.player_name}</span>
                      <span className="scorer-minute">{scorer.minute}'</span>
                    </div>
                  ))}
                </div>
              </div>
            )}
          </div>

          {/* Historical Context Box */}
          <div className="historical-context">
            <h4>Sheffield Rules Context</h4>
            <p className="context-text">
              This match is played under the Sheffield Rules, the world's first
              codified football regulations (1858-1877).
            </p>
            <ul className="context-list">
              <li>Rouge scoring: Points awarded for placing the ball behind goal</li>
              <li>Fair catches permitted from kick-offs</li>
              <li>No offsides from throw-ins</li>
            </ul>
          </div>
        </div>
      </div>
    </div>
  );
}
