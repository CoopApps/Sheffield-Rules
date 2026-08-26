import React, { useState, useEffect } from 'react';
import { invoke } from '@tauri-apps/api/core';
import '../styles/whites-matcher.css';

interface WhitesEntry {
  id: string;
  surname?: string;
  forename?: string;
  full_name?: string;
  name?: string;
  title?: string;
  occupation?: string;
  address?: string;
  street_address?: string;
  year: number;
  source?: string;
  person_id?: string;
}

interface PlayerCandidate {
  id: string;
  name: string;
  first_name?: string;
  surname?: string;
  birth_year?: number;
  profession?: string;
  street_address?: string;
  civil_parish?: string;
  ecclesiastical_parish?: string;
}

interface MatchFactors {
  name_score: number;
  address_score: number;
  profession_score: number;
  parish_score: number;
}

interface PlayerCandidateWithScore {
  player: PlayerCandidate;
  confidence_score: number;
  match_factors: MatchFactors;
  match_reason: string;
}

interface WhitesMatchSuggestion {
  whites_entry: WhitesEntry;
  candidates: PlayerCandidateWithScore[];
}

interface MatchStats {
  total_whites_entries: number;
  matched_entries: number;
  unmatched_entries: number;
  suggestions_generated: number;
}

const WhitesMatcherScreen: React.FC = () => {
  const [whitesEntries, setWhitesEntries] = useState<WhitesEntry[]>([]);
  const [playerCandidates, setPlayerCandidates] = useState<PlayerCandidate[]>([]);
  const [suggestions, setSuggestions] = useState<WhitesMatchSuggestion[]>([]);
  const [stats, setStats] = useState<MatchStats | null>(null);
  const [loading, setLoading] = useState(true);
  const [minConfidence, setMinConfidence] = useState(0.3);
  const [maxSuggestions, setMaxSuggestions] = useState(5);
  const [selectedWhitesEntry, setSelectedWhitesEntry] = useState<WhitesEntry | null>(null);
  const [draggedPlayer, setDraggedPlayer] = useState<PlayerCandidate | null>(null);
  const [filterText, setFilterText] = useState('');
  const [showSuggestionsOnly, setShowSuggestionsOnly] = useState(false);
  const [searchTimeout, setSearchTimeout] = useState<number | null>(null);

  useEffect(() => {
    loadData();
  }, []);

  useEffect(() => {
    // Debounce search with longer delay for better performance
    if (searchTimeout) {
      clearTimeout(searchTimeout);
    }

    if (filterText.length >= 3) {
      const timeout = setTimeout(() => {
        searchPlayers(filterText);
      }, 600);
      setSearchTimeout(timeout);
    } else if (filterText.length === 0) {
      setPlayerCandidates([]);
    }

    return () => {
      if (searchTimeout) {
        clearTimeout(searchTimeout);
      }
    };
  }, [filterText]);

  const loadData = async () => {
    try {
      setLoading(true);

      const [entries, statsData] = await Promise.all([
        invoke<WhitesEntry[]>('whites_get_unmatched_entries'),
        invoke<MatchStats>('whites_get_stats'),
      ]);
      setWhitesEntries(entries);
      setStats(statsData);
    } catch (error) {
      console.error('Error loading data:', error);
      alert('Error loading data: ' + error);
    } finally {
      setLoading(false);
    }
  };

  const searchPlayers = async (searchTerm: string) => {
    try {
      const players = await invoke<PlayerCandidate[]>('whites_search_player_candidates', {
        searchTerm,
        limit: 500,
      });
      setPlayerCandidates(players);
    } catch (error) {
      console.error('Error searching players:', error);
    }
  };

  const deletePlayer = async (playerId: string, playerName: string) => {
    const confirmed = window.confirm(
      `Are you sure you want to permanently delete "${playerName}" from the database?`
    );
    if (confirmed) {
      try {
        await invoke('db_delete_player', { playerId });
        // Remove from local state
        setPlayerCandidates(prev => prev.filter(p => p.id !== playerId));
        alert('Player deleted successfully');
      } catch (error) {
        console.error('Error deleting player:', error);
        alert('Error deleting player: ' + error);
      }
    }
  };

  const generateSuggestions = async () => {
    try {
      setLoading(true);
      const suggestionsData = await invoke<WhitesMatchSuggestion[]>(
        'whites_generate_suggestions',
        { minConfidence, maxSuggestionsPerEntry: maxSuggestions }
      );
      setSuggestions(suggestionsData);
      setStats(prev => prev ? { ...prev, suggestions_generated: suggestionsData.length } : null);
    } catch (error) {
      console.error('Error generating suggestions:', error);
      alert('Error generating suggestions: ' + error);
    } finally {
      setLoading(false);
    }
  };

  const acceptMatch = async (whitesId: string, playerId: string) => {
    try {
      await invoke('whites_accept_match', { whitesEntryId: whitesId, playerId });
      // Refresh data
      await loadData();
      // Clear selection
      setSelectedWhitesEntry(null);
      // Remove from suggestions
      setSuggestions(prev => prev.filter(s => s.whites_entry.id !== whitesId));
      alert('Match accepted successfully!');
    } catch (error) {
      console.error('Error accepting match:', error);
      alert('Error accepting match: ' + error);
    }
  };

  const rejectMatch = async (whitesId: string, playerId: string) => {
    const notes = prompt('Optional: Why are you rejecting this match?');
    try {
      await invoke('whites_reject_match', {
        whitesEntryId: whitesId,
        playerId,
        notes: notes || undefined,
      });
      alert('Match rejected');
    } catch (error) {
      console.error('Error rejecting match:', error);
      alert('Error rejecting match: ' + error);
    }
  };

  const handleDragStart = (player: PlayerCandidate) => {
    setDraggedPlayer(player);
  };

  const handleDragEnd = () => {
    setDraggedPlayer(null);
  };

  const handleDrop = (whitesEntry: WhitesEntry) => {
    if (draggedPlayer) {
      const confirmed = window.confirm(
        `Match "${draggedPlayer.name}" to business "${whitesEntry.name}" at ${whitesEntry.street_address}?`
      );
      if (confirmed) {
        acceptMatch(whitesEntry.id, draggedPlayer.id);
      }
    }
  };

  const handleDragOver = (e: React.DragEvent) => {
    e.preventDefault();
  };

  const getConfidenceColor = (confidence: number): string => {
    if (confidence >= 0.8) return '#4caf50'; // Green
    if (confidence >= 0.6) return '#ff9800'; // Orange
    if (confidence >= 0.4) return '#ff5722'; // Red-orange
    return '#9e9e9e'; // Gray
  };

  const getConfidenceLabel = (confidence: number): string => {
    if (confidence >= 0.8) return 'Excellent';
    if (confidence >= 0.6) return 'Good';
    if (confidence >= 0.4) return 'Fair';
    return 'Weak';
  };

  const displayWhitesEntries = showSuggestionsOnly
    ? suggestions.map(s => s.whites_entry)
    : whitesEntries;

  if (loading && !stats) {
    return <div className="whites-matcher-loading">Loading Whites Directory Matcher...</div>;
  }

  return (
    <div className="whites-matcher-container">
      <div className="whites-matcher-header">
        <h1>Whites Directory Matcher</h1>
        <p className="whites-matcher-subtitle">
          Match business directory entries to Sheffield people in the database
        </p>
      </div>

      {stats && (
        <div className="whites-matcher-stats">
          <div className="stat-card">
            <div className="stat-value">{stats.total_whites_entries}</div>
            <div className="stat-label">Total Entries</div>
          </div>
          <div className="stat-card">
            <div className="stat-value">{stats.matched_entries}</div>
            <div className="stat-label">Matched</div>
          </div>
          <div className="stat-card">
            <div className="stat-value">{stats.unmatched_entries}</div>
            <div className="stat-label">Unmatched</div>
          </div>
          <div className="stat-card">
            <div className="stat-value">{stats.suggestions_generated}</div>
            <div className="stat-label">Suggestions</div>
          </div>
        </div>
      )}

      <div className="whites-matcher-controls">
        <div className="control-group">
          <label>
            Min Confidence:
            <input
              type="range"
              min="0"
              max="1"
              step="0.1"
              value={minConfidence}
              onChange={(e) => setMinConfidence(parseFloat(e.target.value))}
            />
            <span className="confidence-value">{(minConfidence * 100).toFixed(0)}%</span>
          </label>
        </div>
        <div className="control-group">
          <label>
            Max Suggestions:
            <input
              type="number"
              min="1"
              max="20"
              value={maxSuggestions}
              onChange={(e) => setMaxSuggestions(parseInt(e.target.value))}
            />
          </label>
        </div>
        <button onClick={generateSuggestions} className="btn-primary" disabled={loading}>
          {loading ? 'Generating...' : 'Generate Suggestions'}
        </button>
        <button onClick={loadData} className="btn-secondary" disabled={loading}>
          Refresh Data
        </button>
        <label className="checkbox-label">
          <input
            type="checkbox"
            checked={showSuggestionsOnly}
            onChange={(e) => setShowSuggestionsOnly(e.target.checked)}
          />
          Show suggestions only
        </label>
      </div>

      <div className="whites-matcher-main">
        {/* Left Panel: Whites Directory Entries */}
        <div className="whites-panel">
          <div className="panel-header">
            <h2>Whites Directory ({displayWhitesEntries.length})</h2>
            <p className="panel-subtitle">Click to select, then drag a person to match</p>
          </div>
          <div className="whites-list">
            {displayWhitesEntries.map((entry) => {
              const suggestion = suggestions.find(s => s.whites_entry.id === entry.id);
              const isSelected = selectedWhitesEntry?.id === entry.id;

              return (
                <div
                  key={entry.id}
                  className={`whites-entry ${isSelected ? 'selected' : ''} ${draggedPlayer ? 'drop-target' : ''}`}
                  onClick={() => setSelectedWhitesEntry(entry)}
                  onDrop={() => handleDrop(entry)}
                  onDragOver={handleDragOver}
                >
                  <div className="entry-header">
                    <h3>{entry.full_name || `${entry.forename || ''} ${entry.surname || ''}`.trim()}</h3>
                    {suggestion && (
                      <span className="suggestion-badge">
                        {suggestion.candidates.length} match{suggestion.candidates.length !== 1 ? 'es' : ''}
                      </span>
                    )}
                  </div>
                  {entry.title && (
                    <div className="entry-business">{entry.title}</div>
                  )}
                  <div className="entry-details">
                    {entry.occupation && <span className="badge profession">{entry.occupation}</span>}
                  </div>
                  <div className="entry-location">
                    <div>{entry.address || 'No address'}</div>
                  </div>
                  <div className="entry-year">Year: {entry.year}</div>

                  {/* Show suggestions if this entry is selected */}
                  {isSelected && suggestion && (
                    <div className="inline-suggestions">
                      <h4>Suggested Matches:</h4>
                      {suggestion.candidates.map((candidate) => (
                        <div key={candidate.player.id} className="inline-candidate">
                          <div className="candidate-info">
                            <strong>{candidate.player.name}</strong>
                            {candidate.player.birth_year && ` (b. ${candidate.player.birth_year})`}
                            <div className="candidate-details">
                              {candidate.player.profession && <div>Profession: {candidate.player.profession}</div>}
                              {candidate.player.street_address && <div>Address: {candidate.player.street_address}</div>}
                              {(candidate.player.civil_parish || candidate.player.ecclesiastical_parish) && (
                                <div>Parish: {candidate.player.ecclesiastical_parish || candidate.player.civil_parish}</div>
                              )}
                            </div>
                          </div>
                          <div className="candidate-score">
                            <div
                              className="confidence-bar"
                              style={{
                                width: `${candidate.confidence_score * 100}%`,
                                backgroundColor: getConfidenceColor(candidate.confidence_score),
                              }}
                            />
                            <div className="confidence-label">
                              {(candidate.confidence_score * 100).toFixed(0)}% - {getConfidenceLabel(candidate.confidence_score)}
                            </div>
                            <div className="match-reason">{candidate.match_reason}</div>
                          </div>
                          <div className="candidate-actions">
                            <button
                              className="btn-accept"
                              onClick={(e) => {
                                e.stopPropagation();
                                acceptMatch(entry.id, candidate.player.id);
                              }}
                            >
                              ✓ Accept
                            </button>
                            <button
                              className="btn-reject"
                              onClick={(e) => {
                                e.stopPropagation();
                                rejectMatch(entry.id, candidate.player.id);
                              }}
                            >
                              ✗ Reject
                            </button>
                          </div>
                        </div>
                      ))}
                    </div>
                  )}
                </div>
              );
            })}
          </div>
        </div>

        {/* Right Panel: Player Candidates */}
        <div className="players-panel">
          <div className="panel-header">
            <h2>Sheffield People ({playerCandidates.length})</h2>
            <p className="panel-subtitle">Search and drag a person to a business to create a match</p>
          </div>
          <div className="player-search">
            <input
              type="text"
              placeholder="Search by name, profession, address, or parish (min 3 chars)..."
              value={filterText}
              onChange={(e) => setFilterText(e.target.value)}
              className="search-input"
            />
          </div>
          <div className="players-list">
            {playerCandidates.length === 0 && filterText.length > 0 && (
              <div style={{ padding: '20px', textAlign: 'center', color: '#888' }}>
                {filterText.length < 3 ? 'Type at least 3 characters to search...' : 'No results found'}
              </div>
            )}
            {playerCandidates.length === 0 && filterText.length === 0 && (
              <div style={{ padding: '20px', textAlign: 'center', color: '#888' }}>
                Type in the search box to find people
              </div>
            )}
            {playerCandidates.map((player) => (
              <div
                key={player.id}
                className="player-candidate"
                draggable
                onDragStart={() => handleDragStart(player)}
                onDragEnd={handleDragEnd}
              >
                <div className="player-header">
                  <h3>{player.name}</h3>
                  <div style={{ display: 'flex', gap: '8px', alignItems: 'center' }}>
                    {player.birth_year && <span className="birth-year">b. {player.birth_year}</span>}
                    <button
                      className="btn-delete-player"
                      onClick={(e) => {
                        e.stopPropagation();
                        deletePlayer(player.id, player.name);
                      }}
                      title="Delete this person"
                    >
                      ×
                    </button>
                  </div>
                </div>
                {player.profession && (
                  <div className="player-profession">{player.profession}</div>
                )}
                {player.street_address && (
                  <div className="player-address">{player.street_address}</div>
                )}
                {(player.civil_parish || player.ecclesiastical_parish) && (
                  <div className="player-parish">
                    {player.ecclesiastical_parish || player.civil_parish}
                  </div>
                )}
              </div>
            ))}
          </div>
        </div>
      </div>
    </div>
  );
};

export default WhitesMatcherScreen;
