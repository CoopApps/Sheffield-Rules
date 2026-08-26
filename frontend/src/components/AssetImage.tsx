import React, { useState } from 'react'

interface AssetImageProps {
  pack: string
  category: string
  assetId: string
  alt: string
  className?: string
  style?: React.CSSProperties
}

export const AssetImage: React.FC<AssetImageProps> = ({
  pack,
  category,
  assetId,
  alt,
  className = '',
  style,
}) => {
  const [isLoading, setIsLoading] = useState(true)
  const [error, setError] = useState(false)

  const imagePath = `/api/assets/${pack}/${category}/${assetId}`

  return (
    <div className={`asset-image-container ${className}`} style={style}>
      {isLoading && <div className="image-loader">Loading...</div>}
      {error && <div className="image-error">Image not found</div>}
      <img
        src={imagePath}
        alt={alt}
        className="asset-image"
        onLoad={() => setIsLoading(false)}
        onError={() => {
          setIsLoading(false)
          setError(true)
        }}
        style={{ display: isLoading || error ? 'none' : 'block' }}
      />
    </div>
  )
}
