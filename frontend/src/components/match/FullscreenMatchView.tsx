/**
 * Fullscreen Match View - 2D Canvas visualization of match
 * Renders pitch, players, and ball with camera following the action
 */

import React, { useEffect, useRef, useState } from 'react';
import { VisualState, MatchResult, MatchStatistics, MatchEvent } from '../../stores/matchStore';
import './FullscreenMatchView.css';

interface FullscreenMatchViewProps {
  matchType: 'live' | 'replay';
  liveVisualStates: VisualState[];
  liveEvents: MatchEvent[];
  matchData: MatchResult | null;
  currentMinute: number;
  currentStatistics: MatchStatistics | null;
  homeClubName: string;
  awayClubName: string;
  homeScore: number;
  awayScore: number;
}

export function FullscreenMatchView({
  matchType,
  liveVisualStates,
  liveEvents,
  matchData,
  currentMinute,
  currentStatistics,
  homeClubName,
  awayClubName,
  homeScore,
  awayScore,
}: FullscreenMatchViewProps) {
  const canvasRef = useRef<HTMLCanvasElement>(null);
  const animationFrameRef = useRef<number>();
  const [camera, setCamera] = useState({ x: 0.5, y: 0.5, zoom: 1.0 });
  const [currentEventIndex, setCurrentEventIndex] = useState(0);

  // Get current visual state based on match type and minute
  const getCurrentVisualState = (): VisualState | null => {
    if (matchType === 'live') {
      // For live matches, get the latest state
      return liveVisualStates[liveVisualStates.length - 1] || null;
    } else if (matchData?.visual_states) {
      // For replays, find state closest to current minute
      return matchData.visual_states.find(s => s.minute === currentMinute) ||
             matchData.visual_states[matchData.visual_states.length - 1] ||
             null;
    }
    return null;
  };

  const currentState = getCurrentVisualState();

  // Canvas rendering
  useEffect(() => {
    const canvas = canvasRef.current;
    if (!canvas) return;

    const ctx = canvas.getContext('2d');
    if (!ctx) return;

    // Set canvas size
    const updateCanvasSize = () => {
      const container = canvas.parentElement;
      if (container) {
        canvas.width = container.clientWidth;
        canvas.height = container.clientHeight;
      }
    };

    updateCanvasSize();
    window.addEventListener('resize', updateCanvasSize);

    // Game loop
    const render = () => {
      if (!ctx || !canvas) return;

      // Clear canvas
      ctx.clearRect(0, 0, canvas.width, canvas.height);

      // Draw pitch
      drawPitch(ctx, canvas.width, canvas.height);

      // Draw players and ball
      if (currentState) {
        // Update camera to follow ball
        const ballX = currentState.ball_position.x;
        const ballY = currentState.ball_position.y;
        setCamera({ x: ballX, y: ballY, zoom: 1.0 });

        // Draw players
        currentState.player_positions.forEach(player => {
          drawPlayer(ctx, canvas.width, canvas.height, player);
        });

        // Draw ball
        drawBall(ctx, canvas.width, canvas.height, currentState.ball_position);
      }

      // Draw scoreboard
      drawScoreboard(ctx, canvas.width, homeClubName, awayClubName, homeScore, awayScore, currentMinute);

      // Draw possession indicator
      if (currentState) {
        drawPossessionIndicator(ctx, canvas.width, canvas.height, currentState.possession_team);
      }

      animationFrameRef.current = requestAnimationFrame(render);
    };

    render();

    return () => {
      window.removeEventListener('resize', updateCanvasSize);
      if (animationFrameRef.current) {
        cancelAnimationFrame(animationFrameRef.current);
      }
    };
  }, [currentState, homeClubName, awayClubName, homeScore, awayScore, currentMinute]);

  // Get events to display
  const eventsToDisplay = matchType === 'live'
    ? liveEvents
    : (matchData?.events || []);

  const totalEvents = eventsToDisplay.length;

  // When a new event arrives, show it immediately
  useEffect(() => {
    if (totalEvents > 0) {
      setCurrentEventIndex(totalEvents - 1);
    }
  }, [totalEvents]);

  // Auto-cycle through recent events every 3 seconds
  useEffect(() => {
    if (totalEvents === 0) return;

    const timer = setInterval(() => {
      setCurrentEventIndex((prev) => {
        const windowStart = Math.max(0, totalEvents - 20);
        if (prev < totalEvents - 1) {
          return prev + 1;
        } else {
          return windowStart;
        }
      });
    }, 3000);

    return () => clearInterval(timer);
  }, [totalEvents]);

  const currentEvent = eventsToDisplay[currentEventIndex];

  return (
    <div className="fullscreen-match-view">
      <canvas ref={canvasRef} className="match-canvas" />

      {/* Centered Commentary Message */}
      {currentEvent && (
        <div className={`commentary-center event-${currentEvent.event_type.toLowerCase()}`}>
          <div className="commentary-minute">{currentEvent.minute}'</div>
          <div className="commentary-message">{currentEvent.description}</div>
        </div>
      )}
      {!currentEvent && eventsToDisplay.length === 0 && (
        <div className="commentary-center">
          <div className="commentary-message">Match starting...</div>
        </div>
      )}

      {/* Statistics Overlay */}
      {currentStatistics && (
        <div className="stats-overlay">
          <div className="stat-item">
            <span className="stat-label">Possession</span>
            <div className="stat-bar-mini">
              <div
                className="stat-bar-fill home"
                style={{ width: `${currentStatistics.home_possession}%` }}
              />
            </div>
            <span className="stat-value">
              {currentStatistics.home_possession}% - {currentStatistics.away_possession}%
            </span>
          </div>
          <div className="stat-item">
            <span className="stat-label">Shots</span>
            <span className="stat-value">
              {currentStatistics.home_shots} - {currentStatistics.away_shots}
            </span>
          </div>
        </div>
      )}
    </div>
  );
}

