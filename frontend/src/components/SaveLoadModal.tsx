import React, { useState, useEffect } from 'react'
import { invoke } from '../utils/tauriInvoke'
import '../styles/SaveLoadModal.css'

interface SaveLoadModalProps {
  gameState: any
  onClose: () => void
  onSave: (updatedGame: any) => void
  onLoad: (loadedGame: any) => void
  mode: 'save' | 'load'
}

export function SaveLoadModal({ gameState, onClose, onSave, onLoad, mode }: SaveLoadModalProps) {
  const [saves, setSaves] = useState<string[]>([])
  const [saveName, setSaveName] = useState('')
  const [isLoading, setIsLoading] = useState(false)
  const [error, setError] = useState<string | null>(null)
  const [showDeleteConfirm, setShowDeleteConfirm] = useState<string | null>(null)
  const [step, setStep] = useState<'list' | 'input'>('list')

  useEffect(() => {
    loadSaveList()
  }, [])

  async function loadSaveList() {
    try {
      setIsLoading(true)
      const saveList: string[] = await invoke('get_save_list')
      setSaves(saveList)
      setError(null)
    } catch (err) {
      console.error('Failed to load save list:', err)
      setError('Failed to load save list')
    } finally {
      setIsLoading(false)
    }
  }

  async function handleSaveGame(name: string) {
    try {
      setIsLoading(true)
      setError(null)
      const updatedGame = await invoke('save_game_as', {
        game: gameState,
        saveName: name,
      })
      onSave(updatedGame)
      onClose()
    } catch (err) {
      console.error('Failed to save game:', err)
      setError(`Failed to save game: ${err}`)
    } finally {
      setIsLoading(false)
    }
  }

  async function handleLoadGame(name: string) {
    try {
      setIsLoading(true)
      setError(null)
      const loadedGame = await invoke('load_game_by_name', { saveName: name })
      onLoad(loadedGame)
      onClose()
    } catch (err) {
      console.error('Failed to load game:', err)
      setError(`Failed to load game: ${err}`)
    } finally {
      setIsLoading(false)
    }
  }

  async function handleDeleteSave(name: string) {
    try {
      setIsLoading(true)
      setError(null)

      // Delete the save file via the backend
      await invoke('delete_save_file', { saveName: name })

      // Reload the save list
      await loadSaveList()
      setShowDeleteConfirm(null)
    } catch (err) {
      console.error('Failed to delete save:', err)
      setError(`Failed to delete save: ${err}`)
    } finally {
      setIsLoading(false)
    }
  }

  if (mode === 'save') {
    if (step === 'input') {
      return (
        <div className="modal-overlay">
          <div className="modal-content save-load-modal">
            <h2>Save Game</h2>

            {error && <div className="error-message">{error}</div>}

            <div className="input-section">
              <label htmlFor="save-name">Save Name:</label>
              <input
                id="save-name"
                type="text"
                value={saveName}
                onChange={(e) => setSaveName(e.target.value)}
                placeholder="Enter a name for this save..."
                disabled={isLoading}
                onKeyPress={(e) => {
                  if (e.key === 'Enter' && saveName.trim()) {
                    handleSaveGame(saveName.trim())
                  }
                }}
              />
              <p className="hint">
                Tip: Use descriptive names like "Season 1 GW20" or "Before Championship"
              </p>
            </div>

            <div className="modal-footer">
              <button
                className="btn btn-secondary"
                onClick={() => {
                  setSaveName('')
                  setStep('list')
                }}
                disabled={isLoading}
              >
                Back
              </button>
              <button
                className="btn btn-primary"
                onClick={() => {
                  if (saveName.trim()) {
                    handleSaveGame(saveName.trim())
                  }
                }}
                disabled={isLoading || !saveName.trim()}
              >
                {isLoading ? 'Saving...' : 'Save'}
              </button>
            </div>
          </div>
        </div>
      )
    }

    return (
      <div className="modal-overlay">
        <div className="modal-content save-load-modal">
          <h2>Save Game</h2>

          <p className="modal-description">
            Enter a name for your save or overwrite an existing one.
          </p>

          {error && <div className="error-message">{error}</div>}

          {isLoading ? (
            <div className="loading">Loading saves...</div>
          ) : (
            <>
              <div className="saves-list">
                <div className="save-item new-save" onClick={() => setStep('input')}>
                  <span className="save-name">+ Create New Save</span>
                  <span className="save-arrow">→</span>
                </div>

                {saves.map((save) => (
                  <div key={save} className="save-item" onClick={() => setSaveName(save)}>
                    <div className="save-info">
                      <span className="save-name">{save}</span>
                    </div>
                    <button
                      className="btn-small btn-overwrite"
                      onClick={(e) => {
                        e.stopPropagation()
                        setSaveName(save)
                        handleSaveGame(save)
                      }}
                      disabled={isLoading}
                    >
                      Overwrite
                    </button>
                  </div>
                ))}
              </div>

              {saves.length === 0 && (
                <p className="no-saves">No saves yet. Create a new one!</p>
              )}
            </>
          )}

          <div className="modal-footer">
            <button className="btn btn-secondary" onClick={onClose} disabled={isLoading}>
              Cancel
            </button>
          </div>
        </div>
      </div>
    )
  }

  // Load mode
  return (
    <div className="modal-overlay">
      <div className="modal-content save-load-modal">
        <h2>Load Game</h2>

        <p className="modal-description">Select a save file to load.</p>

        {error && <div className="error-message">{error}</div>}

        {isLoading ? (
          <div className="loading">Loading saves...</div>
        ) : (
          <>
            <div className="saves-list">
              {saves.map((save) => (
                <div key={save} className="save-item">
                  <div className="save-info">
                    <span className="save-name">{save}</span>
                  </div>
                  <div className="save-actions">
                    <button
                      className="btn-small btn-load"
                      onClick={() => handleLoadGame(save)}
                      disabled={isLoading}
                    >
                      Load
                    </button>
                    <button
                      className="btn-small btn-delete"
                      onClick={() => setShowDeleteConfirm(save)}
                      disabled={isLoading}
                      title="Delete this save"
                    >
                      ✕
                    </button>
                  </div>
                </div>
              ))}
            </div>

            {saves.length === 0 && (
              <p className="no-saves">No saves found. Create a new game and save it!</p>
            )}
          </>
        )}

        <div className="modal-footer">
          <button className="btn btn-secondary" onClick={onClose} disabled={isLoading}>
            Cancel
          </button>
        </div>
      </div>

      {/* Delete confirmation dialog */}
      {showDeleteConfirm && (
        <div className="modal-overlay">
          <div className="modal-content confirmation-modal">
            <h3>Delete Save?</h3>
            <p>Are you sure you want to delete the save "{showDeleteConfirm}"?</p>
            <p className="warning">This action cannot be undone.</p>

            <div className="modal-footer">
              <button
                className="btn btn-secondary"
                onClick={() => setShowDeleteConfirm(null)}
                disabled={isLoading}
              >
                Cancel
              </button>
              <button
                className="btn btn-danger"
                onClick={() => handleDeleteSave(showDeleteConfirm)}
                disabled={isLoading}
              >
                {isLoading ? 'Deleting...' : 'Delete'}
              </button>
            </div>
          </div>
        </div>
      )}
    </div>
  )
}
