import React, { useState } from 'react';
import { invoke } from '../utils/tauriInvoke';
import '../styles/create-league-modal.css';

interface LeagueMetadata {
  id: string;
  name: string;
  description: string;
  season_year: number;
  is_active: boolean;
  created_at: string;
  updated_at: string;
}

interface CreateLeagueModalProps {
  onClose: () => void;
  onLeagueCreated: (league: LeagueMetadata) => void;
}

export function CreateLeagueModal({ onClose, onLeagueCreated }: CreateLeagueModalProps) {
  const [leagueName, setLeagueName] = useState('');
  const [description, setDescription] = useState('');
  const [seasonYear, setSeasonYear] = useState(1867);
  const [error, setError] = useState('');
  const [creating, setCreating] = useState(false);

  const handleCreate = async () => {
    if (!leagueName.trim()) {
      setError('Please enter a league name');
      return;
    }

    if (!description.trim()) {
      setError('Please enter a description');
      return;
    }

    if (seasonYear < 1850 || seasonYear > 2100) {
      setError('Season year must be between 1850 and 2100');
      return;
    }

    setCreating(true);
    setError('');

    try {
      // Create the league metadata entry
      const league = await invoke<LeagueMetadata>('db_create_league', {
        name: leagueName,
        description: description,
        seasonYear: seasonYear,
      });

      onLeagueCreated(league);
    } catch (err) {
      setError(`Failed to create league: ${err}`);
    } finally {
      setCreating(false);
    }
  };

  return (
    <div className="modal-overlay" onClick={onClose}>
      <div className="modal-content" onClick={(e) => e.stopPropagation()}>
        <div className="modal-header">
          <h2>Create New League</h2>
          <button className="close-button" onClick={onClose}>×</button>
        </div>

        {error && <div className="error-message">{error}</div>}

        <div className="modal-body">
          <div className="form-field">
            <label htmlFor="league-name">League Name</label>
            <input
              id="league-name"
              type="text"
              value={leagueName}
              onChange={(e) => setLeagueName(e.target.value)}
              placeholder="e.g., English Football League 1888"
              className="form-input"
              disabled={creating}
            />
          </div>

          <div className="form-field">
            <label htmlFor="description">Description</label>
            <textarea
              id="description"
              value={description}
              onChange={(e) => setDescription(e.target.value)}
              placeholder="e.g., The first season of the Football League with 12 founding clubs"
              className="form-textarea"
              rows={3}
              disabled={creating}
            />
          </div>

          <div className="form-field">
            <label htmlFor="season-year">Season Year</label>
            <input
              id="season-year"
              type="number"
              value={seasonYear}
              onChange={(e) => setSeasonYear(parseInt(e.target.value))}
              min={1850}
              max={2100}
              className="form-input"
              disabled={creating}
            />
            <span className="field-hint">The starting year for this league season</span>
          </div>

          <div className="info-box">
            <strong>Next Steps:</strong>
            <ul>
              <li>After creating the league, you'll need to define divisions</li>
              <li>Configure league settings (points system, match scheduling, etc.)</li>
              <li>Assign clubs to divisions</li>
              <li>Set up promotion/relegation rules</li>
            </ul>
          </div>
        </div>

        <div className="modal-footer">
          <button onClick={onClose} className="button-secondary" disabled={creating}>
            Cancel
          </button>
          <button onClick={handleCreate} className="button-primary" disabled={creating}>
            {creating ? 'Creating...' : 'Create League'}
          </button>
        </div>
      </div>
    </div>
  );
}
