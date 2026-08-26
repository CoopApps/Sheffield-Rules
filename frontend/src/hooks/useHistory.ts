import { useState, useCallback } from 'react'

interface HistoryState<T> {
  past: T[]
  present: T
  future: T[]
}

interface UseHistoryReturn<T> {
  state: T
  setState: (newState: T | ((prev: T) => T)) => void
  undo: () => void
  redo: () => void
  canUndo: boolean
  canRedo: boolean
  clearHistory: () => void
}

const MAX_HISTORY = 50
const MAX_MEMORY_MB = 50

export function useHistory<T>(initialState: T): UseHistoryReturn<T> {
  const [history, setHistory] = useState<HistoryState<T>>({
    past: [],
    present: initialState,
    future: [],
  })

  const setState = useCallback((newState: T | ((prev: T) => T)) => {
    setHistory((prev) => {
      const nextPresent = typeof newState === 'function' ? (newState as Function)(prev.present) : newState

      // Check if state actually changed
      if (JSON.stringify(nextPresent) === JSON.stringify(prev.present)) {
        return prev
      }

      // Limit history size
      const newPast = [prev.present, ...prev.past].slice(0, MAX_HISTORY)

      return {
        past: newPast,
        present: nextPresent,
        future: [], // Clear future on new change
      }
    })
  }, [])

  const undo = useCallback(() => {
    setHistory((prev) => {
      if (prev.past.length === 0) return prev

      const newPast = prev.past.slice(1)
      const newPresent = prev.past[0]
      const newFuture = [prev.present, ...prev.future]

      return {
        past: newPast,
        present: newPresent,
        future: newFuture,
      }
    })
  }, [])

  const redo = useCallback(() => {
    setHistory((prev) => {
      if (prev.future.length === 0) return prev

      const newFuture = prev.future.slice(1)
      const newPresent = prev.future[0]
      const newPast = [prev.present, ...prev.past]

      return {
        past: newPast,
        present: newPresent,
        future: newFuture,
      }
    })
  }, [])

  const clearHistory = useCallback(() => {
    setHistory({
      past: [],
      present: history.present,
      future: [],
    })
  }, [history.present])

  return {
    state: history.present,
    setState,
    undo,
    redo,
    canUndo: history.past.length > 0,
    canRedo: history.future.length > 0,
    clearHistory,
  }
}
