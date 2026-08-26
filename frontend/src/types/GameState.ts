export interface GameState {
  id: string
  season: number
  currentGameweek: number
  currentDate: string  // ISO 8601 format (YYYY-MM-DD)
  userClubId: string
  clubs: Club[]
  matches: Match[]
  players: Player[]
  standings: Standing[]
  pendingEvents: GameEvent[]
  processedEvents: GameEvent[]
  createdAt: string
  updatedAt: string

  // Sheffield Rules specific fields
  gameMode?: string  // historical-timeline, ahistorical-1862, etc.
  ruleYear?: number  // For ahistorical modes, the locked rule year
  startYear?: number  // The initial year selected
}

export interface Club {
  id: string
  name: string
  points: number
  played: number
  won: number
  drawn: number
  lost: number
  goalsFor: number
  goalsAgainst: number
  budget: number
  reputation: number
}

export interface Match {
  id: string
  gameweek: number
  homeTeamId: string
  awayTeamId: string
  homeScore: number
  awayScore: number
  date: string
  played: boolean

  // Sheffield Rules specific (1862-1868 rouge era)
  homeRouges?: number
  awayRouges?: number
}

export interface Player {
  id: string
  name: string
  clubId: string
  position: string
  age: number
  overallRating: number
  form: number
  fitness: number
  morale: number
}

export interface Standing {
  clubId: string
  clubName: string
  position: number
  points: number
  played: number
  won: number
  drawn: number
  lost: number
  goalsFor: number
  goalsAgainst: number
  goalDifference: number

  // Sheffield Rules specific (1862-1868 rouge era)
  rougesFor?: number
  rougesAgainst?: number
}

export interface GameEvent {
  id: string
  eventType: EventType
  date: string
  requiresUserAction: boolean
  processed: boolean
  result: EventResult | null
}

export type EventType =
  | { type: 'Match'; data: { matchId: string; isUserTeam: boolean } }
  | { type: 'MediaInquiry'; data: { question: string; options: string[] } }
  | { type: 'PlayerNegotiation'; data: { playerId: string; clubId: string; offerType: string } }
  | { type: 'HistoricalAnnouncement'; data: { title: string; description: string } }
  | { type: 'CupAnnouncement'; data: { cupName: string; competitionId: string; title: string; description: string; eligibleDivisions: string[] } }
  | { type: 'CupDraw'; data: { cupName: string; competitionId: string; title: string; description: string; drawBracket: any[] } }
  | { type: 'challenge_sent'; data: { headline: string; body: string; hasActionButton: boolean; actionButtonText: string | null; actionType: string | null; actionData: string | null; isImportant: boolean; relatedInvitationId: string | null } }
  | { type: 'challenge_accepted'; data: { headline: string; body: string; hasActionButton: boolean; actionButtonText: string | null; actionType: string | null; actionData: string | null; isImportant: boolean; relatedInvitationId: string | null } }
  | { type: 'challenge_declined'; data: { headline: string; body: string; hasActionButton: boolean; actionButtonText: string | null; actionType: string | null; actionData: string | null; isImportant: boolean; relatedInvitationId: string | null } }
  | { type: 'challenge_received'; data: { headline: string; body: string; hasActionButton: boolean; actionButtonText: string | null; actionType: string | null; actionData: string | null; isImportant: boolean; relatedInvitationId: string | null } }
  | { type: 'match_scheduled'; data: { headline: string; body: string; hasActionButton: boolean; actionButtonText: string | null; actionType: string | null; actionData: string | null; isImportant: boolean; relatedInvitationId: string | null } }

export interface EventResult {
  eventId: string
  summary: string
  data: any
}

export interface DayProcessingResult {
  autoProcessed: EventResult[]
  requireUserAction: GameEvent[]
  allComplete: boolean
}

export const initialGameState: GameState = {
  id: '',
  season: 1888,
  currentGameweek: 1,
  currentDate: '1888-04-10',  // One week before Football League announcement (April 17, 1888)
  userClubId: 'accrington',
  clubs: [],
  matches: [],
  players: [],
  standings: [],
  pendingEvents: [],
  processedEvents: [],
  createdAt: new Date().toISOString(),
  updatedAt: new Date().toISOString(),
}
