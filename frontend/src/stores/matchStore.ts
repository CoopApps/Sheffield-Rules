/**
 * Match Viewer Store - Zustand state management for match viewing
 * Handles both live match streaming and replay playback
 */

import { create } from 'zustand';
import { invoke } from '@tauri-apps/api/core';
import { listen } from '@tauri-apps/api/event';

// Match event types from backend
export interface MatchEvent {
  minute: number;
  second: number;
  event_type: string; // KickOff | Pass | Shot | Goal | Rouge | Save | Tackle | Turnover | OutOfPlay | ThrowIn | GoalKick | FreeKick | HalfTime | FullTime
  description: string;
  position: {
    x: number;
    y: number;
  };
  players_involved: string[];
  team_side: string;
}

// Visual state for 2D rendering
export interface VisualState {
  minute: number;
  ball_position: {
    x: number;
    y: number;
  };
  ball_z: number;
  player_positions: PlayerPositionState[];
  possession_team: string;
}

export interface PlayerPositionState {
  player_id: string;
  name: string;
  team: string;
  position: {
    x: number;
    y: number;
  };
  has_ball: boolean;
}

// Match statistics
export interface MatchStatistics {
  home_possession: number;
  away_possession: number;
  home_shots: number;
  away_shots: number;
  home_shots_on_target: number;
  away_shots_on_target: number;
  home_passes: number;
  away_passes: number;
  home_pass_accuracy: number;
  away_pass_accuracy: number;
}

// Complete match result (for replays)
export interface MatchResult {
  match_id: string;
  home_score: number;
  away_score: number;
  home_rouges: number;
  away_rouges: number;
  events: MatchEvent[];
  visual_states: VisualState[];
  statistics: MatchStatistics;
  scorers: ScorerInfo[];
}

export interface ScorerInfo {
  player_name: string;
  team: string;
  minute: number;
  score_type: string; // "Goal" or "Rouge"
}

// Live match update from backend
export interface MatchUpdate {
  match_id: string;
  minute: number;
  event: MatchEvent;
  visual_state: VisualState;
  statistics: MatchStatistics;
}

// Store interface
interface MatchState {
  // Current viewing mode
  viewMode: 'fullscreen-2d' | 'text-commentary' | 'live-view';
  playbackSpeed: number;  // 1x, 2x, 4x
  currentMinute: number;
  isPaused: boolean;

  // Match data
  matchId: string | null;
  matchType: 'live' | 'replay';
  matchData: MatchResult | null;
  liveEvents: MatchEvent[];
  liveVisualStates: VisualState[];
  currentStatistics: MatchStatistics | null;

  // Internal: cleanup function for event listener
  _unlistenFn: (() => void) | null;

  // Actions
  setViewMode: (mode: 'fullscreen-2d' | 'text-commentary' | 'live-view') => void;
  setPlaybackSpeed: (speed: number) => void;
  seekTo: (minute: number) => void;
  togglePause: () => void;
  loadReplayMatch: (matchId: string) => Promise<void>;
  startLiveMatch: (matchId: string, homeClubId: string, awayClubId: string, ruleYear: number) => Promise<void>;
  reset: () => void;
}

export const useMatchStore = create<MatchState>((set, get) => ({
  // Initial state
  viewMode: 'fullscreen-2d',
  playbackSpeed: 1,
  currentMinute: 0,
  isPaused: false,
  matchId: null,
  matchType: 'replay',
  matchData: null,
  liveEvents: [],
  liveVisualStates: [],
  currentStatistics: null,
  _unlistenFn: null,

  // Actions
  setViewMode: (mode) => set({ viewMode: mode }),

  setPlaybackSpeed: (speed) => set({ playbackSpeed: speed }),

  seekTo: (minute) => set({ currentMinute: minute }),

  togglePause: () => set((state) => ({ isPaused: !state.isPaused })),

  loadReplayMatch: async (matchId: string) => {
    try {
      const data = await invoke<MatchResult>('get_match_replay_data', { matchId });
      set({
        matchId,
        matchType: 'replay',
        matchData: data,
        currentMinute: 0,
        isPaused: true,
        currentStatistics: data.statistics,
      });
    } catch (error) {
      console.error('Failed to load replay match:', error);
      throw error;
    }
  },

  startLiveMatch: async (matchId: string, homeClubId: string, awayClubId: string, ruleYear: number) => {
    try {
      // Clean up any existing listener to prevent duplicate events
      const existingUnlisten = get()._unlistenFn;
      if (existingUnlisten) {
        existingUnlisten();
      }

      // Clear previous match data
      set({
        matchId,
        matchType: 'live',
        matchData: null,
        liveEvents: [],
        liveVisualStates: [],
        currentMinute: 0,
        isPaused: false,
        currentStatistics: null,
        _unlistenFn: null,
      });

      // Listen for match updates BEFORE starting simulation
      const unlisten = await listen<MatchUpdate>('match_update', (event) => {
        const update = event.payload;

        if (update.match_id === matchId) {
          set((state) => ({
            liveEvents: [...state.liveEvents, update.event],
            liveVisualStates: [...state.liveVisualStates, update.visual_state],
            currentMinute: update.minute,
            currentStatistics: update.statistics,
          }));
        }
      });

      // Store unlisten for cleanup
      set({ _unlistenFn: unlisten });

      // Start live simulation
      await invoke('simulate_match_live', {
        matchId,
        homeClubId,
        awayClubId,
        ruleYear,
      });

    } catch (error) {
      console.error('Failed to start live match:', error);
      throw error;
    }
  },

  reset: () => {
    // Clean up any existing listener
    const existingUnlisten = get()._unlistenFn;
    if (existingUnlisten) {
      existingUnlisten();
    }
    set({
      viewMode: 'fullscreen-2d',
      playbackSpeed: 1,
      currentMinute: 0,
      isPaused: false,
      matchId: null,
      matchType: 'replay',
      matchData: null,
      liveEvents: [],
      liveVisualStates: [],
      currentStatistics: null,
      _unlistenFn: null,
    });
  },
}));
