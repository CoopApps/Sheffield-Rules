import React, { createContext } from 'react'

// AssetContext is no longer needed since we use Tauri + GameState
// This is kept for backward compatibility but not actively used

export const AssetContext = createContext<{
  loading: boolean
}>({
  loading: false,
})

export const AssetProvider: React.FC<{ children: React.ReactNode }> = ({ children }) => {
  return (
    <AssetContext.Provider value={{ loading: false }}>
      {children}
    </AssetContext.Provider>
  )
}
