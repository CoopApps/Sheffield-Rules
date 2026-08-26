import React, { useState } from 'react'

interface EncyclopediaScreenProps {
  onClose: () => void
}

export function EncyclopediaScreen({ onClose }: EncyclopediaScreenProps) {
  const [selectedCategory, setSelectedCategory] = useState<string | null>(null)

  const categories = [
    {
      id: 'history',
      title: 'Football History',
      description: 'Learn about the evolution of football from 1858 onwards',
      icon: '📚',
    },
    {
      id: 'sheffield-rules',
      title: 'Sheffield Rules',
      description: 'Understanding the unique rules of Sheffield Football',
      icon: '📋',
    },
    {
      id: 'clubs',
      title: 'Sheffield Clubs',
      description: 'Information about the clubs competing in Sheffield',
      icon: '🏆',
    },
    {
      id: 'players',
      title: 'Player Attributes',
      description: 'Understanding player statistics and attributes',
      icon: '👥',
    },
    {
      id: 'tactics',
      title: 'Tactics & Strategy',
      description: 'Guide to team tactics and strategic gameplay',
      icon: '⚽',
    },
    {
      id: 'gameplay',
      title: 'Gameplay Guide',
      description: 'How to play and manage your team',
      icon: '🎮',
    },
  ]

  const categoryContent: Record<string, React.ReactNode> = {
    history: (
      <div className="encyclopedia-content">
        <h2>Football History</h2>
        <p>
          Football as we know it today has its roots in Sheffield, England. The period from 1858 to 1877 represents
          a crucial evolutionary phase in the sport's development.
        </p>
        <h3>Early Era (1858-1862)</h3>
        <p>
          The earliest form of organized football featured several variations in rules between different regions.
          Sheffield football was played with unique rules that would eventually influence modern football.
        </p>
        <h3>Rouge Era (1862-1868)</h3>
        <p>
          This period introduced the revolutionary "rouge" scoring system, where a ball kicked between the uprights
          below the crossbar counted as one point, while a goal (above the crossbar) was worth two points. This system
          encouraged a more dynamic form of play.
        </p>
        <h3>Post-Rouge Era (1868-1877)</h3>
        <p>
          As football evolved, the rouge system was gradually abolished. Sheffield rules began to converge with FA
          rules, eventually merging completely by 1877, establishing the foundation for modern association football.
        </p>
      </div>
    ),
    'sheffield-rules': (
      <div className="encyclopedia-content">
        <h2>Sheffield Rules</h2>
        <p>
          Sheffield Rules represent a unique chapter in football history. These rules governed how the game was played
          in Sheffield and influenced the development of modern football.
        </p>
        <h3>Key Features</h3>
        <ul>
          <li>11 players per side</li>
          <li>Rectangular field with goals at each end</li>
          <li>The innovative rouge scoring system (1862-1868)</li>
          <li>Emphasis on skill and tactical play</li>
        </ul>
        <h3>The Rouge System</h3>
        <p>
          The rouge (also spelled "rouges") was a unique scoring method where:
        </p>
        <ul>
          <li>A rouge: 1 point (kick between uprights below crossbar)</li>
          <li>A goal: 2 points (kick above the crossbar)</li>
        </ul>
        <p>
          This created an interesting strategic dynamic, as teams could score valuable points even without scoring a
          full goal.
        </p>
      </div>
    ),
    clubs: (
      <div className="encyclopedia-content">
        <h2>Sheffield Clubs</h2>
        <p>
          Sheffield has a rich tradition of football clubs, many of which were founded during the period you'll be
          managing. Learn about the clubs competing in your games.
        </p>
        <h3>Club Information</h3>
        <p>Each club has:</p>
        <ul>
          <li><strong>Founding Year:</strong> When the club was established</li>
          <li><strong>Ground:</strong> The stadium or pitch where they play</li>
          <li><strong>City/Region:</strong> Their location within Sheffield</li>
          <li><strong>Squad:</strong> The players who compete for the club</li>
        </ul>
        <h3>Managing a Club</h3>
        <p>
          When you manage a Sheffield club, you'll have access to information about all other clubs in the league,
          including their current standings, squads, and performance records.
        </p>
      </div>
    ),
    players: (
      <div className="encyclopedia-content">
        <h2>Player Attributes</h2>
        <p>
          Each player in the game has a range of attributes that determine their abilities in different areas of
          football.
        </p>
        <h3>Physical Attributes</h3>
        <ul>
          <li><strong>Pace:</strong> Speed and acceleration</li>
          <li><strong>Strength:</strong> Physical power</li>
          <li><strong>Stamina:</strong> Endurance throughout matches</li>
          <li><strong>Balance:</strong> Stability and coordination</li>
          <li><strong>Jumping:</strong> Ability to reach high balls</li>
          <li><strong>Agility:</strong> Quickness and flexibility</li>
        </ul>
        <h3>Technical Attributes</h3>
        <ul>
          <li><strong>Passing:</strong> Accuracy of passes</li>
          <li><strong>Dribbling:</strong> Ball control ability</li>
          <li><strong>Heading:</strong> Ability to win aerial duels</li>
          <li><strong>Crossing:</strong> Accuracy of crosses</li>
          <li><strong>Tackling:</strong> Defensive ability</li>
        </ul>
        <h3>Mental Attributes</h3>
        <ul>
          <li><strong>Courage:</strong> Bravery in challenges</li>
          <li><strong>Concentration:</strong> Focus during play</li>
          <li><strong>Leadership:</strong> Ability to inspire teammates</li>
          <li><strong>Aggression:</strong> Intensity of play</li>
          <li><strong>Determination:</strong> Will to win</li>
        </ul>
        <p>All attributes are rated on a scale of 1-20.</p>
      </div>
    ),
    tactics: (
      <div className="encyclopedia-content">
        <h2>Tactics & Strategy</h2>
        <p>
          Successful management requires understanding tactical formations and strategic approaches to the game.
        </p>
        <h3>Key Tactical Concepts</h3>
        <ul>
          <li><strong>Formation:</strong> How your players are arranged on the field</li>
          <li><strong>Mentality:</strong> Your team's aggressive or defensive approach</li>
          <li><strong>Player Positioning:</strong> Where each player operates</li>
          <li><strong>Set Pieces:</strong> Corners, free kicks, and throw-ins</li>
        </ul>
        <h3>Building a Strong Team</h3>
        <p>
          Balance your squad with players of different strengths. Mix experienced veterans with promising young talent.
          Ensure your formation matches the abilities of your available players.
        </p>
      </div>
    ),
    gameplay: (
      <div className="encyclopedia-content">
        <h2>Gameplay Guide</h2>
        <p>
          Master the essential features of Saturday at Three to manage your team effectively.
        </p>
        <h3>Main Screens</h3>
        <ul>
          <li><strong>Squad:</strong> View and manage your players</li>
          <li><strong>Clubs:</strong> Scout other teams in the league</li>
          <li><strong>Tables:</strong> Check current league standings</li>
          <li><strong>Tactics:</strong> Set your team formation and strategy</li>
          <li><strong>Training:</strong> Develop your players' abilities</li>
        </ul>
        <h3>Game Progression</h3>
        <p>
          Click "Continue" to advance through the season day by day. Matches will occur on scheduled dates. Monitor
          your league position and work to improve your team's performance over time.
        </p>
        <h3>Tips for Success</h3>
        <ul>
          <li>Build a balanced squad with good depth</li>
          <li>Monitor player form and rotate your lineup</li>
          <li>Study your opponents in the Clubs section</li>
          <li>Save your game regularly</li>
        </ul>
      </div>
    ),
  }

  return (
    <div className="encyclopedia-modal-overlay" onClick={onClose}>
      <div className="encyclopedia-modal" onClick={(e) => e.stopPropagation()}>
        <div className="encyclopedia-header">
          <h1>Encyclopedia</h1>
          <button className="encyclopedia-close" onClick={onClose}>✕</button>
        </div>

        <div className="encyclopedia-container">
          {/* Categories sidebar */}
          <div className="encyclopedia-sidebar">
            <div className="encyclopedia-categories">
              {categories.map((cat) => (
                <button
                  key={cat.id}
                  className={`encyclopedia-category ${selectedCategory === cat.id ? 'selected' : ''}`}
                  onClick={() => setSelectedCategory(cat.id)}
                >
                  <span className="category-icon">{cat.icon}</span>
                  <div className="category-text">
                    <div className="category-title">{cat.title}</div>
                    <div className="category-desc">{cat.description}</div>
                  </div>
                </button>
              ))}
            </div>
          </div>

          {/* Content area */}
          <div className="encyclopedia-content-area">
            {selectedCategory ? (
              categoryContent[selectedCategory]
            ) : (
              <div className="encyclopedia-content">
                <h2>Welcome to the Encyclopedia</h2>
                <p>
                  Select a category from the left to learn more about football history, Sheffield Rules, clubs,
                  player attributes, tactics, and gameplay mechanics.
                </p>
                <p>
                  This comprehensive guide will help you understand the game and manage your team effectively through
                  the historic 1858-1877 period of Sheffield football.
                </p>
              </div>
            )}
          </div>
        </div>
      </div>
    </div>
  )
}

export default EncyclopediaScreen
