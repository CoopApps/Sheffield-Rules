import React, { useState, useEffect } from 'react';
import { invoke } from '../utils/tauriInvoke';
import '../styles/LeagueConfig.css';

interface LeagueConfig {
  id: string;
  season_year: number;
  season_start_date: string;
  season_end_date: string;
  points_for_win: number;
  points_for_draw: number;
  points_for_loss: number;
  default_match_day: string;
  default_kickoff_time: string;
  allow_midweek_fixtures: boolean;
  midweek_day: string | null;
  midweek_kickoff_time: string | null;
  enable_weather_cancellations: boolean;
  cancellation_threshold: number;
  rearrangement_window_weeks: number;
  priority_rearrangement: boolean;
  enable_promotion_playoffs: boolean;
  playoff_teams_per_division: number;
  playoff_format: string;
  enable_relegation_playoffs: boolean;
  relegation_playoff_teams: number;
  enable_rouge_scoring: boolean;
  rouge_prevents_draws: boolean;
  created_at: string;
  updated_at: string;
}

export function LeagueConfigPanel() {
  const [config, setConfig] = useState<LeagueConfig | null>(null);
  const [loading, setLoading] = useState(true);
  const [saving, setSaving] = useState(false);
  const [error, setError] = useState('');
  const [success, setSuccess] = useState('');
  const [hasChanges, setHasChanges] = useState(false);

  useEffect(() => {
    loadConfig();
  }, []);

  const loadConfig = async () => {
    setLoading(true);
    setError('');
    try {
      const data = await invoke<LeagueConfig>('db_get_league_config');
      setConfig(data);
      setHasChanges(false);
    } catch (err) {
      setError(`Failed to load configuration: ${err}`);
    } finally {
      setLoading(false);
    }
  };

  const handleSave = async () => {
    if (!config) return;

    setSaving(true);
    setError('');
    setSuccess('');

    try {
      await invoke('db_save_league_config', { config });
      setSuccess('Configuration saved successfully!');
      setHasChanges(false);
      setTimeout(() => setSuccess(''), 3000);
    } catch (err) {
      setError(`Failed to save configuration: ${err}`);
    } finally {
      setSaving(false);
    }
  };

  const handleReset = () => {
    loadConfig();
  };

  const updateConfig = <K extends keyof LeagueConfig>(key: K, value: LeagueConfig[K]) => {
    if (!config) return;
    setConfig({ ...config, [key]: value });
    setHasChanges(true);
  };

  if (loading) {
    return <div className="league-config loading">Loading configuration...</div>;
  }

  if (!config) {
    return <div className="league-config error">Failed to load configuration</div>;
  }

  return (
    <div className="league-config">
      <div className="config-header">
        <h2>League Configuration</h2>
        <p>Configure season settings, points system, and match scheduling</p>
      </div>

      {error && <div className="error-message">{error}</div>}
      {success && <div className="success-message">{success}</div>}

      <div className="config-sections">
        {/* Season Settings */}
        <div className="config-section">
          <h3 className="section-title">Season Settings</h3>
          <div className="config-grid">
            <div className="config-field">
              <label>Season Year</label>
              <input
                type="number"
                value={config.season_year}
                onChange={(e) => updateConfig('season_year', parseInt(e.target.value))}
                className="config-input"
              />
              <span className="field-hint">The year this season represents</span>
            </div>

            <div className="config-field">
              <label>Season Start Date</label>
              <input
                type="date"
                value={config.season_start_date}
                onChange={(e) => updateConfig('season_start_date', e.target.value)}
                className="config-input"
              />
              <span className="field-hint">When the season begins</span>
            </div>

            <div className="config-field">
              <label>Season End Date</label>
              <input
                type="date"
                value={config.season_end_date}
                onChange={(e) => updateConfig('season_end_date', e.target.value)}
                className="config-input"
              />
              <span className="field-hint">When the season ends</span>
            </div>
          </div>
        </div>

        {/* Sheffield Rules */}
        <div className="config-section sheffield-rules-section">
          <h3 className="section-title">Sheffield Rules (1858-1877)</h3>
          <p className="section-description">
            Historical features from the world's first football code. Rouge scoring was used 1862-1868 to prevent draws.
          </p>
          <div className="config-grid">
            <div className="config-field checkbox-field">
              <label className="checkbox-label">
                <input
                  type="checkbox"
                  checked={config.enable_rouge_scoring}
                  onChange={(e) => updateConfig('enable_rouge_scoring', e.target.checked)}
                  className="config-checkbox"
                />
                <span>Enable Rouge Scoring</span>
              </label>
              <span className="field-hint">
                Goals outweigh any number of rouges. If no goals scored (or equal), rouges decide winner.
              </span>
            </div>

            {config.enable_rouge_scoring && (
              <div className="config-field checkbox-field nested">
                <label className="checkbox-label">
                  <input
                    type="checkbox"
                    checked={config.rouge_prevents_draws}
                    onChange={(e) => updateConfig('rouge_prevents_draws', e.target.checked)}
                    className="config-checkbox"
                  />
                  <span>Rouge Prevents Draws</span>
                </label>
                <span className="field-hint">
                  As in historical Sheffield (1862-1868), rouges act as tiebreaker to prevent draws
                </span>
              </div>
            )}
          </div>
        </div>

        {/* Points System */}
        <div className="config-section">
          <h3 className="section-title">Points System</h3>
          <p className="section-description">
            Configure match result points. Traditional: 2-1-0, Modern: 3-1-0 (adopted 1981)
          </p>
          <div className="config-grid">
            <div className="config-field">
              <label>Points for Win</label>
              <input
                type="number"
                min="0"
                max="10"
                value={config.points_for_win}
                onChange={(e) => updateConfig('points_for_win', parseInt(e.target.value))}
                className="config-input"
              />
              <span className="field-hint">Modern standard: 3 points, Traditional: 2 points</span>
            </div>

            <div className="config-field">
              <label>Points for Draw</label>
              <input
                type="number"
                min="0"
                max="10"
                value={config.points_for_draw}
                onChange={(e) => updateConfig('points_for_draw', parseInt(e.target.value))}
                className="config-input"
              />
              <span className="field-hint">Standard: 1 point</span>
            </div>

            <div className="config-field">
              <label>Points for Loss</label>
              <input
                type="number"
                min="0"
                max="10"
                value={config.points_for_loss}
                onChange={(e) => updateConfig('points_for_loss', parseInt(e.target.value))}
                className="config-input"
              />
              <span className="field-hint">Standard: 0 points</span>
            </div>
          </div>
        </div>

        {/* Match Scheduling */}
        <div className="config-section">
          <h3 className="section-title">Match Scheduling</h3>
          <div className="config-grid">
            <div className="config-field">
              <label>Default Match Day</label>
              <select
                value={config.default_match_day}
                onChange={(e) => updateConfig('default_match_day', e.target.value)}
                className="config-select"
              >
                <option value="Monday">Monday</option>
                <option value="Tuesday">Tuesday</option>
                <option value="Wednesday">Wednesday</option>
                <option value="Thursday">Thursday</option>
                <option value="Friday">Friday</option>
                <option value="Saturday">Saturday</option>
                <option value="Sunday">Sunday</option>
              </select>
              <span className="field-hint">Primary day for fixtures</span>
            </div>

            <div className="config-field">
              <label>Default Kickoff Time</label>
              <input
                type="time"
                value={config.default_kickoff_time}
                onChange={(e) => updateConfig('default_kickoff_time', e.target.value)}
                className="config-input"
              />
              <span className="field-hint">Traditionally 15:00 (3pm)</span>
            </div>

            <div className="config-field checkbox-field">
              <label className="checkbox-label">
                <input
                  type="checkbox"
                  checked={config.allow_midweek_fixtures}
                  onChange={(e) => updateConfig('allow_midweek_fixtures', e.target.checked)}
                  className="config-checkbox"
                />
                <span>Allow Midweek Fixtures</span>
              </label>
              <span className="field-hint">Enable fixtures on other days</span>
            </div>

            {config.allow_midweek_fixtures && (
              <>
                <div className="config-field">
                  <label>Midweek Day</label>
                  <select
                    value={config.midweek_day || 'Wednesday'}
                    onChange={(e) => updateConfig('midweek_day', e.target.value)}
                    className="config-select"
                  >
                    <option value="Monday">Monday</option>
                    <option value="Tuesday">Tuesday</option>
                    <option value="Wednesday">Wednesday</option>
                    <option value="Thursday">Thursday</option>
                    <option value="Friday">Friday</option>
                  </select>
                  <span className="field-hint">Day for midweek matches</span>
                </div>

                <div className="config-field">
                  <label>Midweek Kickoff Time</label>
                  <input
                    type="time"
                    value={config.midweek_kickoff_time || '19:30'}
                    onChange={(e) => updateConfig('midweek_kickoff_time', e.target.value)}
                    className="config-input"
                  />
                  <span className="field-hint">Evening kickoff time</span>
                </div>
              </>
            )}
          </div>
        </div>

        {/* Weather and Cancellations */}
        <div className="config-section">
          <h3 className="section-title">Weather & Cancellations</h3>
          <div className="config-grid">
            <div className="config-field checkbox-field">
              <label className="checkbox-label">
                <input
                  type="checkbox"
                  checked={config.enable_weather_cancellations}
                  onChange={(e) => updateConfig('enable_weather_cancellations', e.target.checked)}
                  className="config-checkbox"
                />
                <span>Enable Weather Cancellations</span>
              </label>
              <span className="field-hint">Matches can be cancelled due to weather</span>
            </div>

            {config.enable_weather_cancellations && (
              <>
                <div className="config-field">
                  <label>Cancellation Threshold</label>
                  <input
                    type="range"
                    min="1"
                    max="10"
                    value={config.cancellation_threshold}
                    onChange={(e) => updateConfig('cancellation_threshold', parseInt(e.target.value))}
                    className="config-range"
                  />
                  <span className="range-value">{config.cancellation_threshold} / 10</span>
                  <span className="field-hint">Weather severity required to cancel (higher = more strict)</span>
                </div>

                <div className="config-field">
                  <label>Rearrangement Window (weeks)</label>
                  <input
                    type="number"
                    min="1"
                    max="12"
                    value={config.rearrangement_window_weeks}
                    onChange={(e) => updateConfig('rearrangement_window_weeks', parseInt(e.target.value))}
                    className="config-input"
                  />
                  <span className="field-hint">How many weeks to reschedule within</span>
                </div>

                <div className="config-field checkbox-field">
                  <label className="checkbox-label">
                    <input
                      type="checkbox"
                      checked={config.priority_rearrangement}
                      onChange={(e) => updateConfig('priority_rearrangement', e.target.checked)}
                      className="config-checkbox"
                    />
                    <span>Priority Rearrangement</span>
                  </label>
                  <span className="field-hint">Reschedule ASAP vs end of season</span>
                </div>
              </>
            )}
          </div>
        </div>

        {/* Promotion Playoffs */}
        <div className="config-section">
          <h3 className="section-title">Promotion Playoffs</h3>
          <div className="config-grid">
            <div className="config-field checkbox-field">
              <label className="checkbox-label">
                <input
                  type="checkbox"
                  checked={config.enable_promotion_playoffs}
                  onChange={(e) => updateConfig('enable_promotion_playoffs', e.target.checked)}
                  className="config-checkbox"
                />
                <span>Enable Promotion Playoffs</span>
              </label>
              <span className="field-hint">Teams compete for promotion places</span>
            </div>

            {config.enable_promotion_playoffs && (
              <>
                <div className="config-field">
                  <label>Teams per Division</label>
                  <input
                    type="number"
                    min="2"
                    max="8"
                    value={config.playoff_teams_per_division}
                    onChange={(e) => updateConfig('playoff_teams_per_division', parseInt(e.target.value))}
                    className="config-input"
                  />
                  <span className="field-hint">Number of teams in playoff</span>
                </div>

                <div className="config-field">
                  <label>Playoff Format</label>
                  <select
                    value={config.playoff_format}
                    onChange={(e) => updateConfig('playoff_format', e.target.value)}
                    className="config-select"
                  >
                    <option value="single_leg">Single Leg</option>
                    <option value="two_leg_home_away">Two Legs (Home & Away)</option>
                    <option value="neutral_venue">Neutral Venue</option>
                  </select>
                  <span className="field-hint">How playoff matches are played</span>
                </div>
              </>
            )}
          </div>
        </div>

        {/* Relegation Playoffs */}
        <div className="config-section">
          <h3 className="section-title">Relegation Playoffs</h3>
          <div className="config-grid">
            <div className="config-field checkbox-field">
              <label className="checkbox-label">
                <input
                  type="checkbox"
                  checked={config.enable_relegation_playoffs}
                  onChange={(e) => updateConfig('enable_relegation_playoffs', e.target.checked)}
                  className="config-checkbox"
                />
                <span>Enable Relegation Playoffs</span>
              </label>
              <span className="field-hint">Teams compete to avoid relegation</span>
            </div>

            {config.enable_relegation_playoffs && (
              <div className="config-field">
                <label>Playoff Teams</label>
                <input
                  type="number"
                  min="2"
                  max="6"
                  value={config.relegation_playoff_teams}
                  onChange={(e) => updateConfig('relegation_playoff_teams', parseInt(e.target.value))}
                  className="config-input"
                />
                <span className="field-hint">Teams competing in relegation playoff</span>
              </div>
            )}
          </div>
        </div>
      </div>

      {/* Save/Reset Actions */}
      {hasChanges && (
        <div className="config-actions">
          <button onClick={handleSave} className="btn-save" disabled={saving}>
            {saving ? 'Saving...' : 'Save Configuration'}
          </button>
          <button onClick={handleReset} className="btn-reset" disabled={saving}>
            Reset Changes
          </button>
        </div>
      )}
    </div>
  );
}