// Pitch rendering
function drawPitch(ctx: CanvasRenderingContext2D, width: number, height: number) {
  // Grass background
  ctx.fillStyle = '#2a5a2a';
  ctx.fillRect(0, 0, width, height);

  // Pitch outline (white lines)
  ctx.strokeStyle = '#ffffff';
  ctx.lineWidth = 3;

  const margin = 40;
  const pitchWidth = width - margin * 2;
  const pitchHeight = height - margin * 2;

  // Outer boundary
  ctx.strokeRect(margin, margin, pitchWidth, pitchHeight);

  // Center line
  ctx.beginPath();
  ctx.moveTo(width / 2, margin);
  ctx.lineTo(width / 2, height - margin);
  ctx.stroke();

  // Center circle
  const centerX = width / 2;
  const centerY = height / 2;
  const circleRadius = Math.min(pitchWidth, pitchHeight) * 0.15;

  ctx.beginPath();
  ctx.arc(centerX, centerY, circleRadius, 0, Math.PI * 2);
  ctx.stroke();

  // Center spot
  ctx.fillStyle = '#ffffff';
  ctx.beginPath();
  ctx.arc(centerX, centerY, 5, 0, Math.PI * 2);
  ctx.fill();

  // Goal areas
  const goalWidth = pitchHeight * 0.3;
  const goalDepth = 30;

  // Home goal (left)
  ctx.strokeRect(margin, centerY - goalWidth / 2, goalDepth, goalWidth);

  // Away goal (right)
  ctx.strokeRect(width - margin - goalDepth, centerY - goalWidth / 2, goalDepth, goalWidth);

  // Penalty spots (Sheffield Rules didn't have these, but added for visual clarity)
  const penaltyDistance = pitchWidth * 0.15;

  ctx.fillStyle = '#ffffff';
  ctx.beginPath();
  ctx.arc(margin + penaltyDistance, centerY, 4, 0, Math.PI * 2);
  ctx.fill();

  ctx.beginPath();
  ctx.arc(width - margin - penaltyDistance, centerY, 4, 0, Math.PI * 2);
  ctx.fill();
}

// Player rendering
function drawPlayer(
  ctx: CanvasRenderingContext2D,
  canvasWidth: number,
  canvasHeight: number,
  player: { name: string; team: string; position: { x: number; y: number }; has_ball: boolean }
) {
  const margin = 40;
  const pitchWidth = canvasWidth - margin * 2;
  const pitchHeight = canvasHeight - margin * 2;

  // Convert normalized position (0-1) to canvas coordinates
  const x = margin + player.position.x * pitchWidth;
  const y = margin + player.position.y * pitchHeight;

  // Team colors
  const isHome = player.team === 'Home';
  const playerColor = isHome ? '#3498db' : '#e74c3c'; // Blue for home, red for away
  const outlineColor = player.has_ball ? '#f1c40f' : '#ffffff'; // Gold outline if has ball

  // Draw player circle
  ctx.beginPath();
  ctx.arc(x, y, 12, 0, Math.PI * 2);
  ctx.fillStyle = playerColor;
  ctx.fill();

  // Outline
  ctx.strokeStyle = outlineColor;
  ctx.lineWidth = player.has_ball ? 3 : 2;
  ctx.stroke();

  // Player name (abbreviated)
  const nameParts = player.name.split(' ');
  const initials = nameParts.map(p => p[0]).join('').substring(0, 2);

  ctx.fillStyle = '#ffffff';
  ctx.font = 'bold 10px Arial';
  ctx.textAlign = 'center';
  ctx.textBaseline = 'middle';
  ctx.fillText(initials, x, y);
}

