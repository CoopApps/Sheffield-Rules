import React, { useState } from 'react'
import { SheffieldYearSelector } from '../components/SheffieldYearSelector'
import '../styles/sheffield-rules-setup.css'

interface SheffieldRulesSetupScreenProps {
  onGameModeSelected: (gameMode: string, year?: number) => void
  onBack: () => void
  onTestGameplayV2?: (year: number, gameMode: string) => void
}

export function SheffieldRulesSetupScreen({ onGameModeSelected, onBack, onTestGameplayV2 }: SheffieldRulesSetupScreenProps) {
  const [selectedYear, setSelectedYear] = useState<number>(1867)
  const [selectedGameMode, setSelectedGameMode] = useState<string>('')

  const handleYearSelected = (year: number) => {
    setSelectedYear(year)
  }

  const handleGameModeSelected = (gameMode: 'historical-timeline' | 'ahistorical' | 'ahistorical-1860' | 'ahistorical-1862' | 'ahistorical-1868' | 'ahistorical-1875' | 'historical-from-year' | 'sheffield-hallamshire-league', year?: number) => {
    const finalYear = year || selectedYear
    setSelectedGameMode(gameMode)
    onGameModeSelected(gameMode, finalYear)
  }

  const handleTestV2 = () => {
    if (onTestGameplayV2) {
      onTestGameplayV2(selectedYear, selectedGameMode || 'historical-timeline')
    }
  }

  return (
    <div className="sheffield-rules-setup-screen">
      <div className="setup-background"></div>

      <div className="setup-content">
        <SheffieldYearSelector
          onSelectYear={handleYearSelected}
          onSelectGameMode={handleGameModeSelected}
          onBack={onBack}
        />
      </div>
    </div>
  )
}

export default SheffieldRulesSetupScreen
