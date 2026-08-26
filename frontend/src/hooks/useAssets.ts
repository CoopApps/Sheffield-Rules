import { useContext } from 'react'
import { AssetContext } from '../contexts/AssetContext'

export const useAssets = () => {
  const context = useContext(AssetContext)
  if (!context) {
    throw new Error('useAssets must be used within AssetProvider')
  }
  return context
}