// Ball rendering
function drawBall(
  ctx: CanvasRenderingContext2D,
  canvasWidth: number,
  canvasHeight: number,
  ballPosition: { x: number; y: number }
) {
  const margin = 40;
  const pitchWidth = canvasWidth - margin * 2;
  const pitchHeight = canvasHeight - margin * 2;

  const x = margin + ballPosition.x * pitchWidth;
  const y = margin + ballPosition.y * pitchHeight;

  // Draw ball
  ctx.beginPath();
  ctx.arc(x, y, 8, 0, Math.PI * 2);
  ctx.fillStyle = '#ffffff';
  ctx.fill();

  ctx.strokeStyle = '#000000';
  ctx.lineWidth = 2;
  ctx.stroke();

  // Ball shadow for depth
  ctx.beginPath();
  ctx.arc(x + 2, y + 2, 8, 0, Math.PI * 2);
  ctx.fillStyle = 'rgba(0, 0, 0, 0.3)';
  ctx.fill();
}

// Scoreboard
function drawScoreboard(
  ctx: CanvasRenderingContext2D,
  canvasWidth: number,
  homeTeam: string,
  awayTeam: string,
  homeScore: number,
  awayScore: number,
  minute: number
) {
  const scoreboardWidth = 350;
  const scoreboardHeight = 80;
  const x = (canvasWidth - scoreboardWidth) / 2;
  const y = 20;

  // Background
  ctx.fillStyle = 'rgba(0, 0, 0, 0.8)';
  ctx.fillRect(x, y, scoreboardWidth, scoreboardHeight);

  // Border
  ctx.strokeStyle = '#d4af37';
  ctx.lineWidth = 2;
  ctx.strokeRect(x, y, scoreboardWidth, scoreboardHeight);

  // Team names and scores
  ctx.fillStyle = '#ffffff';
  ctx.font = 'bold 16px Arial';
  ctx.textAlign = 'center';

  // Home team
  ctx.fillText(homeTeam, x + scoreboardWidth * 0.25, y + 25);
  ctx.font = 'bold 24px Arial';
  ctx.fillStyle = '#3498db';
  ctx.fillText(homeScore.toString(), x + scoreboardWidth * 0.25, y + 55);

  // VS
  ctx.fillStyle = '#d4af37';
  ctx.font = 'bold 14px Arial';
  ctx.fillText('VS', x + scoreboardWidth * 0.5, y + 40);

  // Away team
  ctx.fillStyle = '#ffffff';
  ctx.font = 'bold 16px Arial';
  ctx.fillText(awayTeam, x + scoreboardWidth * 0.75, y + 25);
  ctx.font = 'bold 24px Arial';
  ctx.fillStyle = '#e74c3c';
  ctx.fillText(awayScore.toString(), x + scoreboardWidth * 0.75, y + 55);

  // Minute
  ctx.fillStyle = '#f1c40f';
  ctx.font = 'bold 14px Arial';
  ctx.textAlign = 'right';
  ctx.fillText(`${minute}'`, x + scoreboardWidth - 10, y + scoreboardHeight - 10);
}

// Possession indicator
function drawPossessionIndicator(
  ctx: CanvasRenderingContext2D,
  canvasWidth: number,
  canvasHeight: number,
  possessionTeam: string
) {
  const text = `${possessionTeam} Possession`;
  const x = 20;
  const y = canvasHeight - 40;

  // Background
  ctx.fillStyle = 'rgba(0, 0, 0, 0.7)';
  ctx.fillRect(x - 10, y - 25, 200, 35);

  // Text
  ctx.fillStyle = possessionTeam === 'Home' ? '#3498db' : '#e74c3c';
  ctx.font = 'bold 16px Arial';
  ctx.textAlign = 'left';
  ctx.textBaseline = 'middle';
  ctx.fillText(text, x, y - 7);
}
