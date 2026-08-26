import React, { useState, useEffect, useRef } from 'react';
import { invoke } from '../utils/tauriInvoke';
import '../styles/database-editor.css';
import LeagueManagementScreen from './LeagueManagementScreen';
import { CupManagement } from '../components/CupManagement';
import { CsvImportButton } from '../components/CsvImportButton';
import { ConfirmationDialog } from '../components/ConfirmationDialog';

interface Club {
  id: string;
  name: string;
  founded_year: number;
  ground_name: string;
  city: string;
  region: string;
  origin: string;
  where_from?: string;
  reserve_of_club_id?: string;
  parent_club_name?: string;
  additional_postcode?: string;
}

interface Division {
  id: string;
  name: string;
  level: number;
  region: string | null;
}

interface ClubDivisionInfo {
  division_id: string;
  division_name: string;
  position: number;
}

interface ClubWithDivision extends Club {
  division_name?: string;
}

interface GeneratedPlayer {
  id: string;
  name: string;
  club_id: string;
  position: string;
  birth_year: number;
  nationality: string;
  height_cm: number;
  overall_rating: number;
  pace: number;
  strength: number;
  stamina: number;
  passing: number;
  dribbling: number;
  finishing: number;
  tackling: number;
  handling: number;
  reflexes: number;
}

interface Player {
  id: string;
  name: string;
  club_id: string;
  position: string;
  birth_year: number;
  age: number;
  nationality: string;
  // Physical
  pace: number;
  acceleration: number;
  strength: number;
  stamina: number;
  balance: number;
  jumping: number;
  agility: number;
  natural_fitness: number;
  // Technical
  passing: number;
  dribbling: number;
  first_touch: number;
  technique: number;
  heading: number;
  long_passing: number;
  crossing: number;
  long_shots: number;
  tackling: number;
  handling: number;
  reflexes: number;
  corners: number;
  free_kicks: number;
  throw_ins: number;
  vision: number;
  left_foot: number;
  right_foot: number;
  one_on_ones: number;
  // Mental
  courage: number;
  bravery: number;
  concentration: number;
  decision_making: number;
  leadership: number;
  aggression: number;
  anticipation: number;
  determination: number;
  flair: number;
  influence: number;
  adaptability: number;
  ambition: number;
  loyalty: number;
  pressure: number;
  professionalism: number;
  sportsmanship: number;
  temperament: number;
  // Positioning
  awareness: number;
  marking: number;
  positioning: number;
  work_rate: number;
  off_the_ball: number;
  movement: number;
  teamwork: number;
  // Specialization
  finishing: number;
  penalties: number;
  set_pieces: number;
  // Hidden
  consistency: number;
  dirtiness: number;
  versatility: number;
  injury_proneness: number;
  important_matches: number;
  // Ability & Reputation
  current_ability: number;
  potential_ability: number;
  current_reputation: number;
}

interface Competition {
  id: string;
  name: string;
  competition_type: string;
  season: number;
  min_division_level: number | null;
  max_division_level: number | null;
  current_round: number;
  total_rounds: number;
  is_active: boolean;
  winner_club_id: string | null;
  runner_up_club_id: string | null;
  rules_type: string;
  prestige_level: string;
  start_week: number;
}

interface CupTie {
  id: string;
  competition_id: string;
  round_number: number;
  round_name: string;
  tie_number: number;
  home_club_id: string | null;
  away_club_id: string | null;
  scheduled_week: number;
  played: boolean;
  home_score: number | null;
  away_score: number | null;
  winner_club_id: string | null;
  match_id: string | null;
}

interface EligibleClub {
  id: string;
  name: string;
  division_level: number;
  division_name: string;
}

type TabType = 'clubs' | 'players' | 'squads' | 'leagues' | 'cups' | 'create-players' | 'assign-stats' | 'assign-clubs' | 'player-database' | 'parishes' | 'club-capacity';

const DatabaseEditorScreen: React.FC = () => {
  const [activeTab, setActiveTab] = useState<TabType>('clubs');
  const [clubs, setClubs] = useState<Club[]>([]);
  const [filteredClubs, setFilteredClubs] = useState<Club[]>([]);
  const [selectedClubId, setSelectedClubId] = useState<string>('');
  const [searchText, setSearchText] = useState<string>('');
  const [filterPostcode, setFilterPostcode] = useState<string>('');
  const [filterArea, setFilterArea] = useState<string>('');
  const [hideReserves, setHideReserves] = useState<boolean>(true);
  const [hideNonReserves, setHideNonReserves] = useState<boolean>(false);

  // Player generation state
  const [year, setYear] = useState<number>(1867);
  const [hasGoalkeeper, setHasGoalkeeper] = useState<boolean>(true);
  const [playerNamesText, setPlayerNamesText] = useState<string>('');
  const [generatedPlayers, setGeneratedPlayers] = useState<GeneratedPlayer[]>([]);

  // Multi-club selection for player generation
  const [selectedClubIds, setSelectedClubIds] = useState<string[]>([]);
  const [playerGenPostcode, setPlayerGenPostcode] = useState<string>('');
  const [defaultParish, setDefaultParish] = useState<string>('');

  // Table-based player entry with geographic data
  const [manualPlayers, setManualPlayers] = useState<Array<{
    id: string;
    name: string;
    position: string;
    birthYear: number;
    age?: number;
    relation?: string;
    gender?: string;
    nationality: string;
    whereBorn?: string;
    birthTown?: string;
    birthCounty?: string;
    birthCountry?: string;
    civilParish?: string;
    ecclesiasticalParish?: string;
    registrationDistrict?: string;
    subRegistrationDistrict?: string;
    edInstitution?: string;
    householdScheduleNumber?: string;
    piece?: string;
    folio?: string;
    pageNumber?: string;
  }>>([]);

  // Name editing
  const [editingName, setEditingName] = useState<boolean>(false);
  const [editName, setEditName] = useState<string>('');

  // Location editing
  const [editingLocation, setEditingLocation] = useState<boolean>(false);
  const [editCity, setEditCity] = useState<string>('');
  const [editRegion, setEditRegion] = useState<string>('');
  const [editOrigin, setEditOrigin] = useState<string>('');

  // Division information
  const [divisions, setDivisions] = useState<Division[]>([]);
  const [clubDivisionInfo, setClubDivisionInfo] = useState<ClubDivisionInfo | null>(null);
  const [editingDivision, setEditingDivision] = useState<boolean>(false);
  const [editDivisionId, setEditDivisionId] = useState<string>('');
  const [editPosition, setEditPosition] = useState<number>(1);

  // Create new club
  const [creatingClub, setCreatingClub] = useState<boolean>(false);
  const [newClubName, setNewClubName] = useState<string>('');
  const [newClubFoundedYear, setNewClubFoundedYear] = useState<number>(1867);
  const [newClubGroundName, setNewClubGroundName] = useState<string>('Unknown');
  const [newClubCity, setNewClubCity] = useState<string>('');
  const [newClubRegion, setNewClubRegion] = useState<string>('');
  const [newClubOrigin, setNewClubOrigin] = useState<string>('');

  // Squad Management
  const [squadClubs, setSquadClubs] = useState<ClubWithDivision[]>([]);
  const [selectedSquadClubId, setSelectedSquadClubId] = useState<string>('');
  const [squadPlayers, setSquadPlayers] = useState<Player[]>([]);
  const [selectedPlayerId, setSelectedPlayerId] = useState<string | null>(null);
  const [editingPlayer, setEditingPlayer] = useState<Player | null>(null);

  // Cup Management
  const [cupSeason, setCupSeason] = useState<number>(1867);
  const [cupStartWeek, setCupStartWeek] = useState<number>(3);
  const [competitions, setCompetitions] = useState<Competition[]>([]);
  const [selectedCompetitionId, setSelectedCompetitionId] = useState<string>('');
  const [eligibleClubs, setEligibleClubs] = useState<EligibleClub[]>([]);
  const [cupBracket, setCupBracket] = useState<CupTie[]>([]);
  const [cupSeeded, setCupSeeded] = useState<boolean>(false);

  const [loading, setLoading] = useState<boolean>(false);
  const [error, setError] = useState<string>('');
  const [success, setSuccess] = useState<string>('');
  const [progressMessage, setProgressMessage] = useState<string>('');
  const [startFromClub, setStartFromClub] = useState<number>(1);

  // Players without stats (for Assign Stats tab)
  const [playersWithoutStats, setPlayersWithoutStats] = useState<any[]>([]);
  const [selectedPlayerForStats, setSelectedPlayerForStats] = useState<any | null>(null);

  // All players (for Player Database tab) - NOW WITH PAGINATION
  const [allPlayers, setAllPlayers] = useState<any[]>([]);
  const [allPlayersForParishes, setAllPlayersForParishes] = useState<any[]>([]); // Full player list for parish management
  const [parishesFromDB, setParishesFromDB] = useState<any[]>([]); // Parish data from sheffield_parishes table
  const [playerSearchText, setPlayerSearchText] = useState<string>('');
  const [searchInputValue, setSearchInputValue] = useState<string>(''); // For debounced search
  const [showDuplicatesOnly, setShowDuplicatesOnly] = useState<boolean>(false);
  const [selectedParish, setSelectedParish] = useState<string>('');
  const [parishPostcode, setParishPostcode] = useState<string>('');
  const [selectedBirthYear, setSelectedBirthYear] = useState<string>('');
  const [assignmentFilter, setAssignmentFilter] = useState<string>('all'); // 'all', 'assigned', 'unassigned'
  const [showAgeStats, setShowAgeStats] = useState<boolean>(false);
  const [showFirstNameStats, setShowFirstNameStats] = useState<boolean>(false);
  const [showMiddleNameStats, setShowMiddleNameStats] = useState<boolean>(false);
  const [showSurnameStats, setShowSurnameStats] = useState<boolean>(false);
  const [useDefaultParish, setUseDefaultParish] = useState<boolean>(false);

  // Pagination state
  const [currentPage, setCurrentPage] = useState<number>(0);
  const [pageSize, setPageSize] = useState<number>(100);
  const [totalPlayers, setTotalPlayers] = useState<number>(0);
  const [isLoadingPlayers, setIsLoadingPlayers] = useState<boolean>(false);

  // Horizontal scroll state for player table
  const tableScrollRef = useRef<HTMLDivElement>(null);
  const [canScrollLeft, setCanScrollLeft] = useState<boolean>(false);
  const [canScrollRight, setCanScrollRight] = useState<boolean>(false);

  // Statistics data (loaded separately)
  const [ageStats, setAgeStats] = useState<any[]>([]);
  const [firstNameStats, setFirstNameStats] = useState<any[]>([]);
  const [middleNameStats, setMiddleNameStats] = useState<any[]>([]);
  const [surnameStats, setSurnameStats] = useState<any[]>([]);
  const [allParishes, setAllParishes] = useState<string[]>([]);
  const [allBirthYears, setAllBirthYears] = useState<number[]>([]);

  // Parish management
  const [parishPostcodes, setParishPostcodes] = useState<{ [parish: string]: string }>({});

  // Confirmation dialog state
  const [confirmDialog, setConfirmDialog] = useState<{
    isOpen: boolean;
    title: string;
    message: string;
    onConfirm: () => void;
    danger?: boolean;
  }>({
    isOpen: false,
    title: '',
    message: '',
    onConfirm: () => {},
    danger: false
  });

  // Backup state
  const [backups, setBackups] = useState<any[]>([]);
  const [showBackupManager, setShowBackupManager] = useState(false);

  useEffect(() => {
    loadClubs();
    loadDivisions();
  }, []);

  useEffect(() => {
    applyFilters();
  }, [clubs, searchText, filterPostcode, filterArea, hideReserves, hideNonReserves]);

  useEffect(() => {
    if (selectedClubId) {
      loadClubDivisionInfo();
    }
  }, [selectedClubId]);

  useEffect(() => {
    if (activeTab === 'squads') {
      loadSquadClubs();
    }
    if (activeTab === 'club-capacity' || activeTab === 'parishes') {
      loadAllPlayers();
    }
  }, [activeTab]);

  useEffect(() => {
    if (selectedSquadClubId && activeTab === 'squads') {
      loadSquadPlayers(selectedSquadClubId);
    }
  }, [selectedSquadClubId]);

  // Auto-load players without stats when switching to assign-stats tab
  useEffect(() => {
    console.log('[Tab Change] activeTab changed to:', activeTab);
    if (activeTab === 'assign-stats') {
      loadPlayersWithoutStats();
    } else if (activeTab === 'player-database') {
      loadAllPlayers();
      loadFilterOptions(); // Load filter options once
    } else if (activeTab === 'parishes') {
      console.log('[Tab Change] Parishes tab activated, calling loadAllPlayersForParishes and loadParishesFromDB');
      loadAllPlayersForParishes(); // Load all players for parish management
      loadParishesFromDB(); // Load parish data from sheffield_parishes table
    }
  }, [activeTab]);

  // Debounced search: Update playerSearchText 500ms after user stops typing
  useEffect(() => {
    const timer = setTimeout(() => {
      setPlayerSearchText(searchInputValue);
      setCurrentPage(0); // Reset to first page on new search
    }, 500);

    return () => clearTimeout(timer);
  }, [searchInputValue]);

  // Reload players when filters or pagination changes
  useEffect(() => {
    if (activeTab === 'player-database') {
      loadAllPlayers();
    }
  }, [currentPage, pageSize, playerSearchText, selectedParish, selectedBirthYear, assignmentFilter]);

  // Update scroll buttons when table data changes or window resizes
  useEffect(() => {
    if (activeTab === 'player-database') {
      // Small delay to ensure table is rendered
      const timer = setTimeout(() => {
        updateScrollButtons();
      }, 100);

      const handleResize = () => updateScrollButtons();
      window.addEventListener('resize', handleResize);

      return () => {
        clearTimeout(timer);
        window.removeEventListener('resize', handleResize);
      };
    }
  }, [allPlayers, activeTab]);

  const loadPlayersWithoutStats = async () => {
    try {
      setError('');
      const players = await invoke<any[]>('db_get_all_players_without_stats');
      setPlayersWithoutStats(players);
      console.log('Loaded players without stats:', players);
    } catch (err) {
      console.error('Error loading players without stats:', err);
      setError(`Failed to load players: ${err}`);
    }
  };

  const loadAllPlayers = async () => {
    try {
      setError('');
      setIsLoadingPlayers(true);

      const result = await invoke<{
        players: any[];
        total_count: number;
        page: number;
        page_size: number;
      }>('db_get_players_paginated', {
        page: currentPage,
        pageSize: pageSize,
        search: playerSearchText || null,
        parish: selectedParish || null,
        birthYear: selectedBirthYear ? parseInt(selectedBirthYear) : null,
        assignmentFilter: assignmentFilter,
      });

      setAllPlayers(result.players);
      setTotalPlayers(result.total_count);
      console.log(`Loaded page ${result.page + 1} (${result.players.length} players of ${result.total_count} total)`);
    } catch (err) {
      console.error('Error loading players:', err);
      setError(`Failed to load players: ${err}`);
    } finally {
      setIsLoadingPlayers(false);
    }
  };

  // Load parishes and birth years for filters (once at start)
  const loadFilterOptions = async () => {
    try {
      // Get unique parishes - we'll do this with a simple query
      const allPlayersForFilters = await invoke<any[]>('db_get_all_players');
      const parishes = Array.from(new Set(
        allPlayersForFilters
          .map(p => p.ecclesiastical_parish)
          .filter(p => p && p.trim() !== '')
      )).sort();
      setAllParishes(parishes as string[]);

      const years = Array.from(new Set(
        allPlayersForFilters
          .map(p => p.birth_year)
          .filter(y => y)
      )).sort((a, b) => a - b);
      setAllBirthYears(years as number[]);
    } catch (err) {
      console.error('Error loading filter options:', err);
    }
  };

  // Load ALL players for Parish Management tab
  const loadAllPlayersForParishes = async () => {
    try {
      console.log('[Parish] Loading all players for parish management...');
      const players = await invoke<any[]>('db_get_all_players');
      console.log(`[Parish] Loaded ${players.length} total players`);
      console.log('[Parish] Sample player:', players[0]);
      setAllPlayersForParishes(players);
      console.log('[Parish] State updated with players');
    } catch (err) {
      console.error('[Parish] ERROR loading all players for parishes:', err);
      setError(`Failed to load players: ${err}`);
    }
  };

  // Load all parishes from sheffield_parishes table
  const loadParishesFromDB = async () => {
    try {
      console.log('[Parish] Loading parishes from sheffield_parishes table...');
      const parishes = await invoke<any[]>('db_get_all_parishes');
      console.log(`[Parish] Loaded ${parishes.length} parishes`);
      setParishesFromDB(parishes);
    } catch (err) {
      console.error('[Parish] ERROR loading parishes:', err);
      setError(`Failed to load parishes: ${err}`);
    }
  };

  // Load statistics separately when toggled
  const loadAgeStatistics = async () => {
    if (ageStats.length > 0) return; // Already loaded
    try {
      const stats = await invoke<any[]>('db_get_age_statistics', { year });
      setAgeStats(stats);
    } catch (err) {
      console.error('Error loading age statistics:', err);
    }
  };

  const loadFirstNameStatistics = async () => {
    if (firstNameStats.length > 0) return; // Already loaded
    try {
      const stats = await invoke<any[]>('db_get_first_name_statistics');
      setFirstNameStats(stats);
    } catch (err) {
      console.error('Error loading first name statistics:', err);
    }
  };

  const loadMiddleNameStatistics = async () => {
    if (middleNameStats.length > 0) return; // Already loaded
    try {
      const stats = await invoke<any[]>('db_get_middle_name_statistics');
      setMiddleNameStats(stats);
    } catch (err) {
      console.error('Error loading middle name statistics:', err);
    }
  };

  const loadSurnameStatistics = async () => {
    if (surnameStats.length > 0) return; // Already loaded
    try {
      const stats = await invoke<any[]>('db_get_surname_statistics');
      setSurnameStats(stats);
    } catch (err) {
      console.error('Error loading surname statistics:', err);
    }
  };

  // Load stats when toggled on
  useEffect(() => {
    if (showAgeStats) loadAgeStatistics();
  }, [showAgeStats]);

  useEffect(() => {
    if (showFirstNameStats) loadFirstNameStatistics();
  }, [showFirstNameStats]);

  useEffect(() => {
    if (showMiddleNameStats) loadMiddleNameStatistics();
  }, [showMiddleNameStats]);

  useEffect(() => {
    if (showSurnameStats) loadSurnameStatistics();
  }, [showSurnameStats]);

  const deletePlayer = async (playerId: string) => {
    if (!confirm('Are you sure you want to delete this player? This action cannot be undone.')) {
      return;
    }

    try {
      setError('');
      await invoke('db_delete_player', { playerId });
      console.log('Deleted player:', playerId);
      // Reload the player list
      await loadAllPlayers();
      setSuccess('Player deleted successfully');
      setTimeout(() => setSuccess(''), 3000);
    } catch (err) {
      console.error('Error deleting player:', err);
      setError(`Failed to delete player: ${err}`);
    }
  };

  const assignPostcodeToParish = async () => {
    try {
      setError('');
      const count = await invoke<number>('db_assign_postcode_to_parish', {
        parish: selectedParish,
        postcode: parishPostcode
      });
      setSuccess(`Successfully assigned postcode to ${count} players`);
      await loadAllPlayers(); // Refresh the list
      setParishPostcode(''); // Clear the input
      setTimeout(() => setSuccess(''), 3000);
    } catch (err) {
      console.error('Error assigning postcode:', err);
      setError(`Failed to assign postcode: ${err}`);
    }
  };

  const loadClubs = async () => {
    try {
      console.log('CALLING db_get_all_clubs...');
      const clubsList = await invoke<Club[]>('db_get_all_clubs');
      console.log('RECEIVED CLUBS:', clubsList.length, clubsList.slice(0, 3));
      setClubs(clubsList);
      if (clubsList.length > 0 && !selectedClubId) {
        setSelectedClubId(clubsList[0].id);
      }
    } catch (err) {
      console.error('LOAD CLUBS ERROR:', err);
      setError(`Failed to load clubs: ${err}`);
    }
  };

  const loadDivisions = async () => {
    try {
      const divisionsList = await invoke<Division[]>('db_get_all_divisions');
      setDivisions(divisionsList);
    } catch (err) {
      console.error(`Failed to load divisions: ${err}`);
    }
  };

  const loadClubDivisionInfo = async () => {
    if (!selectedClubId) return;

    try {
      const info = await invoke<ClubDivisionInfo>('get_club_division_info', {
        clubId: selectedClubId
      });
      setClubDivisionInfo(info);
    } catch (err) {
      // Club might not be in a division yet
      setClubDivisionInfo(null);
    }
  };

  const applyFilters = () => {
    let filtered = clubs;

    // Hide reserve clubs if checkbox is enabled
    if (hideReserves) {
      filtered = filtered.filter(c =>
        !c.name.toLowerCase().includes('reserve')
      );
    }

    // Hide non-reserve clubs if checkbox is enabled
    if (hideNonReserves) {
      filtered = filtered.filter(c =>
        c.name.toLowerCase().includes('reserve')
      );
    }

    if (searchText) {
      filtered = filtered.filter(c =>
        c.name.toLowerCase().includes(searchText.toLowerCase()) ||
        c.ground_name.toLowerCase().includes(searchText.toLowerCase())
      );
    }

    if (filterPostcode) {
      filtered = filtered.filter(c =>
        c.where_from && c.where_from.toLowerCase().includes(filterPostcode.toLowerCase())
      );
    }

    if (filterArea) {
      filtered = filtered.filter(c =>
        c.city && c.city.toLowerCase().includes(filterArea.toLowerCase())
      );
    }

    setFilteredClubs(filtered);
  };

  const handleEditLocation = () => {
    if (selectedClub) {
      setEditCity(selectedClub.city || '');
      setEditRegion(selectedClub.region || '');
      setEditOrigin(selectedClub.origin || '');
      setEditingLocation(true);
      setError('');
      setSuccess('');
    }
  };

  const handleSaveLocation = async () => {
    setError('');
    setSuccess('');

    try {
      await invoke('db_update_club_location', {
        clubId: selectedClubId,
        city: editCity.trim() || null,
        region: editRegion.trim() || null,
        origin: editOrigin.trim() || null,
      });

      setSuccess('Location saved successfully!');
      setEditingLocation(false);
      await loadClubs();
    } catch (err) {
      setError(`Failed to save location: ${err}`);
    }
  };

  const handleCancelEditLocation = () => {
    setEditingLocation(false);
    setEditCity('');
    setEditRegion('');
    setEditOrigin('');
    setError('');
    setSuccess('');
  };

  const handleEditName = () => {
    if (selectedClub) {
      setEditName(selectedClub.name);
      setEditingName(true);
      setError('');
      setSuccess('');
    }
  };

  const handleSaveName = async () => {
    if (!editName.trim()) {
      setError('Club name cannot be empty');
      return;
    }

    setError('');
    setSuccess('');

    try {
      await invoke('db_update_club_name', {
        clubId: selectedClubId,
        name: editName.trim(),
      });

      setSuccess('Name saved successfully!');
      setEditingName(false);
      await loadClubs();
    } catch (err) {
      setError(`Failed to save name: ${err}`);
    }
  };

  const handleCancelEditName = () => {
    setEditingName(false);
    setEditName('');
    setError('');
    setSuccess('');
  };

  const handleCopyToReserve = async () => {
    if (!selectedClubId) return;

    setError('');
    setSuccess('');

    try {
      const result = await invoke<string>('db_copy_parent_data_to_reserve', {
        parentClubId: selectedClubId,
      });
      setSuccess(result);
      await loadClubs();
    } catch (err) {
      setError(`Failed to copy data: ${err}`);
    }
  };

  const handleCreateClub = () => {
    setCreatingClub(true);
    setNewClubName('');
    setNewClubFoundedYear(1867);
    setNewClubGroundName('Unknown');
    setNewClubCity('');
    setNewClubRegion('');
    setNewClubOrigin('');
    setError('');
    setSuccess('');
  };

  const handleSaveNewClub = async () => {
    if (!newClubName.trim()) {
      setError('Club name cannot be empty');
      return;
    }

    setError('');
    setSuccess('');

    try {
      // Create a kebab-case ID from the club name
      const clubId = newClubName.toLowerCase().replace(/\s+/g, '-').replace(/[^a-z0-9-]/g, '');

      await invoke('db_create_club', {
        clubId,
        clubName: newClubName,
        foundedYear: newClubFoundedYear,
        groundName: newClubGroundName,
        city: newClubCity,
        region: newClubRegion,
        origin: newClubOrigin,
      });

      setSuccess('Club created successfully!');
      setCreatingClub(false);
      await loadClubs();
      setSelectedClubId(clubId);
    } catch (err) {
      setError(`Failed to create club: ${err}`);
    }
  };

  const handleCancelCreateClub = () => {
    setCreatingClub(false);
    setNewClubName('');
    setNewClubFoundedYear(1867);
    setNewClubGroundName('Unknown');
    setNewClubCity('');
    setNewClubRegion('');
    setNewClubOrigin('');
    setError('');
    setSuccess('');
  };

  // Squad Management Handlers
  const loadSquadClubs = async () => {
    try {
      const clubsList = await invoke<Club[]>('db_get_all_clubs');

      // Load division info for each club
      const clubsWithDivision = await Promise.all(
        clubsList.map(async (club) => {
          try {
            const divInfo = await invoke<ClubDivisionInfo>('get_club_division_info', {
              clubId: club.id,
            });
            return { ...club, division_name: divInfo.division_name };
          } catch {
            // Club not in a division
            return { ...club, division_name: undefined };
          }
        })
      );

      setSquadClubs(clubsWithDivision);
      if (clubsWithDivision.length > 0 && !selectedSquadClubId) {
        setSelectedSquadClubId(clubsWithDivision[0].id);
      }
    } catch (err) {
      setError(`Failed to load clubs: ${err}`);
    }
  };

  const loadSquadPlayers = async (clubId: string) => {
    if (!clubId) return;
    console.log('[SQUAD DEBUG] Loading squad for club ID:', clubId);
    try {
      const players = await invoke<any[]>('db_get_squad_simple', { clubId });
      console.log('[SQUAD DEBUG] Received players:', players.length, 'players');
      console.log('[SQUAD DEBUG] First player:', players[0]);
      setSquadPlayers(players);
      setSelectedPlayerId(null);
      setEditingPlayer(null);
    } catch (err) {
      console.error('[SQUAD DEBUG] Error loading players:', err);
      setError(`Failed to load players: ${err}`);
      setSquadPlayers([]);
    }
  };

  const handleSelectSquadClub = async (clubId: string) => {
    setSelectedSquadClubId(clubId);
    await loadSquadPlayers(clubId);
  };

  const handleEditPlayer = (player: Player) => {
    setEditingPlayer({ ...player });
    setSelectedPlayerId(player.id);
  };

  const handleCancelEditPlayer = () => {
    setEditingPlayer(null);
    setSelectedPlayerId(null);
  };

  const handleSavePlayer = async () => {
    if (!editingPlayer) return;

    try {
      await invoke('db_update_player_stats', {
        playerId: editingPlayer.id,
        stats: {
          position: editingPlayer.position,
          pace: editingPlayer.pace,
          acceleration: editingPlayer.acceleration,
          strength: editingPlayer.strength,
          stamina: editingPlayer.stamina,
          balance: editingPlayer.balance,
          jumping: editingPlayer.jumping,
          agility: editingPlayer.agility,
          natural_fitness: editingPlayer.natural_fitness,
          passing: editingPlayer.passing,
          dribbling: editingPlayer.dribbling,
          first_touch: editingPlayer.first_touch,
          technique: editingPlayer.technique,
          heading: editingPlayer.heading,
          long_passing: editingPlayer.long_passing,
          crossing: editingPlayer.crossing,
          long_shots: editingPlayer.long_shots,
          tackling: editingPlayer.tackling,
          handling: editingPlayer.handling,
          reflexes: editingPlayer.reflexes,
          corners: editingPlayer.corners,
          free_kicks: editingPlayer.free_kicks,
          throw_ins: editingPlayer.throw_ins,
          vision: editingPlayer.vision,
          left_foot: editingPlayer.left_foot,
          right_foot: editingPlayer.right_foot,
          one_on_ones: editingPlayer.one_on_ones,
          courage: editingPlayer.courage,
          bravery: editingPlayer.bravery,
          concentration: editingPlayer.concentration,
          decision_making: editingPlayer.decision_making,
          leadership: editingPlayer.leadership,
          aggression: editingPlayer.aggression,
          anticipation: editingPlayer.anticipation,
          determination: editingPlayer.determination,
          flair: editingPlayer.flair,
          influence: editingPlayer.influence,
          adaptability: editingPlayer.adaptability,
          ambition: editingPlayer.ambition,
          loyalty: editingPlayer.loyalty,
          pressure: editingPlayer.pressure,
          professionalism: editingPlayer.professionalism,
          sportsmanship: editingPlayer.sportsmanship,
          temperament: editingPlayer.temperament,
          awareness: editingPlayer.awareness,
          marking: editingPlayer.marking,
          positioning: editingPlayer.positioning,
          work_rate: editingPlayer.work_rate,
          off_the_ball: editingPlayer.off_the_ball,
          movement: editingPlayer.movement,
          teamwork: editingPlayer.teamwork,
          finishing: editingPlayer.finishing,
          penalties: editingPlayer.penalties,
          set_pieces: editingPlayer.set_pieces,
          consistency: editingPlayer.consistency,
          dirtiness: editingPlayer.dirtiness,
          versatility: editingPlayer.versatility,
          injury_proneness: editingPlayer.injury_proneness,
          important_matches: editingPlayer.important_matches,
          current_ability: editingPlayer.current_ability,
          potential_ability: editingPlayer.potential_ability,
          current_reputation: editingPlayer.current_reputation,
        },
      });
      setSuccess('Player stats updated successfully!');
      await loadSquadPlayers(selectedSquadClubId);
      setEditingPlayer(null);
      setSelectedPlayerId(null);
    } catch (err) {
      setError(`Failed to update player: ${err}`);
    }
  };

  const handleRandomizeStats = () => {
    if (!editingPlayer) return;

    // Determine era based on birth year
    const birthYear = editingPlayer.birth_year;
    let minStat = 4;
    let maxStat = 16;

    if (birthYear < 1870) {
      minStat = 4;
      maxStat = 12;
    } else if (birthYear < 1875) {
      minStat = 6;
      maxStat = 14;
    } else {
      minStat = 8;
      maxStat = 16;
    }

    const random = (min: number, max: number) => Math.floor(Math.random() * (max - min + 1)) + min;

    const position = editingPlayer.position;
    let updatedPlayer = { ...editingPlayer };

    // Generate stats based on position like player_generator.rs
    if (position === 'GK') {
      // GK: High handling, reflexes, positioning
      updatedPlayer.handling = random(minStat + 4, maxStat);
      updatedPlayer.reflexes = random(minStat + 4, maxStat);
      updatedPlayer.positioning = random(minStat + 3, maxStat);
      updatedPlayer.strength = random(minStat + 2, maxStat - 2);
      updatedPlayer.courage = random(minStat + 2, maxStat - 2);
      updatedPlayer.concentration = random(minStat + 2, maxStat - 2);
      // Low outfield stats
      updatedPlayer.passing = random(minStat, minStat + 4);
      updatedPlayer.dribbling = random(minStat, minStat + 2);
      updatedPlayer.finishing = random(minStat, minStat + 2);
      updatedPlayer.tackling = random(minStat, minStat + 3);
    } else if (position === 'CB') {
      // CB: High tackling, heading, strength, marking
      updatedPlayer.tackling = random(minStat + 4, maxStat);
      updatedPlayer.heading = random(minStat + 4, maxStat);
      updatedPlayer.strength = random(minStat + 3, maxStat);
      updatedPlayer.marking = random(minStat + 4, maxStat);
      updatedPlayer.positioning = random(minStat + 3, maxStat - 1);
      updatedPlayer.courage = random(minStat + 3, maxStat - 1);
      updatedPlayer.concentration = random(minStat + 2, maxStat - 2);
      // Lower attacking stats
      updatedPlayer.passing = random(minStat + 1, maxStat - 4);
      updatedPlayer.dribbling = random(minStat, maxStat - 6);
      updatedPlayer.finishing = random(minStat, maxStat - 6);
      updatedPlayer.handling = random(minStat, minStat + 2);
      updatedPlayer.reflexes = random(minStat, minStat + 2);
    } else if (position === 'FB') {
      // FB: Good pace, tackling, stamina
      updatedPlayer.pace = random(minStat + 3, maxStat);
      updatedPlayer.tackling = random(minStat + 3, maxStat - 1);
      updatedPlayer.stamina = random(minStat + 3, maxStat);
      updatedPlayer.marking = random(minStat + 2, maxStat - 2);
      updatedPlayer.work_rate = random(minStat + 3, maxStat);
      updatedPlayer.crossing = random(minStat + 1, maxStat - 3);
      updatedPlayer.passing = random(minStat + 2, maxStat - 2);
      updatedPlayer.handling = random(minStat, minStat + 2);
      updatedPlayer.reflexes = random(minStat, minStat + 2);
    } else if (position === 'MID') {
      // MID: High passing, stamina, work_rate, awareness
      updatedPlayer.passing = random(minStat + 4, maxStat);
      updatedPlayer.stamina = random(minStat + 3, maxStat);
      updatedPlayer.work_rate = random(minStat + 4, maxStat);
      updatedPlayer.awareness = random(minStat + 3, maxStat);
      updatedPlayer.tackling = random(minStat + 2, maxStat - 2);
      updatedPlayer.dribbling = random(minStat + 2, maxStat - 2);
      updatedPlayer.concentration = random(minStat + 2, maxStat - 2);
      updatedPlayer.handling = random(minStat, minStat + 2);
      updatedPlayer.reflexes = random(minStat, minStat + 2);
    } else if (position === 'FWD') {
      // FWD: High finishing, heading, strength, positioning
      updatedPlayer.finishing = random(minStat + 4, maxStat);
      updatedPlayer.heading = random(minStat + 3, maxStat);
      updatedPlayer.strength = random(minStat + 2, maxStat - 1);
      updatedPlayer.positioning = random(minStat + 3, maxStat);
      updatedPlayer.dribbling = random(minStat + 2, maxStat - 2);
      updatedPlayer.pace = random(minStat + 2, maxStat - 1);
      updatedPlayer.courage = random(minStat + 2, maxStat - 2);
      updatedPlayer.handling = random(minStat, minStat + 2);
      updatedPlayer.reflexes = random(minStat, minStat + 2);
      updatedPlayer.tackling = random(minStat, maxStat - 6);
    } else if (position === 'WG') {
      // WG: High pace, dribbling, crossing, agility
      updatedPlayer.pace = random(minStat + 4, maxStat);
      updatedPlayer.dribbling = random(minStat + 4, maxStat);
      updatedPlayer.crossing = random(minStat + 3, maxStat);
      updatedPlayer.agility = random(minStat + 3, maxStat);
      updatedPlayer.flair = random(minStat + 2, maxStat - 1);
      updatedPlayer.stamina = random(minStat + 2, maxStat - 2);
      updatedPlayer.handling = random(minStat, minStat + 2);
      updatedPlayer.reflexes = random(minStat, minStat + 2);
      updatedPlayer.tackling = random(minStat, maxStat - 5);
    }

    // Generic stats for all positions
    updatedPlayer.balance = random(minStat, maxStat);
    updatedPlayer.jumping = random(minStat, maxStat);
    updatedPlayer.aggression = random(minStat, maxStat);
    updatedPlayer.determination = random(minStat, maxStat);
    updatedPlayer.leadership = random(minStat, maxStat - 4);
    updatedPlayer.influence = random(minStat, maxStat - 4);
    updatedPlayer.penalties = random(minStat, maxStat);
    updatedPlayer.set_pieces = random(minStat, maxStat);

    setEditingPlayer(updatedPlayer);
  };

  const handleApplyPreset = (preset: 'elite' | 'good' | 'average' | 'poor') => {
    if (!editingPlayer) return;

    // Determine era based on birth year
    const birthYear = editingPlayer.birth_year;
    let baseMin = 4;
    let baseMax = 16;

    if (birthYear < 1870) {
      baseMin = 4;
      baseMax = 12;
    } else if (birthYear < 1875) {
      baseMin = 6;
      baseMax = 14;
    } else {
      baseMin = 8;
      baseMax = 16;
    }

    // Adjust ranges based on preset
    let minStat = baseMin;
    let maxStat = baseMax;
    let bonus = 0;

    switch (preset) {
      case 'elite':
        minStat = baseMax - 5;
        maxStat = baseMax;
        bonus = 2;
        break;
      case 'good':
        minStat = Math.floor((baseMin + baseMax) / 2);
        maxStat = baseMax - 2;
        bonus = 1;
        break;
      case 'average':
        minStat = Math.floor((baseMin + baseMax) / 2) - 2;
        maxStat = Math.floor((baseMin + baseMax) / 2) + 2;
        bonus = 0;
        break;
      case 'poor':
        minStat = baseMin;
        maxStat = baseMin + 5;
        bonus = 0;
        break;
    }

    const random = (min: number, max: number) => Math.floor(Math.random() * (max - min + 1)) + min;
    const clamp = (val: number) => Math.max(1, Math.min(20, val));

    const position = editingPlayer.position;
    let updatedPlayer = { ...editingPlayer };

    // Generate stats based on position with preset multipliers
    if (position === 'GK') {
      updatedPlayer.handling = clamp(random(minStat + 4, maxStat) + bonus);
      updatedPlayer.reflexes = clamp(random(minStat + 4, maxStat) + bonus);
      updatedPlayer.positioning = clamp(random(minStat + 3, maxStat) + bonus);
      updatedPlayer.strength = clamp(random(minStat + 2, maxStat - 2) + bonus);
      updatedPlayer.courage = clamp(random(minStat + 2, maxStat - 2) + bonus);
      updatedPlayer.concentration = clamp(random(minStat + 2, maxStat - 2) + bonus);
      updatedPlayer.passing = clamp(random(baseMin, baseMin + 4));
      updatedPlayer.dribbling = clamp(random(baseMin, baseMin + 2));
      updatedPlayer.finishing = clamp(random(baseMin, baseMin + 2));
      updatedPlayer.tackling = clamp(random(baseMin, baseMin + 3));
    } else if (position === 'CB') {
      updatedPlayer.tackling = clamp(random(minStat + 4, maxStat) + bonus);
      updatedPlayer.heading = clamp(random(minStat + 4, maxStat) + bonus);
      updatedPlayer.strength = clamp(random(minStat + 3, maxStat) + bonus);
      updatedPlayer.marking = clamp(random(minStat + 4, maxStat) + bonus);
      updatedPlayer.positioning = clamp(random(minStat + 3, maxStat - 1) + bonus);
      updatedPlayer.courage = clamp(random(minStat + 3, maxStat - 1) + bonus);
      updatedPlayer.concentration = clamp(random(minStat + 2, maxStat - 2) + bonus);
      updatedPlayer.passing = clamp(random(minStat + 1, maxStat - 4));
      updatedPlayer.dribbling = clamp(random(minStat, maxStat - 6));
      updatedPlayer.finishing = clamp(random(minStat, maxStat - 6));
      updatedPlayer.handling = clamp(random(baseMin, baseMin + 2));
      updatedPlayer.reflexes = clamp(random(baseMin, baseMin + 2));
    } else if (position === 'FB') {
      updatedPlayer.pace = clamp(random(minStat + 3, maxStat) + bonus);
      updatedPlayer.tackling = clamp(random(minStat + 3, maxStat - 1) + bonus);
      updatedPlayer.stamina = clamp(random(minStat + 3, maxStat) + bonus);
      updatedPlayer.marking = clamp(random(minStat + 2, maxStat - 2) + bonus);
      updatedPlayer.work_rate = clamp(random(minStat + 3, maxStat) + bonus);
      updatedPlayer.crossing = clamp(random(minStat + 1, maxStat - 3) + bonus);
      updatedPlayer.passing = clamp(random(minStat + 2, maxStat - 2) + bonus);
      updatedPlayer.handling = clamp(random(baseMin, baseMin + 2));
      updatedPlayer.reflexes = clamp(random(baseMin, baseMin + 2));
    } else if (position === 'MID') {
      updatedPlayer.passing = clamp(random(minStat + 4, maxStat) + bonus);
      updatedPlayer.stamina = clamp(random(minStat + 3, maxStat) + bonus);
      updatedPlayer.work_rate = clamp(random(minStat + 4, maxStat) + bonus);
      updatedPlayer.awareness = clamp(random(minStat + 3, maxStat) + bonus);
      updatedPlayer.tackling = clamp(random(minStat + 2, maxStat - 2) + bonus);
      updatedPlayer.dribbling = clamp(random(minStat + 2, maxStat - 2) + bonus);
      updatedPlayer.concentration = clamp(random(minStat + 2, maxStat - 2) + bonus);
      updatedPlayer.handling = clamp(random(baseMin, baseMin + 2));
      updatedPlayer.reflexes = clamp(random(baseMin, baseMin + 2));
    } else if (position === 'FWD') {
      updatedPlayer.finishing = clamp(random(minStat + 4, maxStat) + bonus);
      updatedPlayer.heading = clamp(random(minStat + 3, maxStat) + bonus);
      updatedPlayer.strength = clamp(random(minStat + 2, maxStat - 1) + bonus);
      updatedPlayer.positioning = clamp(random(minStat + 3, maxStat) + bonus);
      updatedPlayer.dribbling = clamp(random(minStat + 2, maxStat - 2) + bonus);
      updatedPlayer.pace = clamp(random(minStat + 2, maxStat - 1) + bonus);
      updatedPlayer.courage = clamp(random(minStat + 2, maxStat - 2) + bonus);
      updatedPlayer.handling = clamp(random(baseMin, baseMin + 2));
      updatedPlayer.reflexes = clamp(random(baseMin, baseMin + 2));
      updatedPlayer.tackling = clamp(random(minStat, maxStat - 6));
    } else if (position === 'WG') {
      updatedPlayer.pace = clamp(random(minStat + 4, maxStat) + bonus);
      updatedPlayer.dribbling = clamp(random(minStat + 4, maxStat) + bonus);
      updatedPlayer.crossing = clamp(random(minStat + 3, maxStat) + bonus);
      updatedPlayer.agility = clamp(random(minStat + 3, maxStat) + bonus);
      updatedPlayer.flair = clamp(random(minStat + 2, maxStat - 1) + bonus);
      updatedPlayer.stamina = clamp(random(minStat + 2, maxStat - 2) + bonus);
      updatedPlayer.handling = clamp(random(baseMin, baseMin + 2));
      updatedPlayer.reflexes = clamp(random(baseMin, baseMin + 2));
      updatedPlayer.tackling = clamp(random(minStat, maxStat - 5));
    }

    // Generic stats for all positions
    updatedPlayer.balance = clamp(random(minStat, maxStat));
    updatedPlayer.jumping = clamp(random(minStat, maxStat));
    updatedPlayer.aggression = clamp(random(minStat, maxStat));
    updatedPlayer.determination = clamp(random(minStat, maxStat));
    updatedPlayer.leadership = clamp(random(minStat, maxStat - 4));
    updatedPlayer.influence = clamp(random(minStat, maxStat - 4));
    updatedPlayer.penalties = clamp(random(minStat, maxStat));
    updatedPlayer.set_pieces = clamp(random(minStat, maxStat));

    setEditingPlayer(updatedPlayer);
  };

  // Helper function to determine club quality tier
  const getClubQualityTier = (clubName: string, divisionName: string | undefined, isReserve: boolean): number => {
    // Hallam FC and Sheffield FC are the elite clubs
    if (clubName.includes('Hallam') && !isReserve) return 1;
    if (clubName.includes('Sheffield FC') && !isReserve) return 1;

    // Parse division level from division name
    let divisionLevel = 6; // Default to lowest
    if (divisionName) {
      const match = divisionName.match(/Division (\d+)/i);
      if (match) {
        divisionLevel = parseInt(match[1]);
      }
    }

    // Reserve teams are always weaker
    if (isReserve) {
      return divisionLevel + 6; // Push reserves down the quality scale
    }

    return divisionLevel;
  };

  // Helper function to assign positions to a squad
  const assignPositionsToSquad = (playerCount: number): string[] => {
    const positions: string[] = [];

    // Always add 1-2 goalkeepers
    const numGK = playerCount >= 12 ? 2 : 1;
    for (let i = 0; i < numGK; i++) {
      positions.push('GK');
    }

    const remainingPlayers = playerCount - numGK;
    const positionTypes = ['CB', 'FB', 'MID', 'FWD', 'WG'];
    const playersPerPosition = Math.floor(remainingPlayers / positionTypes.length);
    const extra = remainingPlayers % positionTypes.length;

    // Distribute positions evenly
    for (let i = 0; i < positionTypes.length; i++) {
      const count = playersPerPosition + (i < extra ? 1 : 0);
      for (let j = 0; j < count; j++) {
        positions.push(positionTypes[i]);
      }
    }

    // Shuffle to randomize assignment
    for (let i = positions.length - 1; i > 0; i--) {
      const j = Math.floor(Math.random() * (i + 1));
      [positions[i], positions[j]] = [positions[j], positions[i]];
    }

    return positions;
  };

  // Comprehensive stat generation function
  const generatePlayerStats = (
    player: Player,
    qualityTier: number,
    isReserve: boolean,
    position: string
  ): Player => {
    const random = (min: number, max: number) => Math.floor(Math.random() * (max - min + 1)) + min;
    const clamp = (val: number) => Math.max(1, Math.min(20, val));
    const clamp200 = (val: number) => Math.max(1, Math.min(200, val));

    // Determine base stats based on quality tier (1 = best, 12 = worst for reserves)
    let baseMin = 4;
    let baseMax = 16;

    // Quality tier affects base ranges
    if (qualityTier === 1) {
      // Elite clubs (Hallam, Sheffield FC)
      baseMin = 11;
      baseMax = 18;
    } else if (qualityTier <= 2) {
      // Division 2
      baseMin = 9;
      baseMax = 16;
    } else if (qualityTier <= 3) {
      // Division 3
      baseMin = 8;
      baseMax = 14;
    } else if (qualityTier <= 4) {
      // Division 4
      baseMin = 7;
      baseMax = 13;
    } else if (qualityTier <= 5) {
      // Division 5
      baseMin = 6;
      baseMax = 12;
    } else if (qualityTier <= 6) {
      // Division 6
      baseMin = 5;
      baseMax = 11;
    } else {
      // Reserve teams
      baseMin = 4;
      baseMax = 10;
    }

    // Adjust for birth year (older era = slightly lower baseline)
    const birthYear = player.birth_year;
    if (birthYear < 1845) {
      baseMax = Math.max(baseMin + 2, baseMax - 2);
    }

    // Add variance for hidden gems and poor performers
    const variance = random(-2, 3);
    const minStat = Math.max(1, baseMin + Math.floor(variance / 2));
    const maxStat = Math.min(20, baseMax + variance);

    // Determine if this is a hidden gem (10% chance in lower tiers)
    const isHiddenGem = qualityTier >= 4 && Math.random() < 0.10;
    const gemBonus = isHiddenGem ? random(2, 4) : 0;

    // Determine potential ability
    const age = 1867 - birthYear;
    const isYoung = age < 23;

    // Calculate current ability (1-200 scale)
    let currentAbility = random(
      Math.floor((minStat / 20) * 200),
      Math.floor((maxStat / 20) * 200)
    ) + (gemBonus * 10);
    currentAbility = clamp200(currentAbility);

    // Calculate potential ability
    let potentialAbility = currentAbility;
    if (isYoung) {
      // Young players have higher potential
      const potentialBoost = random(10, 40);
      potentialAbility = clamp200(currentAbility + potentialBoost);
    } else if (isReserve && Math.random() < 0.15) {
      // Some reserve players have untapped potential
      const potentialBoost = random(15, 35);
      potentialAbility = clamp200(currentAbility + potentialBoost);
    } else {
      // Older players or established players have potential close to current
      potentialAbility = clamp200(currentAbility + random(-5, 10));
    }

    // Calculate reputation based on division and current ability
    let baseReputation = Math.floor(currentAbility * 0.8);
    if (qualityTier === 1) baseReputation += 20;
    else if (qualityTier <= 3) baseReputation += 10;
    const currentReputation = clamp200(baseReputation + random(-10, 10));

    const updatedPlayer = { ...player, position };

    // Position-specific stats with gem bonus
    if (position === 'GK') {
      updatedPlayer.handling = clamp(random(minStat + 4, maxStat) + gemBonus);
      updatedPlayer.reflexes = clamp(random(minStat + 4, maxStat) + gemBonus);
      updatedPlayer.positioning = clamp(random(minStat + 3, maxStat) + gemBonus);
      updatedPlayer.one_on_ones = clamp(random(minStat + 3, maxStat) + gemBonus);
      updatedPlayer.strength = clamp(random(minStat + 2, maxStat - 2));
      updatedPlayer.courage = clamp(random(minStat + 2, maxStat - 2));
      updatedPlayer.bravery = clamp(random(minStat + 3, maxStat - 1));
      updatedPlayer.concentration = clamp(random(minStat + 3, maxStat - 2));
      updatedPlayer.passing = clamp(random(minStat, minStat + 4));
      updatedPlayer.dribbling = clamp(random(minStat, minStat + 2));
      updatedPlayer.finishing = clamp(random(minStat, minStat + 2));
      updatedPlayer.tackling = clamp(random(minStat, minStat + 3));
    } else if (position === 'CB') {
      updatedPlayer.tackling = clamp(random(minStat + 4, maxStat) + gemBonus);
      updatedPlayer.heading = clamp(random(minStat + 4, maxStat) + gemBonus);
      updatedPlayer.strength = clamp(random(minStat + 3, maxStat) + gemBonus);
      updatedPlayer.marking = clamp(random(minStat + 4, maxStat) + gemBonus);
      updatedPlayer.positioning = clamp(random(minStat + 3, maxStat - 1));
      updatedPlayer.courage = clamp(random(minStat + 3, maxStat - 1));
      updatedPlayer.bravery = clamp(random(minStat + 4, maxStat));
      updatedPlayer.concentration = clamp(random(minStat + 2, maxStat - 2));
      updatedPlayer.anticipation = clamp(random(minStat + 3, maxStat - 1));
      updatedPlayer.passing = clamp(random(minStat + 1, maxStat - 4));
      updatedPlayer.dribbling = clamp(random(minStat, maxStat - 6));
      updatedPlayer.finishing = clamp(random(minStat, maxStat - 6));
      updatedPlayer.handling = clamp(random(minStat, minStat + 2));
      updatedPlayer.reflexes = clamp(random(minStat, minStat + 2));
    } else if (position === 'FB') {
      updatedPlayer.pace = clamp(random(minStat + 3, maxStat) + gemBonus);
      updatedPlayer.acceleration = clamp(random(minStat + 3, maxStat) + gemBonus);
      updatedPlayer.tackling = clamp(random(minStat + 3, maxStat - 1));
      updatedPlayer.stamina = clamp(random(minStat + 3, maxStat));
      updatedPlayer.marking = clamp(random(minStat + 2, maxStat - 2));
      updatedPlayer.work_rate = clamp(random(minStat + 3, maxStat));
      updatedPlayer.crossing = clamp(random(minStat + 1, maxStat - 3));
      updatedPlayer.passing = clamp(random(minStat + 2, maxStat - 2));
      updatedPlayer.handling = clamp(random(minStat, minStat + 2));
      updatedPlayer.reflexes = clamp(random(minStat, minStat + 2));
    } else if (position === 'MID') {
      updatedPlayer.passing = clamp(random(minStat + 4, maxStat) + gemBonus);
      updatedPlayer.long_passing = clamp(random(minStat + 3, maxStat - 1) + gemBonus);
      updatedPlayer.vision = clamp(random(minStat + 3, maxStat) + gemBonus);
      updatedPlayer.stamina = clamp(random(minStat + 3, maxStat));
      updatedPlayer.work_rate = clamp(random(minStat + 4, maxStat));
      updatedPlayer.awareness = clamp(random(minStat + 3, maxStat));
      updatedPlayer.decision_making = clamp(random(minStat + 3, maxStat - 1));
      updatedPlayer.technique = clamp(random(minStat + 2, maxStat - 2));
      updatedPlayer.first_touch = clamp(random(minStat + 2, maxStat - 2));
      updatedPlayer.tackling = clamp(random(minStat + 2, maxStat - 2));
      updatedPlayer.dribbling = clamp(random(minStat + 2, maxStat - 2));
      updatedPlayer.concentration = clamp(random(minStat + 2, maxStat - 2));
      updatedPlayer.handling = clamp(random(minStat, minStat + 2));
      updatedPlayer.reflexes = clamp(random(minStat, minStat + 2));
    } else if (position === 'FWD') {
      updatedPlayer.finishing = clamp(random(minStat + 4, maxStat) + gemBonus);
      updatedPlayer.heading = clamp(random(minStat + 3, maxStat) + gemBonus);
      updatedPlayer.long_shots = clamp(random(minStat + 2, maxStat - 1) + gemBonus);
      updatedPlayer.strength = clamp(random(minStat + 2, maxStat - 1));
      updatedPlayer.positioning = clamp(random(minStat + 3, maxStat));
      updatedPlayer.off_the_ball = clamp(random(minStat + 3, maxStat - 1));
      updatedPlayer.anticipation = clamp(random(minStat + 3, maxStat - 1));
      updatedPlayer.dribbling = clamp(random(minStat + 2, maxStat - 2));
      updatedPlayer.pace = clamp(random(minStat + 2, maxStat - 1));
      updatedPlayer.courage = clamp(random(minStat + 2, maxStat - 2));
      updatedPlayer.handling = clamp(random(minStat, minStat + 2));
      updatedPlayer.reflexes = clamp(random(minStat, minStat + 2));
      updatedPlayer.tackling = clamp(random(minStat, maxStat - 6));
    } else if (position === 'WG') {
      updatedPlayer.pace = clamp(random(minStat + 4, maxStat) + gemBonus);
      updatedPlayer.acceleration = clamp(random(minStat + 4, maxStat) + gemBonus);
      updatedPlayer.dribbling = clamp(random(minStat + 4, maxStat) + gemBonus);
      updatedPlayer.crossing = clamp(random(minStat + 3, maxStat));
      updatedPlayer.agility = clamp(random(minStat + 3, maxStat));
      updatedPlayer.flair = clamp(random(minStat + 2, maxStat - 1));
      updatedPlayer.stamina = clamp(random(minStat + 2, maxStat - 2));
      updatedPlayer.technique = clamp(random(minStat + 2, maxStat - 2));
      updatedPlayer.handling = clamp(random(minStat, minStat + 2));
      updatedPlayer.reflexes = clamp(random(minStat, minStat + 2));
      updatedPlayer.tackling = clamp(random(minStat, maxStat - 5));
    }

    // Fill in remaining stats for all positions
    updatedPlayer.balance = updatedPlayer.balance || clamp(random(minStat, maxStat));
    updatedPlayer.jumping = updatedPlayer.jumping || clamp(random(minStat, maxStat));
    updatedPlayer.agility = updatedPlayer.agility || clamp(random(minStat, maxStat));
    updatedPlayer.natural_fitness = clamp(random(minStat, maxStat));
    updatedPlayer.pace = updatedPlayer.pace || clamp(random(minStat, maxStat));
    updatedPlayer.acceleration = updatedPlayer.acceleration || clamp(random(minStat, maxStat));
    updatedPlayer.strength = updatedPlayer.strength || clamp(random(minStat, maxStat));
    updatedPlayer.stamina = updatedPlayer.stamina || clamp(random(minStat, maxStat));

    // Technical stats
    updatedPlayer.passing = updatedPlayer.passing || clamp(random(minStat, maxStat));
    updatedPlayer.dribbling = updatedPlayer.dribbling || clamp(random(minStat, maxStat));
    updatedPlayer.first_touch = updatedPlayer.first_touch || clamp(random(minStat, maxStat));
    updatedPlayer.technique = updatedPlayer.technique || clamp(random(minStat, maxStat));
    updatedPlayer.heading = updatedPlayer.heading || clamp(random(minStat, maxStat));
    updatedPlayer.long_passing = updatedPlayer.long_passing || clamp(random(minStat, maxStat - 2));
    updatedPlayer.crossing = updatedPlayer.crossing || clamp(random(minStat, maxStat - 2));
    updatedPlayer.long_shots = updatedPlayer.long_shots || clamp(random(minStat, maxStat - 2));
    updatedPlayer.tackling = updatedPlayer.tackling || clamp(random(minStat, maxStat));
    updatedPlayer.handling = updatedPlayer.handling || clamp(random(minStat, minStat + 3));
    updatedPlayer.reflexes = updatedPlayer.reflexes || clamp(random(minStat, minStat + 3));
    updatedPlayer.corners = clamp(random(minStat, maxStat - 3));
    updatedPlayer.free_kicks = clamp(random(minStat, maxStat - 3));
    updatedPlayer.throw_ins = clamp(random(minStat, maxStat));
    updatedPlayer.vision = updatedPlayer.vision || clamp(random(minStat, maxStat - 1));
    updatedPlayer.left_foot = clamp(random(minStat, maxStat));
    updatedPlayer.right_foot = clamp(random(minStat, maxStat));
    updatedPlayer.one_on_ones = updatedPlayer.one_on_ones || clamp(random(minStat, maxStat - 2));

    // Mental stats
    updatedPlayer.courage = updatedPlayer.courage || clamp(random(minStat, maxStat));
    updatedPlayer.bravery = updatedPlayer.bravery || clamp(random(minStat, maxStat));
    updatedPlayer.concentration = updatedPlayer.concentration || clamp(random(minStat, maxStat));
    updatedPlayer.decision_making = updatedPlayer.decision_making || clamp(random(minStat, maxStat - 1));
    updatedPlayer.leadership = clamp(random(minStat, maxStat - 3));
    updatedPlayer.aggression = clamp(random(minStat, maxStat));
    updatedPlayer.anticipation = updatedPlayer.anticipation || clamp(random(minStat, maxStat - 1));
    updatedPlayer.determination = clamp(random(minStat, maxStat));
    updatedPlayer.flair = updatedPlayer.flair || clamp(random(minStat, maxStat - 2));
    updatedPlayer.influence = clamp(random(minStat, maxStat - 3));
    updatedPlayer.adaptability = clamp(random(minStat, maxStat));
    updatedPlayer.ambition = clamp(random(minStat, maxStat));
    updatedPlayer.loyalty = clamp(random(minStat, maxStat));
    updatedPlayer.pressure = clamp(random(minStat, maxStat));
    updatedPlayer.professionalism = clamp(random(minStat, maxStat));
    updatedPlayer.sportsmanship = clamp(random(minStat, maxStat));
    updatedPlayer.temperament = clamp(random(minStat, maxStat));

    // Positioning stats
    updatedPlayer.awareness = updatedPlayer.awareness || clamp(random(minStat, maxStat));
    updatedPlayer.marking = updatedPlayer.marking || clamp(random(minStat, maxStat));
    updatedPlayer.positioning = updatedPlayer.positioning || clamp(random(minStat, maxStat));
    updatedPlayer.work_rate = updatedPlayer.work_rate || clamp(random(minStat, maxStat));
    updatedPlayer.off_the_ball = updatedPlayer.off_the_ball || clamp(random(minStat, maxStat - 1));
    updatedPlayer.movement = clamp(random(minStat, maxStat));
    updatedPlayer.teamwork = clamp(random(minStat, maxStat));

    // Specialization
    updatedPlayer.finishing = updatedPlayer.finishing || clamp(random(minStat, maxStat));
    updatedPlayer.penalties = clamp(random(minStat, maxStat - 2));
    updatedPlayer.set_pieces = clamp(random(minStat, maxStat - 3));

    // Hidden attributes
    updatedPlayer.consistency = clamp(random(minStat, maxStat));
    updatedPlayer.dirtiness = clamp(random(minStat - 2, minStat + 4));
    updatedPlayer.versatility = clamp(random(minStat, maxStat - 4));
    updatedPlayer.injury_proneness = clamp(random(minStat - 1, minStat + 6));
    updatedPlayer.important_matches = clamp(random(minStat, maxStat));

    // Ability & Reputation
    updatedPlayer.current_ability = currentAbility;
    updatedPlayer.potential_ability = potentialAbility;
    updatedPlayer.current_reputation = currentReputation;

    return updatedPlayer;
  };

  // Generate stats for all clubs
  const handleGenerateStatsForAllClubs = async () => {
    const startIndex = Math.max(1, startFromClub);
    if (!confirm(`This will generate stats and positions for players starting from club ${startIndex}. This cannot be undone.\n\nContinue?`)) {
      return;
    }

    setError('');
    setSuccess('');
    setProgressMessage('');
    setLoading(true);

    try {
      // Load all clubs
      setProgressMessage('Loading all clubs...');
      const allClubs = await invoke<Club[]>('db_get_all_clubs');

      let totalUpdated = 0;
      let clubsProcessed = 0;

      // Start from the specified club index (1-based)
      for (let clubIndex = startIndex - 1; clubIndex < allClubs.length; clubIndex++) {
        const club = allClubs[clubIndex];
        clubsProcessed++;
        const actualClubNumber = clubIndex + 1;
        setProgressMessage(`Processing club ${actualClubNumber}/${allClubs.length}: ${club.name}...`);
        // Get all players for this club
        const players = await invoke<Player[]>('db_get_club_players', { clubId: club.id });

        if (players.length === 0) continue;

        // Get club division info
        let divisionName: string | undefined;
        try {
          const divisionInfo = await invoke<ClubDivisionInfo>('get_club_division_info', { clubId: club.id });
          divisionName = divisionInfo.division_name;
        } catch {
          // Club not in a division
          divisionName = undefined;
        }

        const clubWithDivision = { ...club, division_name: divisionName };

        // Determine if this is a reserve team
        const isReserve = clubWithDivision.name.toLowerCase().includes('reserve') ||
                         clubWithDivision.name.toLowerCase().includes('second') ||
                         clubWithDivision.name.toLowerCase().includes('junior');

        // Get quality tier for this club
        const qualityTier = getClubQualityTier(clubWithDivision.name, clubWithDivision.division_name, isReserve);

        // Assign positions
        const positions = assignPositionsToSquad(players.length);

        // Generate stats for each player
        for (let i = 0; i < players.length; i++) {
          const player = players[i];
          const position = positions[i];

          const updatedPlayer = generatePlayerStats(player, qualityTier, isReserve, position);

          // Save to database
          await invoke('db_update_player_stats', {
            playerId: updatedPlayer.id,
            stats: {
              position: updatedPlayer.position,
              pace: updatedPlayer.pace,
              acceleration: updatedPlayer.acceleration,
              strength: updatedPlayer.strength,
              stamina: updatedPlayer.stamina,
              balance: updatedPlayer.balance,
              jumping: updatedPlayer.jumping,
              agility: updatedPlayer.agility,
              natural_fitness: updatedPlayer.natural_fitness,
              passing: updatedPlayer.passing,
              dribbling: updatedPlayer.dribbling,
              first_touch: updatedPlayer.first_touch,
              technique: updatedPlayer.technique,
              heading: updatedPlayer.heading,
              long_passing: updatedPlayer.long_passing,
              crossing: updatedPlayer.crossing,
              long_shots: updatedPlayer.long_shots,
              tackling: updatedPlayer.tackling,
              handling: updatedPlayer.handling,
              reflexes: updatedPlayer.reflexes,
              corners: updatedPlayer.corners,
              free_kicks: updatedPlayer.free_kicks,
              throw_ins: updatedPlayer.throw_ins,
              vision: updatedPlayer.vision,
              left_foot: updatedPlayer.left_foot,
              right_foot: updatedPlayer.right_foot,
              one_on_ones: updatedPlayer.one_on_ones,
              courage: updatedPlayer.courage,
              bravery: updatedPlayer.bravery,
              concentration: updatedPlayer.concentration,
              decision_making: updatedPlayer.decision_making,
              leadership: updatedPlayer.leadership,
              aggression: updatedPlayer.aggression,
              anticipation: updatedPlayer.anticipation,
              determination: updatedPlayer.determination,
              flair: updatedPlayer.flair,
              influence: updatedPlayer.influence,
              adaptability: updatedPlayer.adaptability,
              ambition: updatedPlayer.ambition,
              loyalty: updatedPlayer.loyalty,
              pressure: updatedPlayer.pressure,
              professionalism: updatedPlayer.professionalism,
              sportsmanship: updatedPlayer.sportsmanship,
              temperament: updatedPlayer.temperament,
              awareness: updatedPlayer.awareness,
              marking: updatedPlayer.marking,
              positioning: updatedPlayer.positioning,
              work_rate: updatedPlayer.work_rate,
              off_the_ball: updatedPlayer.off_the_ball,
              movement: updatedPlayer.movement,
              teamwork: updatedPlayer.teamwork,
              finishing: updatedPlayer.finishing,
              penalties: updatedPlayer.penalties,
              set_pieces: updatedPlayer.set_pieces,
              consistency: updatedPlayer.consistency,
              dirtiness: updatedPlayer.dirtiness,
              versatility: updatedPlayer.versatility,
              injury_proneness: updatedPlayer.injury_proneness,
              important_matches: updatedPlayer.important_matches,
              current_ability: updatedPlayer.current_ability,
              potential_ability: updatedPlayer.potential_ability,
              current_reputation: updatedPlayer.current_reputation,
            }
          });

          totalUpdated++;
        }
      }

      setProgressMessage('');
      setLoading(false);
      setSuccess(`Successfully generated stats for ${totalUpdated} players across ${clubsProcessed} clubs!`);

      // Reload current squad if viewing one
      if (selectedSquadClubId) {
        await loadSquadPlayers(selectedSquadClubId);
      }

      setTimeout(() => setSuccess(''), 10000);
    } catch (err) {
      setProgressMessage('');
      setLoading(false);
      setError(`Failed to generate stats: ${err}`);
    }
  };

  // Generate stats for this club only
  const handleGenerateStatsForThisClub = async () => {
    if (!selectedSquadClubId) return;

    if (!confirm('This will generate stats and positions for all players in this club. This cannot be undone.\n\nContinue?')) {
      return;
    }

    setError('');
    setSuccess('');
    setProgressMessage('');
    setLoading(true);

    try {
      setProgressMessage('Loading club information...');
      // Get club info
      const club = squadClubs.find(c => c.id === selectedSquadClubId);
      if (!club) {
        setError('Club not found');
        return;
      }

      const isReserve = club.name.toLowerCase().includes('reserve') ||
                       club.name.toLowerCase().includes('second') ||
                       club.name.toLowerCase().includes('junior');

      const qualityTier = getClubQualityTier(club.name, club.division_name, isReserve);

      // Assign positions
      setProgressMessage('Assigning positions...');
      const positions = assignPositionsToSquad(squadPlayers.length);

      // Generate stats for each player
      for (let i = 0; i < squadPlayers.length; i++) {
        setProgressMessage(`Generating stats for player ${i + 1}/${squadPlayers.length}...`);
        const player = squadPlayers[i];
        const position = positions[i];

        const updatedPlayer = generatePlayerStats(player, qualityTier, isReserve, position);

        // Save to database
        await invoke('db_update_player_stats', {
          playerId: updatedPlayer.id,
          stats: {
            position: updatedPlayer.position,
            pace: updatedPlayer.pace,
            acceleration: updatedPlayer.acceleration,
            strength: updatedPlayer.strength,
            stamina: updatedPlayer.stamina,
            balance: updatedPlayer.balance,
            jumping: updatedPlayer.jumping,
            agility: updatedPlayer.agility,
            natural_fitness: updatedPlayer.natural_fitness,
            passing: updatedPlayer.passing,
            dribbling: updatedPlayer.dribbling,
            first_touch: updatedPlayer.first_touch,
            technique: updatedPlayer.technique,
            heading: updatedPlayer.heading,
            long_passing: updatedPlayer.long_passing,
            crossing: updatedPlayer.crossing,
            long_shots: updatedPlayer.long_shots,
            tackling: updatedPlayer.tackling,
            handling: updatedPlayer.handling,
            reflexes: updatedPlayer.reflexes,
            corners: updatedPlayer.corners,
            free_kicks: updatedPlayer.free_kicks,
            throw_ins: updatedPlayer.throw_ins,
            vision: updatedPlayer.vision,
            left_foot: updatedPlayer.left_foot,
            right_foot: updatedPlayer.right_foot,
            one_on_ones: updatedPlayer.one_on_ones,
            courage: updatedPlayer.courage,
            bravery: updatedPlayer.bravery,
            concentration: updatedPlayer.concentration,
            decision_making: updatedPlayer.decision_making,
            leadership: updatedPlayer.leadership,
            aggression: updatedPlayer.aggression,
            anticipation: updatedPlayer.anticipation,
            determination: updatedPlayer.determination,
            flair: updatedPlayer.flair,
            influence: updatedPlayer.influence,
            adaptability: updatedPlayer.adaptability,
            ambition: updatedPlayer.ambition,
            loyalty: updatedPlayer.loyalty,
            pressure: updatedPlayer.pressure,
            professionalism: updatedPlayer.professionalism,
            sportsmanship: updatedPlayer.sportsmanship,
            temperament: updatedPlayer.temperament,
            awareness: updatedPlayer.awareness,
            marking: updatedPlayer.marking,
            positioning: updatedPlayer.positioning,
            work_rate: updatedPlayer.work_rate,
            off_the_ball: updatedPlayer.off_the_ball,
            movement: updatedPlayer.movement,
            teamwork: updatedPlayer.teamwork,
            finishing: updatedPlayer.finishing,
            penalties: updatedPlayer.penalties,
            set_pieces: updatedPlayer.set_pieces,
            consistency: updatedPlayer.consistency,
            dirtiness: updatedPlayer.dirtiness,
            versatility: updatedPlayer.versatility,
            injury_proneness: updatedPlayer.injury_proneness,
            important_matches: updatedPlayer.important_matches,
            current_ability: updatedPlayer.current_ability,
            potential_ability: updatedPlayer.potential_ability,
            current_reputation: updatedPlayer.current_reputation,
          }
        });
      }

      setProgressMessage('Reloading squad...');
      await loadSquadPlayers(selectedSquadClubId);

      setProgressMessage('');
      setLoading(false);
      setSuccess(`Successfully generated stats for ${squadPlayers.length} players in ${club.name}!`);

      setTimeout(() => setSuccess(''), 8000);
    } catch (err) {
      setProgressMessage('');
      setLoading(false);
      setError(`Failed to generate stats: ${err}`);
    }
  };

  const handleCapitalizeName = () => {
    const abbreviations = ['FC', 'XI', 'B'];
    const preserveWords = ['Reserves', 'Reserve', 'Juniors', 'Junior', 'Second'];
    const capitalizedName = editName
      .split(' ')
      .map(word => {
        if (word.length === 0) return word;

        // Check if word has parentheses
        const hasOpenParen = word.startsWith('(');
        const hasCloseParen = word.endsWith(')');
        let cleanWord = word;

        if (hasOpenParen) cleanWord = cleanWord.slice(1);
        if (hasCloseParen) cleanWord = cleanWord.slice(0, -1);

        let result = cleanWord;

        // Keep abbreviations in uppercase
        if (abbreviations.includes(cleanWord.toUpperCase())) {
          result = cleanWord.toUpperCase();
        }
        // Preserve specific words with their proper capitalization
        else {
          const matchedWord = preserveWords.find(w => w.toLowerCase() === cleanWord.toLowerCase());
          if (matchedWord) {
            result = matchedWord;
          } else {
            // Capitalize first letter, lowercase the rest
            result = cleanWord.charAt(0).toUpperCase() + cleanWord.slice(1).toLowerCase();
          }
        }

        // Add back parentheses if they existed
        if (hasOpenParen) result = '(' + result;
        if (hasCloseParen) result = result + ')';

        return result;
      })
      .join(' ');
    setEditName(capitalizedName);
  };

  const handleEditDivision = () => {
    if (clubDivisionInfo) {
      setEditDivisionId(clubDivisionInfo.division_id);
      setEditPosition(clubDivisionInfo.position);
    } else {
      setEditDivisionId('');
      setEditPosition(1);
    }
    setEditingDivision(true);
    setError('');
    setSuccess('');
  };

  const handleSaveDivision = async () => {
    if (!selectedClubId || !editDivisionId) {
      setError('Please select a division');
      return;
    }

    setError('');
    setSuccess('');

    try {
      await invoke('db_update_club_division', {
        clubId: selectedClubId,
        divisionId: editDivisionId,
        position: editPosition,
      });

      setSuccess('Division saved successfully!');
      setEditingDivision(false);
      await loadClubDivisionInfo();
    } catch (err) {
      setError(`Failed to save division: ${err}`);
    }
  };

  const handleCancelEditDivision = () => {
    setEditingDivision(false);
    setEditDivisionId('');
    setEditPosition(1);
    setError('');
    setSuccess('');
  };

  // Parse census data intelligently
  const parseCensusData = (text: string) => {
    const lines = text.split('\n').map(l => l.trim()).filter(l => l.length > 0);
    const players: Array<{
      id: string;
      name: string;
      position: string;
      birthYear: number;
      nationality: string;
      age?: number;
      relation?: string;
      gender?: string;
      whereBorn?: string;
      birthTown?: string;
      birthCounty?: string;
      birthCountry?: string;
      civilParish?: string;
      ecclesiasticalParish?: string;
      registrationDistrict?: string;
      subRegistrationDistrict?: string;
      edInstitution?: string;
      householdScheduleNumber?: string;
      piece?: string;
      folio?: string;
      pageNumber?: string;
    }> = [];

    let currentRecord: any = {};
    let hasStartedRecord = false;

    const saveCurrentRecord = () => {
      if (hasStartedRecord && currentRecord.name) {
        console.log('Saving player record:', currentRecord);
        players.push({
          id: `manual_${Date.now()}_${Math.random()}`,
          name: currentRecord.name,
          position: 'FWD',
          birthYear: currentRecord.birthYear || year - 25,
          age: currentRecord.age,
          relation: currentRecord.relation,
          gender: currentRecord.gender,
          nationality: currentRecord.nationality || 'English',
          whereBorn: currentRecord.whereBorn,
          birthTown: currentRecord.birthTown,
          birthCounty: currentRecord.birthCounty,
          birthCountry: currentRecord.birthCountry,
          civilParish: currentRecord.civilParish,
          ecclesiasticalParish: currentRecord.ecclesiasticalParish,
          registrationDistrict: currentRecord.registrationDistrict,
          subRegistrationDistrict: currentRecord.subRegistrationDistrict,
          edInstitution: currentRecord.edInstitution,
          householdScheduleNumber: currentRecord.householdScheduleNumber,
          piece: currentRecord.piece,
          folio: currentRecord.folio,
          pageNumber: currentRecord.pageNumber,
        });
      } else if (hasStartedRecord) {
        console.log('Skipping record without name:', currentRecord);
      }
    };

    for (const line of lines) {
      // Match "Field\tValue" or "Field    Value" patterns (tab or multiple spaces)
      const parts = line.split(/\t/).map(p => p.trim());

      if (parts.length >= 2) {
        const field = parts[0].toLowerCase().replace(/:\s*$/, ''); // Remove trailing colon
        const value = parts.slice(1).join('\t').trim();
        console.log(`Parsed field: "${field}" = "${value}"`);

        // Key fields we care about
        if (field === 'name') {
          // Save previous record before starting a new one
          saveCurrentRecord();
          // Start new record
          currentRecord = { name: value };
          hasStartedRecord = true;
        }
        else if (field === 'age') {
          const age = parseInt(value);
          if (!isNaN(age)) {
            if (!hasStartedRecord) {
              currentRecord = { name: 'Unknown Player' };
              hasStartedRecord = true;
            }
            currentRecord.age = age;
            if (!currentRecord.birthYear) currentRecord.birthYear = year - age;
          }
        }
        else if (field === 'estimated birth year' || field === 'birth year') {
          const birthYear = parseInt(value);
          if (!isNaN(birthYear) && hasStartedRecord) {
            currentRecord.birthYear = birthYear;
          }
        }
        else if (field === 'where born') {
          if (hasStartedRecord) {
            currentRecord.whereBorn = value;
            // Parse location parts: "Sheffield, Yorkshire, England"
            const parts = value.split(',').map(p => p.trim());
            if (parts.length >= 1) currentRecord.birthTown = parts[0];
            if (parts.length >= 2) currentRecord.birthCounty = parts[1];
            if (parts.length >= 3) currentRecord.birthCountry = parts[2];

            // Extract nationality from where born
            if (value.includes('England')) currentRecord.nationality = 'English';
            else if (value.includes('Scotland')) currentRecord.nationality = 'Scottish';
            else if (value.includes('Wales')) currentRecord.nationality = 'Welsh';
            else if (value.includes('Ireland')) currentRecord.nationality = 'Irish';
            else currentRecord.nationality = 'English';
          }
        }
        else if (field === 'town') {
          if (hasStartedRecord) currentRecord.birthTown = value;
        }
        else if (field === 'county' || field === 'county/island') {
          if (hasStartedRecord) currentRecord.birthCounty = value;
        }
        else if (field === 'country') {
          if (hasStartedRecord) currentRecord.birthCountry = value;
        }
        else if (field === 'civil parish') {
          if (hasStartedRecord) currentRecord.civilParish = value;
        }
        else if (field === 'ecclesiastical parish') {
          if (hasStartedRecord) currentRecord.ecclesiasticalParish = value;
        }
        else if (field === 'registration district') {
          if (hasStartedRecord) currentRecord.registrationDistrict = value;
        }
        else if (field === 'sub-registration district') {
          if (hasStartedRecord) currentRecord.subRegistrationDistrict = value;
        }
        else if (field === 'relation') {
          if (hasStartedRecord) currentRecord.relation = value;
        }
        else if (field === 'gender' || field === 'sex') {
          if (hasStartedRecord) currentRecord.gender = value;
        }
        else if (field === 'ed, institution, or vessel' || field === 'ed' || field === 'institution') {
          if (hasStartedRecord) currentRecord.edInstitution = value;
        }
        else if (field === 'household schedule number' || field === 'schedule number') {
          if (hasStartedRecord) currentRecord.householdScheduleNumber = value;
        }
        else if (field === 'piece') {
          if (hasStartedRecord) currentRecord.piece = value;
        }
        else if (field === 'folio') {
          if (hasStartedRecord) currentRecord.folio = value;
        }
        else if (field === 'page number' || field === 'page') {
          if (hasStartedRecord) {
            currentRecord.pageNumber = value;
            saveCurrentRecord();
            currentRecord = {};
            hasStartedRecord = false;
          }
        }
        else if (field === 'household members') {
          saveCurrentRecord();
          currentRecord = {};
          hasStartedRecord = false;
        }
        // Successfully parsed tab-separated format, continue to next line
        continue;
      } else {
        // Try to match colon-separated format: "Field: Value" or "Field:    Value"
        const colonMatch = line.match(/^([^:]+):\s*(.+)$/);
        if (colonMatch) {
          const field = colonMatch[1].trim().toLowerCase();
          const value = colonMatch[2].trim();

          if (field === 'name') {
            saveCurrentRecord();
            currentRecord = { name: value };
            hasStartedRecord = true;
          } else if (field === 'age') {
            const age = parseInt(value);
            if (!isNaN(age)) {
              if (!hasStartedRecord) { currentRecord = { name: 'Unknown Player' }; hasStartedRecord = true; }
              currentRecord.age = age;
              if (!currentRecord.birthYear) currentRecord.birthYear = year - age;
            }
          } else if (field === 'estimated birth year' || field === 'birth year') {
            const birthYear = parseInt(value);
            if (!isNaN(birthYear) && hasStartedRecord) currentRecord.birthYear = birthYear;
          } else if (field === 'where born') {
            if (hasStartedRecord) {
              currentRecord.whereBorn = value;
              const loc = value.split(',').map((p: string) => p.trim());
              if (loc.length >= 1) currentRecord.birthTown = loc[0];
              if (loc.length >= 2) currentRecord.birthCounty = loc[1];
              if (loc.length >= 3) currentRecord.birthCountry = loc[2];
              if (value.includes('England')) currentRecord.nationality = 'English';
              else if (value.includes('Scotland')) currentRecord.nationality = 'Scottish';
              else if (value.includes('Wales')) currentRecord.nationality = 'Welsh';
              else if (value.includes('Ireland')) currentRecord.nationality = 'Irish';
              else currentRecord.nationality = 'English';
            }
          } else if (field === 'town') {
            if (hasStartedRecord) currentRecord.birthTown = value;
          } else if (field === 'county' || field === 'county/island') {
            if (hasStartedRecord) currentRecord.birthCounty = value;
          } else if (field === 'country') {
            if (hasStartedRecord) currentRecord.birthCountry = value;
          } else if (field === 'civil parish') {
            if (hasStartedRecord) currentRecord.civilParish = value;
          } else if (field === 'ecclesiastical parish') {
            if (hasStartedRecord) currentRecord.ecclesiasticalParish = value;
          } else if (field === 'registration district') {
            if (hasStartedRecord) currentRecord.registrationDistrict = value;
          } else if (field === 'sub-registration district') {
            if (hasStartedRecord) currentRecord.subRegistrationDistrict = value;
          } else if (field === 'relation') {
            if (hasStartedRecord) currentRecord.relation = value;
          } else if (field === 'gender' || field === 'sex') {
            if (hasStartedRecord) currentRecord.gender = value;
          } else if (field === 'ed, institution, or vessel' || field === 'ed' || field === 'institution') {
            if (hasStartedRecord) currentRecord.edInstitution = value;
          } else if (field === 'household schedule number' || field === 'schedule number') {
            if (hasStartedRecord) currentRecord.householdScheduleNumber = value;
          } else if (field === 'piece') {
            if (hasStartedRecord) currentRecord.piece = value;
          } else if (field === 'folio') {
            if (hasStartedRecord) currentRecord.folio = value;
          } else if (field === 'page number' || field === 'page') {
            if (hasStartedRecord) {
              currentRecord.pageNumber = value;
              saveCurrentRecord();
              currentRecord = {};
              hasStartedRecord = false;
            }
          } else if (field === 'household members') {
            saveCurrentRecord();
            currentRecord = {};
            hasStartedRecord = false;
          }
        } else {
          // Try to match tabular format: "Name    Age    Birth Year..."
          const parts = line.split(/\s{2,}|\t/).map(p => p.trim()).filter(p => p.length > 0);

        if (parts.length >= 2) {
          const name = parts[0];
          const secondCol = parts[1];

          // Skip header rows
          if (name.toLowerCase().includes('name') || name.toLowerCase().includes('view') || name.toLowerCase() === 'age') {
            continue;
          }

          // Try to parse age or birth year from second column
          const numVal = parseInt(secondCol);
          let birthYear = year - 25; // default

          if (!isNaN(numVal)) {
            if (numVal > 1800 && numVal < year) {
              // It's a birth year
              birthYear = numVal;
            } else if (numVal < 100) {
              // It's an age
              birthYear = year - numVal;
            }
          }

          // Try to find birth place in remaining columns
          let birthPlace = parts.slice(2).join(' ');
          let nationality = 'English';
          if (birthPlace?.includes('England')) nationality = 'English';
          else if (birthPlace?.includes('Scotland')) nationality = 'Scottish';
          else if (birthPlace?.includes('Wales')) nationality = 'Welsh';
          else if (birthPlace?.includes('Ireland')) nationality = 'Irish';

          players.push({
            id: `manual_${Date.now()}_${Math.random()}`,
            name: name,
            position: 'FWD',
            birthYear: birthYear,
            nationality: nationality
          });
        }
      }
    }
    }

    // Don't forget the last record if we're still building one
    saveCurrentRecord();

    console.log('Parsed players:', players);
    return players;
  };

  // Parse "View Record" format from census website
  const parseViewRecordFormat = (text: string) => {
    const players: Array<{
      id: string;
      name: string;
      position: string;
      birthYear: number;
      nationality: string;
      whereBorn?: string;
      birthTown?: string;
      birthCounty?: string;
      birthCountry?: string;
      civilParish?: string;
      ecclesiasticalParish?: string;
      registrationDistrict?: string;
      subRegistrationDistrict?: string;
    }> = [];

    // Split by "View Record" to get individual records
    const records = text.split(/View Record/i).filter(r => r.trim().length > 0);

    for (const record of records) {
      const lines = record.split('\n').map(l => l.trim()).filter(l => l.length > 0);

      // Skip header lines
      if (lines.length === 0 || lines[0].toLowerCase().includes('name') || lines[0].toLowerCase().includes('search results')) {
        continue;
      }

      // Expected format:
      // Line 0: Name
      // Line 1: Parent or Spouse Names (optional, might be empty or have multiple names)
      // Line 2 or 3: Birth Year (4 digits)
      // Next: Birth Place (contains "England", "Yorkshire", etc.)
      // Next: Relation (Head, Son, etc.)
      // Next: Residence Place (e.g., "Brightside Bierlow, Yorkshire")

      let name = '';
      let birthYear = year - 25; // default
      let birthPlace = '';
      let residencePlace = '';

      for (let i = 0; i < lines.length; i++) {
        const line = lines[i];

        // First non-empty line should be the name
        if (i === 0 && !line.toLowerCase().includes('primary') && !line.toLowerCase().includes('view image')) {
          name = line;
        }
        // Look for 4-digit birth year
        else if (/^\d{4}$/.test(line)) {
          const year = parseInt(line);
          if (year > 1800 && year < 1900) {
            birthYear = year;
          }
        }
        // Look for birth place (contains commas and location names)
        else if (line.includes(',') && (line.includes('England') || line.includes('Yorkshire') || line.includes('Scotland') || line.includes('Wales') || line.includes('Ireland'))) {
          if (!birthPlace) {
            birthPlace = line;
          }
        }
        // Look for residence place (usually contains "Bierlow" or "Yorkshire" after relation)
        else if ((line.includes('Bierlow') || line.includes('Yorkshire') || line.includes('Dewsbury') || line.includes('Wakefield') || line.includes('Rotherham')) && !birthPlace.includes(line)) {
          residencePlace = line;
        }
      }

      // Only add if we have a valid name
      if (name && name.length > 0) {
        // Parse birth place
        const placeParts = birthPlace.split(',').map(p => p.trim());
        const birthTown = placeParts[0] || undefined;
        const birthCounty = placeParts[1] || undefined;
        const birthCountry = placeParts[2] || undefined;

        // Parse residence place for civil parish
        const residenceParts = residencePlace.split(',').map(p => p.trim());
        const civilParish = residenceParts[0] || undefined;

        // Determine nationality
        let nationality = 'English';
        if (birthPlace.includes('Scotland')) nationality = 'Scottish';
        else if (birthPlace.includes('Wales')) nationality = 'Welsh';
        else if (birthPlace.includes('Ireland')) nationality = 'Irish';

        players.push({
          id: `manual_${Date.now()}_${Math.random()}`,
          name: name,
          position: 'FWD',
          birthYear: birthYear,
          nationality: nationality,
          whereBorn: birthPlace || undefined,
          birthTown: birthTown,
          birthCounty: birthCounty,
          birthCountry: birthCountry,
          civilParish: civilParish
        });
      }
    }

    console.log('Parsed View Record format players:', players);
    return players;
  };

  // Parse line-by-line format (7 lines per record ending with Yorkshire)
  const parseLineByLineFormat = (text: string) => {
    const players: Array<{
      id: string;
      name: string;
      position: string;
      birthYear: number;
      nationality: string;
      whereBorn?: string;
      birthTown?: string;
      birthCounty?: string;
      birthCountry?: string;
      civilParish?: string;
      ecclesiasticalParish?: string;
      registrationDistrict?: string;
      subRegistrationDistrict?: string;
    }> = [];

    const lines = text.split('\n').map(l => l.trim()).filter(l => l.length > 0);

    let i = 0;
    while (i < lines.length) {
      // Each record is 7 lines:
      // 0: First name(s) and possibly middle name(s)
      // 1: Last name
      // 2: Birth year
      // 3: Birth county
      // 4: Parish
      // 5: Registration district
      // 6: County (contains "Yorkshire")

      if (i + 6 < lines.length) {
        const firstNameLine = lines[i];
        const lastName = lines[i + 1];
        const birthYearStr = lines[i + 2];
        const birthCounty = lines[i + 3];
        const parish = lines[i + 4];
        const registrationDistrict = lines[i + 5];
        const countyLine = lines[i + 6];

        // Validate this is a record ending (should contain "Yorkshire")
        if (countyLine.includes('Yorkshire')) {
          const birthYear = parseInt(birthYearStr);

          if (!isNaN(birthYear) && firstNameLine && lastName) {
            // Parse first name(s) - could be "George", "Richard C", "Amos Joseph", "Joseph J R"
            const nameParts = firstNameLine.trim().split(/\s+/);
            const fullName = `${firstNameLine} ${lastName}`.trim();

            players.push({
              id: `manual_${Date.now()}_${Math.random()}`,
              name: fullName,
              position: 'FWD',
              birthYear: birthYear,
              nationality: 'English',
              birthCounty: birthCounty || undefined,
              civilParish: registrationDistrict || undefined,
              ecclesiasticalParish: parish || undefined,
              registrationDistrict: registrationDistrict || undefined
            });
          }

          // Move to next record (7 lines forward)
          i += 7;
        } else {
          // Not a valid record, skip this line
          i++;
        }
      } else {
        // Not enough lines left for a complete record
        break;
      }
    }

    console.log('Parsed line-by-line format players:', players);
    return players;
  };

  // Handle paste of census data
  // Parse vertical census format (7 lines per record, no header required)
  const parseVerticalCensusFormat = (text: string) => {
    const players: Array<{
      id: string;
      name: string;
      position: string;
      birthYear: number;
      nationality: string;
      whereBorn?: string;
      birthTown?: string;
      birthCounty?: string;
      birthCountry?: string;
      civilParish?: string;
      ecclesiasticalParish?: string;
      registrationDistrict?: string;
      subRegistrationDistrict?: string;
    }> = [];

    const lines = text.split('\n').map(l => l.trim()).filter(l => l.length > 0);

    if (lines.length < 7) return players;

    let i = 0;

    while (i + 6 < lines.length) {
      // Each record has 7 lines: firstName, lastName, birthYear, birthCounty, parish, registrationDistrict, county
      const firstName = lines[i];
      const lastName = lines[i + 1];
      const birthYearStr = lines[i + 2];
      const birthCounty = lines[i + 3];
      const parish = lines[i + 4];
      const registrationDistrict = lines[i + 5];
      const county = lines[i + 6];

      // Parse birth year
      const birthYear = parseInt(birthYearStr);

      // Skip if birth year is invalid
      if (isNaN(birthYear) || birthYear < 1800 || birthYear > 1900) {
        i++;
        continue;
      }

      // Construct full name
      const name = `${firstName} ${lastName}`.trim();
      if (!name) {
        i++;
        continue;
      }

      // Determine nationality from birth county
      let nationality = 'English';
      const birthPlace = birthCounty.toLowerCase();
      if (birthPlace.includes('scotland')) nationality = 'Scottish';
      else if (birthPlace.includes('wales')) nationality = 'Welsh';
      else if (birthPlace.includes('ireland')) nationality = 'Irish';

      players.push({
        id: `manual_${Date.now()}_${Math.random()}`,
        name: name,
        position: 'FWD',
        birthYear: birthYear,
        nationality: nationality,
        birthCounty: birthCounty,
        civilParish: parish,
        ecclesiasticalParish: parish,
        registrationDistrict: registrationDistrict,
        birthCountry: county
      });

      // Move to next record (7 lines per record)
      i += 7;
    }

    console.log('Parsed vertical census format players:', players);
    return players;
  };

  // Parse tab-separated table format with headers - vertical format where each field is on its own line
  const parseTabSeparatedTable = (text: string) => {
    const players: Array<{
      id: string;
      name: string;
      position: string;
      birthYear: number;
      nationality: string;
      whereBorn?: string;
      birthTown?: string;
      birthCounty?: string;
      birthCountry?: string;
      civilParish?: string;
      ecclesiasticalParish?: string;
      registrationDistrict?: string;
      subRegistrationDistrict?: string;
    }> = [];

    const lines = text.split('\n').map(l => l.trim()).filter(l => l.length > 0);

    if (lines.length < 8) return players; // Need at least header + 7 fields per record

    // Find header line
    const headerIndex = lines.findIndex(l => l.includes('First name') && l.includes('Last name'));
    if (headerIndex === -1) return players;

    // Data starts after header line
    let i = headerIndex + 1;

    while (i < lines.length) {
      // Each record has 7 lines: firstName, lastName, birthYear, birthCounty, parish, registrationDistrict, county
      if (i + 6 >= lines.length) break;

      const firstName = lines[i];
      const lastName = lines[i + 1];
      const birthYearStr = lines[i + 2];
      const birthCounty = lines[i + 3];
      const parish = lines[i + 4];
      const registrationDistrict = lines[i + 5];
      const county = lines[i + 6];

      // Parse birth year
      const birthYear = parseInt(birthYearStr);

      // Skip if birth year is invalid
      if (isNaN(birthYear) || birthYear < 1800 || birthYear > 1900) {
        i++;
        continue;
      }

      // Construct full name
      const name = `${firstName} ${lastName}`.trim();
      if (!name) {
        i++;
        continue;
      }

      // Determine nationality from birth county
      let nationality = 'English';
      const birthPlace = birthCounty.toLowerCase();
      if (birthPlace.includes('scotland')) nationality = 'Scottish';
      else if (birthPlace.includes('wales')) nationality = 'Welsh';
      else if (birthPlace.includes('ireland')) nationality = 'Irish';
      else if (birthPlace.includes('yorkshire')) nationality = 'English';
      else if (birthPlace.includes('nottinghamshire')) nationality = 'English';
      else if (birthPlace.includes('derbyshire')) nationality = 'English';
      else if (birthPlace.includes('leicestershire')) nationality = 'English';

      players.push({
        id: `manual_${Date.now()}_${Math.random()}`,
        name: name,
        position: 'FWD',
        birthYear: birthYear,
        nationality: nationality,
        birthCounty: birthCounty,
        civilParish: parish,
        ecclesiasticalParish: parish,
        registrationDistrict: registrationDistrict,
        birthCountry: county
      });

      // Move to next record (7 lines per record)
      i += 7;
    }

    console.log('Parsed tab-separated table players:', players);
    return players;
  };

  // Parse horizontal TSV format: one row per person, header row with uppercase column names
  // e.g. AGE \t Birth Date \t Birth Place \t CIVIL PARISH \t ... \t Name \t ...
  const parseHorizontalTsvFormat = (text: string) => {
    const players: typeof manualPlayers = [];

    // Split into lines, keeping quoted fields intact
    const rawLines = text.split('\n').filter(l => l.trim().length > 0);
    if (rawLines.length < 2) return players;

    // Parse a TSV line, respecting double-quoted fields that may contain tabs
    const parseTsvLine = (line: string): string[] => {
      const fields: string[] = [];
      let current = '';
      let inQuotes = false;
      for (let i = 0; i < line.length; i++) {
        const ch = line[i];
        if (ch === '"') {
          inQuotes = !inQuotes;
        } else if (ch === '\t' && !inQuotes) {
          fields.push(current.trim());
          current = '';
        } else {
          current += ch;
        }
      }
      fields.push(current.trim());
      return fields;
    };

    const headers = parseTsvLine(rawLines[0]).map(h => h.toLowerCase().trim());

    // Map header names to indices - find by exact match first, then partial
    const idx = (name: string) => {
      const lower = name.toLowerCase();
      const exact = headers.indexOf(lower);
      if (exact >= 0) return exact;
      return headers.findIndex(h => h === lower);
    };
    const iAge = idx('age');
    const iBirthDate = idx('birth date');
    const iBirthPlace = idx('birth place');
    const iCivilParish = idx('civil parish');
    const iCountry = idx('country');
    const iCounty = idx('county/island');
    const iEcclParish = idx('ecclesiastical parish');
    const iEd = idx('ed, institution, or vessel');
    const iBirthYear = idx('estimated birth year');
    const iFolio = idx('folio');
    const iGender = idx('gender');
    const iHhMembers = idx('household members');
    const iHhSched = idx('household schedule number');
    const iHouseholdHead = headers.lastIndexOf('name'); // 'NAME' (uppercase) - household head
    // 'Name' (title-case) is the individual - find by checking both slots
    const iNameUpper = headers.indexOf('name');
    // There are two 'name' columns after lowercasing; the second one is the individual
    const iName = (() => {
      let count = 0;
      for (let i = 0; i < headers.length; i++) {
        if (headers[i] === 'name') { count++; if (count === 2) return i; }
      }
      return iNameUpper; // fallback to first if only one
    })();
    const iPageNum = idx('page number');
    const iPiece = idx('piece');
    const iRegDist = idx('registration district');
    const iRelation = idx('relation');
    const iSubReg = idx('sub-registration district');
    const iTown = idx('town');
    const iWhereBorn = idx('where born');

    for (let r = 1; r < rawLines.length; r++) {
      const cols = parseTsvLine(rawLines[r]);
      const get = (i: number) => (i >= 0 && i < cols.length) ? cols[i] : '';

      const name = get(iName);
      if (!name) continue;

      const birthYearStr = get(iBirthYear);
      const birthYear = parseInt(birthYearStr);
      if (isNaN(birthYear) || birthYear < 1800 || birthYear > 1900) continue;

      const ageStr = get(iAge);
      const age = parseInt(ageStr) || undefined;

      // whereBorn: prefer WHERE BORN column, fall back to Birth Place
      const whereBorn = get(iWhereBorn) || get(iBirthPlace) || undefined;

      const birthCountry = get(iCountry);
      let nationality = 'English';
      if (birthCountry.toLowerCase().includes('scotland')) nationality = 'Scottish';
      else if (birthCountry.toLowerCase().includes('wales')) nationality = 'Welsh';
      else if (birthCountry.toLowerCase().includes('ireland')) nationality = 'Irish';

      players.push({
        id: `manual_${Date.now()}_${Math.random()}`,
        name,
        position: 'FWD',
        birthYear,
        nationality,
        age,
        gender: get(iGender) || undefined,
        relation: get(iRelation) || undefined,
        whereBorn,
        birthTown: get(iTown) || undefined,
        birthCounty: get(iCounty) || undefined,
        birthCountry: birthCountry || undefined,
        civilParish: get(iCivilParish) || undefined,
        ecclesiasticalParish: get(iEcclParish) || undefined,
        registrationDistrict: get(iRegDist) || undefined,
        subRegistrationDistrict: get(iSubReg) || undefined,
        edInstitution: get(iEd) || undefined,
        householdScheduleNumber: get(iHhSched) || undefined,
        piece: get(iPiece) || undefined,
        folio: get(iFolio) || undefined,
        pageNumber: get(iPageNum) || undefined,
      });
    }

    console.log('Parsed horizontal TSV format players:', players);
    return players;
  };

  const handlePasteCensusData = (pastedText: string) => {
    // Detect format - if it contains "View Record", use new parser
    let parsed;
    if (pastedText.includes('View Record')) {
      console.log('Detected View Record format');
      parsed = parseViewRecordFormat(pastedText);
    } else if (pastedText.includes('Yorkshire, Yorkshire (West Riding)')) {
      console.log('Detected line-by-line format');
      parsed = parseLineByLineFormat(pastedText);
    } else if (pastedText.includes('ESTIMATED BIRTH YEAR') || pastedText.includes('HOUSEHOLD SCHEDULE NUMBER')) {
      console.log('Detected horizontal TSV format');
      parsed = parseHorizontalTsvFormat(pastedText);
    } else if (pastedText.includes('First name(s)') && pastedText.includes('Last name') && pastedText.includes('Birth year')) {
      console.log('Detected tab-separated table format with header');
      parsed = parseTabSeparatedTable(pastedText);
    } else if (pastedText.match(/^[A-Z][a-z]+\n[A-Z][a-z]+\n\d{4}\n[A-Z][a-z]+/m)) {
      console.log('Detected vertical census format (7 lines per record)');
      parsed = parseVerticalCensusFormat(pastedText);
    } else {
      console.log('Using original parser');
      parsed = parseCensusData(pastedText);
    }

    // Apply default parish to all parsed players if checkbox is enabled and parish is selected
    const playersWithParish = (useDefaultParish && defaultParish)
      ? parsed.map(p => ({ ...p, ecclesiasticalParish: defaultParish }))
      : parsed;
    setManualPlayers(prev => [...prev, ...playersWithParish]);
    setPlayerNamesText(''); // Clear the text area
  };

  // Manual player table functions
  const addManualPlayerRow = () => {
    setManualPlayers(prev => [...prev, {
      id: `manual_${Date.now()}`,
      name: '',
      position: 'FWD',
      birthYear: year - 20,
      nationality: 'English',
      ecclesiasticalParish: (useDefaultParish && defaultParish) ? defaultParish : undefined
    }]);
  };

  const updateManualPlayer = (id: string, field: string, value: any) => {
    setManualPlayers(prev => prev.map(p =>
      p.id === id ? { ...p, [field]: value } : p
    ));
  };

  const removeManualPlayer = (id: string) => {
    setManualPlayers(prev => prev.filter(p => p.id !== id));
  };

  // Create players handler (shared between top and bottom buttons)
  const handleCreatePlayers = async () => {
    setLoading(true);
    setError('');
    setSuccess('');

    try {
      console.log('Creating players with data:', manualPlayers);

      // Filter 1: Remove players born after 1853
      const tooYoung: any[] = [];
      const eligibleByAge = manualPlayers.filter(manual => {
        if (manual.birthYear > 1853) {
          tooYoung.push(manual);
          return false;
        }
        return true;
      });

      // Load all existing players
      const existingPlayers = await invoke<any[]>('db_get_all_players');

      // Filter 2: Check for duplicates (same first_name, surname, and birth_year)
      const duplicates: Array<{manual: any, existing: any}> = [];

      eligibleByAge.forEach(manual => {
        // Parse the manual player's name into components
        const nameParts = manual.name.trim().split(/\s+/);
        let firstName = '';
        let surname = '';

        if (nameParts.length === 1) {
          firstName = nameParts[0];
        } else if (nameParts.length === 2) {
          firstName = nameParts[0];
          surname = nameParts[1];
        } else if (nameParts.length >= 3) {
          firstName = nameParts[0];
          surname = nameParts[nameParts.length - 1];
        }

        // Check against existing players
        const duplicate = existingPlayers.find(existing => {
          const firstMatch = (existing.first_name || '').toLowerCase() === firstName.toLowerCase();
          const surnameMatch = (existing.surname || '').toLowerCase() === surname.toLowerCase();
          const birthYearMatch = existing.birth_year === manual.birthYear;

          return firstMatch && surnameMatch && birthYearMatch;
        });

        if (duplicate) {
          duplicates.push({ manual, existing: duplicate });
        }
      });

      // Report filtering results
      let playersToProcess = eligibleByAge;
      let filterMessages: string[] = [];

      if (tooYoung.length > 0) {
        const tooYoungNames = tooYoung.map(p => `${p.name} (${p.birthYear})`).join('\n');
        filterMessages.push(`Filtered out ${tooYoung.length} player(s) born after 1853:\n${tooYoungNames}`);
      }

      // If duplicates found, ask for confirmation
      if (duplicates.length > 0) {
        const duplicateNames = duplicates.map(d =>
          `${d.manual.name} (${d.manual.birthYear})`
        ).join('\n');

        const duplicateIds = new Set(duplicates.map(d => d.manual.id));

        let message = '';
        if (filterMessages.length > 0) {
          message = filterMessages.join('\n\n') + '\n\n';
        }
        message += `Warning: ${duplicates.length} player(s) already exist in the database with the same name and birth year:\n\n${duplicateNames}\n\nClick OK to create ALL eligible players including duplicates\nClick Cancel to skip duplicates and create only the ${eligibleByAge.length - duplicates.length} new player(s)`;

        const createAll = confirm(message);

        if (!createAll) {
          // User chose to skip duplicates - filter them out
          playersToProcess = eligibleByAge.filter(p => !duplicateIds.has(p.id));

          if (playersToProcess.length === 0) {
            setError('All eligible players are duplicates. No players created.');
            setLoading(false);
            return;
          }

          filterMessages.push(`Skipped ${duplicates.length} duplicate(s)`);
        }
      } else if (tooYoung.length > 0) {
        // No duplicates but some filtered by age - inform user
        alert(filterMessages.join('\n\n') + `\n\nContinuing to create ${playersToProcess.length} eligible player(s).`);
      }

      // Show summary if any filtering occurred
      if (filterMessages.length > 0 && playersToProcess.length > 0) {
        const summary = filterMessages.join('. ') + `. Creating ${playersToProcess.length} player(s)...`;
        setSuccess(summary);
      }

      // Prepare players for backend (remove 'position' field as it's not needed for creation)
      const playersToCreate = playersToProcess.map(p => ({
        name: p.name,
        birth_year: p.birthYear,
        nationality: p.nationality || 'English',
        where_born: p.whereBorn,
        birth_town: p.birthTown,
        birth_county: p.birthCounty,
        birth_country: p.birthCountry,
        civil_parish: p.civilParish,
        ecclesiastical_parish: p.ecclesiasticalParish,
        registration_district: p.registrationDistrict,
        sub_registration_district: p.subRegistrationDistrict,
        census_age: p.age ?? null,
        census_relation: p.relation ?? null,
        census_gender: p.gender ?? null,
        census_ed: p.edInstitution ?? null,
        census_household_schedule: p.householdScheduleNumber ?? null,
        census_piece: p.piece ?? null,
        census_folio: p.folio ?? null,
        census_page: p.pageNumber ?? null,
      }));

      const playerIds = await invoke<string[]>('db_create_players_with_geographic_data', { players: playersToCreate });
      setSuccess(`Successfully created ${playerIds.length} player(s)! You can now assign stats in the "Assign Stats" tab.`);
      setManualPlayers([]);

    } catch (err) {
      setError(`Failed to create players: ${err}`);
      console.error('Error creating players:', err);
    } finally {
      setLoading(false);
    }
  };

  // Toggle club selection
  const toggleClubSelection = (clubId: string) => {
    setSelectedClubIds(prev => {
      if (prev.includes(clubId)) {
        return prev.filter(id => id !== clubId);
      } else {
        return [...prev, clubId];
      }
    });
  };

  // Save manual players to selected clubs
  const handleSaveManualPlayers = async () => {
    if (selectedClubIds.length === 0) {
      setError('Please select at least one club');
      return;
    }

    if (manualPlayers.length === 0) {
      setError('Please add at least one player');
      return;
    }

    const invalidPlayers = manualPlayers.filter(p => !p.name.trim());
    if (invalidPlayers.length > 0) {
      setError('Please fill in all player names');
      return;
    }

    setLoading(true);
    setError('');
    setSuccess('');

    try {
      let totalCreated = 0;

      for (const clubId of selectedClubIds) {
        for (const player of manualPlayers) {
          await invoke('db_create_player_simple', {
            clubId: clubId,
            name: player.name.trim(),
            position: player.position,
            birthYear: player.birthYear,
            year: year,
            hasGoalkeeper: true
          });
          totalCreated++;
        }
      }

      setSuccess(`Successfully created ${manualPlayers.length} player(s) for ${selectedClubIds.length} club(s) (${totalCreated} total)`);
      setManualPlayers([]);
      setSelectedClubIds([]);
    } catch (err) {
      setError(`Failed to save players: ${err}`);
    } finally {
      setLoading(false);
    }
  };

  const handleGeneratePlayers = async () => {
    setError('');
    setSuccess('');
    setLoading(true);

    try {
      const playerData = playerNamesText
        .split('\n')
        .map(line => line.trim())
        .filter(line => line.length > 0)
        .map(line => {
          let name = line;
          let age: number | null = null;

          const commaMatch = line.match(/^(.+?),\s*(\d+)$/);
          if (commaMatch) {
            name = commaMatch[1].trim();
            age = parseInt(commaMatch[2]);
          } else {
            const dashMatch = line.match(/^(.+?)\s*-\s*(\d+)$/);
            if (dashMatch) {
              name = dashMatch[1].trim();
              age = parseInt(dashMatch[2]);
            } else {
              const parenMatch = line.match(/^(.+?)\s*\((\d+)\)$/);
              if (parenMatch) {
                name = parenMatch[1].trim();
                age = parseInt(parenMatch[2]);
              } else {
                const spaceMatch = line.match(/^(.+?)\s+(\d+)$/);
                if (spaceMatch) {
                  name = spaceMatch[1].trim();
                  age = parseInt(spaceMatch[2]);
                }
              }
            }
          }

          return { name, age };
        });

      if (playerData.length === 0) {
        setError('Please enter at least one player name');
        setLoading(false);
        return;
      }

      if (!selectedClubId) {
        setError('Please select a club');
        setLoading(false);
        return;
      }

      const playerInputs = playerData.map(({ name, age }) => ({
        name,
        birthYear: age !== null ? year - age : null
      }));

      const players = await invoke<GeneratedPlayer[]>('db_bulk_add_players_with_ages', {
        clubId: selectedClubId,
        playerInputs: playerInputs,
        year: year,
        hasGoalkeeper: hasGoalkeeper,
      });

      setGeneratedPlayers(players);
      setSuccess(`Successfully generated ${players.length} players!`);
      setPlayerNamesText('');
    } catch (err) {
      setError(`Failed to generate players: ${err}`);
    } finally {
      setLoading(false);
    }
  };

  // Cup Management Handlers
  const loadCompetitions = async () => {
    try {
      const comps = await invoke<Competition[]>('db_get_competitions_for_season', { season: cupSeason });
      setCompetitions(comps);
    } catch (err) {
      setError(`Failed to load competitions: ${err}`);
    }
  };

  const handleCreateAnnualCups = async () => {
    try {
      setLoading(true);
      setError('');
      setSuccess('');
      const createdComps = await invoke<Competition[]>('db_create_annual_cups', {
        season: cupSeason,
        startWeek: cupStartWeek
      });
      setSuccess(`Created ${createdComps.length} cup competitions (Youdan Cup & Cromwell Cup)`);
      await loadCompetitions();
    } catch (err) {
      setError(`Failed to create cups: ${err}`);
    } finally {
      setLoading(false);
    }
  };

  const handleLoadEligibleClubs = async (compId: string) => {
    try {
      const clubs = await invoke<EligibleClub[]>('db_get_eligible_clubs', { competitionId: compId });
      setEligibleClubs(clubs);
    } catch (err) {
      setError(`Failed to load eligible clubs: ${err}`);
    }
  };

  const handleGenerateDraw = async () => {
    if (!selectedCompetitionId) {
      setError('Please select a competition first');
      return;
    }
    try {
      setLoading(true);
      setError('');
      setSuccess('');
      await invoke('db_generate_cup_draw', {
        competitionId: selectedCompetitionId,
        seeded: cupSeeded
      });
      setSuccess('Draw generated successfully!');
      await handleLoadBracket(selectedCompetitionId);
    } catch (err) {
      setError(`Failed to generate draw: ${err}`);
    } finally {
      setLoading(false);
    }
  };

  const handleLoadBracket = async (compId: string) => {
    try {
      const bracket = await invoke<CupTie[]>('db_get_cup_bracket', { competitionId: compId });
      setCupBracket(bracket);
    } catch (err) {
      setError(`Failed to load bracket: ${err}`);
    }
  };

  const handleDeleteCups = async () => {
    try {
      setLoading(true);
      setError('');
      setSuccess('');
      await invoke('db_delete_competitions_for_season', { season: cupSeason });
      setSuccess('All cup competitions for this season have been deleted');
      setCompetitions([]);
      setEligibleClubs([]);
      setCupBracket([]);
      setSelectedCompetitionId('');
    } catch (err) {
      setError(`Failed to delete competitions: ${err}`);
    } finally {
      setLoading(false);
    }
  };

  const selectedClub = clubs.find(c => c.id === selectedClubId);

  // Backup functions
  const loadBackups = async () => {
    try {
      const backupList = await invoke<any[]>('db_list_backups');
      setBackups(backupList);
    } catch (err) {
      console.error('Error loading backups:', err);
    }
  };

  const createBackup = async () => {
    try {
      setLoading(true);
      const backupPath = await invoke<string>('db_create_backup');
      setSuccess(`Backup created: ${backupPath}`);
      await loadBackups();
      setTimeout(() => setSuccess(''), 3000);
    } catch (err) {
      setError(`Failed to create backup: ${err}`);
    } finally {
      setLoading(false);
    }
  };

  const restoreBackup = async (backupPath: string) => {
    setConfirmDialog({
      isOpen: true,
      title: 'Restore Backup',
      message: `Are you sure you want to restore from this backup? The current database will be backed up first, but this operation cannot be undone.`,
      danger: true,
      onConfirm: async () => {
        try {
          setLoading(true);
          await invoke('db_restore_backup', { backupPath });
          setSuccess('Backup restored successfully! Please reload the page.');
          setTimeout(() => window.location.reload(), 2000);
        } catch (err) {
          setError(`Failed to restore backup: ${err}`);
        } finally {
          setLoading(false);
          setConfirmDialog(prev => ({ ...prev, isOpen: false }));
        }
      }
    });
  };

  const handleDeletePlayerWithConfirm = (playerId: string, playerName: string) => {
    setConfirmDialog({
      isOpen: true,
      title: 'Delete Player',
      message: `Are you sure you want to delete "${playerName}"? This action cannot be undone.`,
      danger: true,
      onConfirm: () => {
        deletePlayer(playerId);
        setConfirmDialog(prev => ({ ...prev, isOpen: false }));
      }
    });
  };

  const handleCsvImport = (content: string, filename: string) => {
    console.log(`Importing CSV file: ${filename}`);
    // Use existing parser - it already handles the census CSV format!
    handlePasteCensusData(content);
    setSuccess(`Imported data from ${filename}`);
    setTimeout(() => setSuccess(''), 3000);
  };

  // Horizontal scroll handlers for player table
  const updateScrollButtons = () => {
    if (tableScrollRef.current) {
      const { scrollLeft, scrollWidth, clientWidth } = tableScrollRef.current;
      setCanScrollLeft(scrollLeft > 0);
      setCanScrollRight(scrollLeft < scrollWidth - clientWidth - 1);
    }
  };

  const scrollTable = (direction: 'left' | 'right') => {
    if (tableScrollRef.current) {
      const scrollAmount = 300; // Scroll 300px at a time
      const newScrollLeft = direction === 'left'
        ? tableScrollRef.current.scrollLeft - scrollAmount
        : tableScrollRef.current.scrollLeft + scrollAmount;

      tableScrollRef.current.scrollTo({
        left: newScrollLeft,
        behavior: 'smooth'
      });
    }
  };

  const handleBulkCreatePlayersWithBackup = async () => {
    // Create backup before bulk operation
    setConfirmDialog({
      isOpen: true,
      title: 'Create Players',
      message: `Create ${manualPlayers.length} player(s)? A backup will be created automatically before this operation.`,
      onConfirm: async () => {
        try {
          setLoading(true);
          setProgressMessage('Creating backup...');
          await invoke('db_create_backup');

          setProgressMessage('Creating players...');
          await handleCreatePlayers();

          setSuccess(`Created ${manualPlayers.length} players successfully!`);
          setTimeout(() => setSuccess(''), 3000);
        } catch (err) {
          setError(`Operation failed: ${err}`);
        } finally {
          setLoading(false);
          setProgressMessage('');
          setConfirmDialog(prev => ({ ...prev, isOpen: false }));
        }
      }
    });
  };

  const handleBulkImportAllCensusFiles = async () => {
    setConfirmDialog({
      isOpen: true,
      title: 'Bulk Import All Census Files',
      message: `This will import ALL 73 census CSV files (1772-1844) directly into the database. This will add approximately 2000+ players. A backup will be created automatically. Continue?`,
      onConfirm: async () => {
        try {
          setLoading(true);
          setProgressMessage('Creating backup before import...');
          await invoke('db_create_backup');

          setProgressMessage('Importing all census files... This may take 1-2 minutes...');
          const stats = await invoke<any>('db_import_all_census_files');

          setSuccess(`Import complete! ${stats.total_players} players from ${stats.successful_files}/${stats.total_files} files imported successfully!`);

          if (stats.failed_files > 0) {
            console.error('Failed files:', stats.errors);
            const errorDetails = stats.errors.slice(0, 5).join('\n');
            const moreErrors = stats.errors.length > 5 ? `\n...and ${stats.errors.length - 5} more errors` : '';
            setError(`${stats.failed_files} files failed to import:\n\n${errorDetails}${moreErrors}`);
          }

          // Refresh player list if on player database tab
          if (activeTab === 'player-database') {
            await loadAllPlayers();
          }

          setTimeout(() => setSuccess(''), 5000);
        } catch (err) {
          setError(`Bulk import failed: ${err}`);
        } finally {
          setLoading(false);
          setProgressMessage('');
          setConfirmDialog(prev => ({ ...prev, isOpen: false }));
        }
      }
    });
  };

  // Get unique postcodes and areas for filter dropdowns
  console.log('ALL CLUBS COUNT:', clubs.length);
  console.log('filteredClubs length:', filteredClubs.length);
  console.log('First 3 ALL clubs:', clubs.slice(0, 3));
  console.log('First 3 clubs:', filteredClubs.slice(0, 3));
  console.log('where_from values FROM ALL CLUBS:', clubs.slice(0, 5).map(c => c.where_from));
  console.log('where_from values:', filteredClubs.slice(0, 5).map(c => c.where_from));
  const uniquePostcodes = Array.from(new Set(
    ['N/A', ...clubs
      .map(c => c.where_from)
      .filter((loc): loc is string => Boolean(loc))
      .flatMap(loc => {
        // Extract postcodes like S3, S4, S10, etc. from location strings
        const matches = loc.match(/\bS\d+\b/g);
        console.log('Location:', loc, 'Matches:', matches);
        return matches || [];
      })]
  )).sort();
  console.log('Unique postcodes found:', uniquePostcodes);
  const uniqueAreas = Array.from(new Set(filteredClubs.map(c => c.city).filter(Boolean))).sort();

  return (
    <div className="database-editor-container">
      <div className="database-editor-header">
        <h1>Sheffield Rules Database Editor</h1>
        <p>Manage clubs and players for the Sheffield & Hallamshire Fantasy League</p>
      </div>

      {/* Tab Navigation */}
      <div className="tab-navigation">
        <button
          className={`tab-button ${activeTab === 'clubs' ? 'active' : ''}`}
          onClick={() => setActiveTab('clubs')}
        >
          Club Management
        </button>
        <button
          className={`tab-button ${activeTab === 'create-players' ? 'active' : ''}`}
          onClick={() => setActiveTab('create-players')}
        >
          Create Players
        </button>
        <button
          className={`tab-button ${activeTab === 'assign-stats' ? 'active' : ''}`}
          onClick={() => setActiveTab('assign-stats')}
        >
          Assign Stats
        </button>
        <button
          className={`tab-button ${activeTab === 'assign-clubs' ? 'active' : ''}`}
          onClick={() => setActiveTab('assign-clubs')}
        >
          Assign to Clubs
        </button>
        <button
          className={`tab-button ${activeTab === 'player-database' ? 'active' : ''}`}
          onClick={() => setActiveTab('player-database')}
        >
          Player Database
        </button>
        <button
          className={`tab-button ${activeTab === 'parishes' ? 'active' : ''}`}
          onClick={() => setActiveTab('parishes')}
        >
          Parishes
        </button>
        <button
          className={`tab-button ${activeTab === 'club-capacity' ? 'active' : ''}`}
          onClick={() => setActiveTab('club-capacity')}
        >
          Club Capacity
        </button>
        <button
          className={`tab-button ${activeTab === 'squads' ? 'active' : ''}`}
          onClick={() => setActiveTab('squads')}
        >
          Squad Management
        </button>
        <button
          className={`tab-button ${activeTab === 'leagues' ? 'active' : ''}`}
          onClick={() => setActiveTab('leagues')}
        >
          League Management
        </button>
        <button
          className={`tab-button ${activeTab === 'cups' ? 'active' : ''}`}
          onClick={() => setActiveTab('cups')}
        >
          Cup Management
        </button>
      </div>

      <div className="database-editor-content">
        {/* CLUB MANAGEMENT TAB */}
        {activeTab === 'clubs' && (
          <>
            {/* Club Selector and Filters */}
            <div className="club-management-sidebar">
              <h2>Club Selection</h2>

              {/* Database Migration */}
              <div className="migration-section" style={{ marginBottom: '20px', padding: '10px', background: '#f0f0f0', borderRadius: '4px' }}>
                <button
                  onClick={async () => {
                    if (confirm('Run database migrations to add missing columns? This is safe and will not delete any data.')) {
                      setLoading(true);
                      try {
                        await invoke('db_run_schema_migrations');
                        setSuccess('Database migrations completed successfully!');
                      } catch (err) {
                        setError(`Migration failed: ${err}`);
                      } finally {
                        setLoading(false);
                      }
                    }
                  }}
                  style={{ width: '100%', padding: '8px', background: '#4CAF50', color: 'white', border: 'none', borderRadius: '4px', cursor: 'pointer' }}
                >
                  🔧 Run Database Migrations
                </button>
                <small style={{ display: 'block', marginTop: '5px', fontSize: '11px' }}>
                  Adds missing columns (where_born, has_stats, etc.)
                </small>
              </div>

              {/* Backup Management Section */}
              <div className="backup-section" style={{ marginTop: '15px', marginBottom: '20px', padding: '10px', background: '#f0f8ff', borderRadius: '4px', border: '1px solid #3498db' }}>
                <div style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', marginBottom: '10px' }}>
                  <strong style={{ color: '#2c3e50' }}>📦 Database Backups</strong>
                  <button
                    onClick={() => {
                      setShowBackupManager(!showBackupManager);
                      if (!showBackupManager) loadBackups();
                    }}
                    style={{ padding: '4px 12px', fontSize: '12px', background: '#3498db', color: 'white', border: 'none', borderRadius: '4px', cursor: 'pointer' }}
                  >
                    {showBackupManager ? 'Hide' : 'Show'} Backups
                  </button>
                </div>

                <button
                  onClick={createBackup}
                  disabled={loading}
                  style={{ width: '100%', padding: '8px', marginBottom: '10px', background: '#3498db', color: 'white', border: 'none', borderRadius: '4px', cursor: 'pointer' }}
                >
                  {loading ? 'Creating...' : '💾 Create Backup Now'}
                </button>

                {showBackupManager && (
                  <div style={{ maxHeight: '200px', overflow: 'auto', background: 'white', padding: '8px', borderRadius: '4px', border: '1px solid #ddd' }}>
                    <p style={{ fontSize: '12px', color: '#666', marginBottom: '8px' }}>
                      {backups.length} backup(s) available
                    </p>
                    {backups.map((backup: any) => (
                      <div key={backup.path} style={{ display: 'flex', justifyContent: 'space-between', alignItems: 'center', padding: '6px', background: '#f9f9f9', marginBottom: '4px', borderRadius: '3px', fontSize: '12px' }}>
                        <div>
                          <div style={{ fontWeight: 'bold' }}>{backup.filename}</div>
                          <div style={{ color: '#666', fontSize: '11px' }}>
                            {new Date(backup.created_timestamp * 1000).toLocaleString()} • {(backup.size / 1024 / 1024).toFixed(2)} MB
                          </div>
                        </div>
                        <button
                          onClick={() => restoreBackup(backup.path)}
                          style={{ padding: '4px 10px', background: '#2ecc71', color: 'white', border: 'none', borderRadius: '3px', cursor: 'pointer', fontSize: '11px' }}
                        >
                          Restore
                        </button>
                      </div>
                    ))}
                    {backups.length === 0 && (
                      <p style={{ textAlign: 'center', color: '#999', fontSize: '12px', padding: '10px' }}>
                        No backups yet. Click "Create Backup Now" to create one.
                      </p>
                    )}
                  </div>
                )}

                <small style={{ display: 'block', marginTop: '5px', fontSize: '11px', color: '#666' }}>
                  Backups are automatically created before bulk operations
                </small>
              </div>

              {/* Search and Filters */}
              <div className="filters-section">
                <div className="form-group">
                  <label htmlFor="search">Search:</label>
                  <input
                    id="search"
                    type="text"
                    value={searchText}
                    onChange={(e) => setSearchText(e.target.value)}
                    placeholder="Search clubs or grounds..."
                    className="filter-input"
                  />
                </div>

                <div className="form-group">
                  <label htmlFor="filter-postcode">Filter by Postcode:</label>
                  <select
                    id="filter-postcode"
                    value={filterPostcode}
                    onChange={(e) => setFilterPostcode(e.target.value)}
                    className="filter-select"
                  >
                    <option value="">All Postcodes</option>
                    {uniquePostcodes.map(pc => (
                      <option key={pc} value={pc}>{pc}</option>
                    ))}
                  </select>
                </div>

                <div className="form-group">
                  <label htmlFor="filter-area">Filter by Area:</label>
                  <select
                    id="filter-area"
                    value={filterArea}
                    onChange={(e) => setFilterArea(e.target.value)}
                    className="filter-select"
                  >
                    <option value="">All Areas</option>
                    {uniqueAreas.map(area => (
                      <option key={area} value={area}>{area}</option>
                    ))}
                  </select>
                </div>

                <div className="form-group">
                  <label className="checkbox-label">
                    <input
                      type="checkbox"
                      checked={hideReserves}
                      onChange={(e) => setHideReserves(e.target.checked)}
                    />
                    Hide Reserve Clubs
                  </label>
                </div>

                <div className="form-group">
                  <label className="checkbox-label">
                    <input
                      type="checkbox"
                      checked={hideNonReserves}
                      onChange={(e) => setHideNonReserves(e.target.checked)}
                    />
                    Hide Non-Reserve Clubs
                  </label>
                </div>

                <button
                  onClick={() => {
                    setSearchText('');
                    setFilterPostcode('');
                    setFilterArea('');
                    setHideReserves(true);
                    setHideNonReserves(false);
                  }}
                  className="clear-filters-button"
                >
                  Clear Filters
                </button>

                <button
                  onClick={handleCreateClub}
                  className="clear-filters-button"
                  style={{ marginTop: '10px', background: '#27ae60' }}
                >
                  Create New Club
                </button>
              </div>

              {/* Club List */}
              <div className="club-list-section">
                <p className="club-count">
                  Showing {filteredClubs.length} of {clubs.length} clubs
                </p>
                <div className="club-list">
                  {filteredClubs.map(club => (
                    <button
                      key={club.id}
                      className={`club-list-item ${selectedClubId === club.id ? 'selected' : ''}`}
                      onClick={() => {
                        setSelectedClubId(club.id);
                        setEditingName(false);
                        setEditingLocation(false);
                        setEditingDivision(false);
                        setError('');
                        setSuccess('');
                      }}
                    >
                      <div className="club-list-name">{club.name}</div>
                      <div className="club-list-details">
                        {club.founded_year} • {club.region || '?'}
                      </div>
                    </button>
                  ))}
                </div>
              </div>
            </div>

            {/* Club Details Panel */}
            <div className="club-details-panel">
              {selectedClub ? (
                <>
                  <h2>{selectedClub.name}</h2>

                  <div className="club-details-view">
                    <div className="detail-row">
                      <span className="detail-label">Founded:</span>
                      <span className="detail-value">{selectedClub.founded_year}</span>
                    </div>
                    <div className="detail-row">
                      <span className="detail-label">Ground:</span>
                      <span className="detail-value">{selectedClub.ground_name}</span>
                    </div>
                    <div className="detail-row">
                      <span className="detail-label">Area:</span>
                      <span className="detail-value">{selectedClub.city || <em className="missing">Not set</em>}</span>
                    </div>
                    <div className="detail-row">
                      <span className="detail-label">Postcode:</span>
                      <span className="detail-value">{selectedClub.region || <em className="missing">Not set</em>}</span>
                    </div>
                    <div className="detail-row">
                      <span className="detail-label">Origin/Notes:</span>
                      <span className="detail-value">{selectedClub.origin || <em className="missing">No notes</em>}</span>
                    </div>
                    <div className="detail-row">
                      <span className="detail-label">League/Division:</span>
                      <span className="detail-value">
                        {clubDivisionInfo ? (
                          `${clubDivisionInfo.division_name} (Position ${clubDivisionInfo.position})`
                        ) : (
                          <em className="missing">Not assigned</em>
                        )}
                      </span>
                    </div>
                    {selectedClub.parent_club_name && (
                      <div className="detail-row">
                        <span className="detail-label">Reserve Team Of:</span>
                        <span className="detail-value">{selectedClub.parent_club_name}</span>
                      </div>
                    )}

                    {!editingName && !editingLocation && !editingDivision && (
                      <>
                        <button onClick={handleEditName} className="edit-location-button-large">
                          Edit Name
                        </button>
                        <button onClick={handleEditLocation} className="edit-location-button-large" style={{marginTop: '10px'}}>
                          Edit Location Data
                        </button>
                        <button onClick={handleEditDivision} className="edit-location-button-large" style={{marginTop: '10px'}}>
                          Edit League/Division
                        </button>
                        {selectedClub.reserve_of_club_id === null && (
                          <button onClick={handleCopyToReserve} className="edit-location-button-large" style={{marginTop: '10px', background: '#27ae60'}}>
                            Copy Data to Reserve Team
                          </button>
                        )}
                      </>
                    )}
                  </div>

                  {editingName && (
                    <div className="club-location-edit-large">
                      <h3>Edit Club Name</h3>
                      <div className="form-group">
                        <label htmlFor="edit-name">Club Name:</label>
                        <input
                          id="edit-name"
                          type="text"
                          value={editName}
                          onChange={(e) => setEditName(e.target.value)}
                          placeholder="Enter club name"
                          className="location-input-large"
                        />
                        <p className="help-text">The official name of the club</p>
                        <button
                          onClick={handleCapitalizeName}
                          className="edit-location-button-large"
                          style={{marginTop: '10px'}}
                        >
                          Capitalize Name
                        </button>
                      </div>
                      <div className="form-buttons-large">
                        <button onClick={handleSaveName} className="save-button-large">
                          Save Name
                        </button>
                        <button onClick={handleCancelEditName} className="cancel-button-large">
                          Cancel
                        </button>
                      </div>
                    </div>
                  )}

                  {editingLocation && (
                    <div className="club-location-edit-large">
                      <h3>Edit Location Information</h3>
                      <div className="form-group">
                        <label htmlFor="edit-city">Area/City:</label>
                        <input
                          id="edit-city"
                          type="text"
                          value={editCity}
                          onChange={(e) => setEditCity(e.target.value)}
                          placeholder="e.g., Loxley, Heeley, Dore"
                          className="location-input-large"
                        />
                        <p className="help-text">The area or neighborhood where this club played</p>
                      </div>
                      <div className="form-group">
                        <label htmlFor="edit-region">Postcode District:</label>
                        <input
                          id="edit-region"
                          type="text"
                          value={editRegion}
                          onChange={(e) => setEditRegion(e.target.value)}
                          placeholder="e.g., S6, S2, S17"
                          className="location-input-large"
                        />
                        <p className="help-text">Sheffield postcode district (S1-S36)</p>
                      </div>
                      <div className="form-group">
                        <label htmlFor="edit-origin">Origin/Notes:</label>
                        <textarea
                          id="edit-origin"
                          value={editOrigin}
                          onChange={(e) => setEditOrigin(e.target.value)}
                          placeholder="e.g., played at East Bank, founded by..."
                          className="location-input-large"
                          rows={3}
                        />
                        <p className="help-text">Historical notes about the club's origin and location</p>
                      </div>
                      <div className="location-edit-buttons-large">
                        <button onClick={handleSaveLocation} className="save-button-large">
                          Save Changes
                        </button>
                        <button onClick={handleCancelEditLocation} className="cancel-button-large">
                          Cancel
                        </button>
                      </div>
                    </div>
                  )}

                  {editingDivision && (
                    <div className="club-location-edit-large">
                      <h3>Edit League/Division</h3>
                      <div className="form-group">
                        <label htmlFor="edit-division">Division:</label>
                        <select
                          id="edit-division"
                          value={editDivisionId}
                          onChange={(e) => setEditDivisionId(e.target.value)}
                          className="location-input-large"
                        >
                          <option value="">Select a division...</option>
                          {divisions.map(div => (
                            <option key={div.id} value={div.id}>
                              {div.name} {div.region ? `(${div.region})` : ''}
                            </option>
                          ))}
                        </select>
                        <p className="help-text">Select which division this club plays in</p>
                      </div>
                      <div className="form-group">
                        <label htmlFor="edit-position">Position in Division:</label>
                        <input
                          id="edit-position"
                          type="number"
                          min="1"
                          value={editPosition}
                          onChange={(e) => setEditPosition(parseInt(e.target.value) || 1)}
                          className="location-input-large"
                        />
                        <p className="help-text">Current standing position (1 = top of table)</p>
                      </div>
                      <div className="location-edit-buttons-large">
                        <button onClick={handleSaveDivision} className="save-button-large">
                          Save Changes
                        </button>
                        <button onClick={handleCancelEditDivision} className="cancel-button-large">
                          Cancel
                        </button>
                      </div>
                    </div>
                  )}

                  {creatingClub && (
                    <div className="club-location-edit-large">
                      <h3>Create New Club</h3>
                      <div className="form-group">
                        <label htmlFor="new-club-name">Club Name:*</label>
                        <input
                          id="new-club-name"
                          type="text"
                          value={newClubName}
                          onChange={(e) => setNewClubName(e.target.value)}
                          className="location-input-large"
                          placeholder="e.g., Sheffield FC"
                        />
                        <p className="help-text">Enter the club's full name (required)</p>
                      </div>
                      <div className="form-group">
                        <label htmlFor="new-founded-year">Founded Year:</label>
                        <input
                          id="new-founded-year"
                          type="number"
                          min="1857"
                          max="2026"
                          value={newClubFoundedYear}
                          onChange={(e) => setNewClubFoundedYear(parseInt(e.target.value) || 1867)}
                          className="location-input-large"
                        />
                        <p className="help-text">Year the club was founded</p>
                      </div>
                      <div className="form-group">
                        <label htmlFor="new-ground-name">Ground Name:</label>
                        <input
                          id="new-ground-name"
                          type="text"
                          value={newClubGroundName}
                          onChange={(e) => setNewClubGroundName(e.target.value)}
                          className="location-input-large"
                          placeholder="e.g., Bramall Lane"
                        />
                        <p className="help-text">Home ground or stadium name</p>
                      </div>
                      <div className="form-group">
                        <label htmlFor="new-club-city">City:</label>
                        <input
                          id="new-club-city"
                          type="text"
                          value={newClubCity}
                          onChange={(e) => setNewClubCity(e.target.value)}
                          className="location-input-large"
                          placeholder="e.g., Sheffield"
                        />
                        <p className="help-text">City or town where the club is based</p>
                      </div>
                      <div className="form-group">
                        <label htmlFor="new-club-region">Region:</label>
                        <input
                          id="new-club-region"
                          type="text"
                          value={newClubRegion}
                          onChange={(e) => setNewClubRegion(e.target.value)}
                          className="location-input-large"
                          placeholder="e.g., South Yorkshire"
                        />
                        <p className="help-text">County or region</p>
                      </div>
                      <div className="form-group">
                        <label htmlFor="new-club-origin">Origin/Notes:</label>
                        <textarea
                          id="new-club-origin"
                          value={newClubOrigin}
                          onChange={(e) => setNewClubOrigin(e.target.value)}
                          className="location-input-large"
                          rows={4}
                          placeholder="Historical notes, founding details, or other information..."
                        />
                        <p className="help-text">Historical information or notes about the club</p>
                      </div>
                      <div className="location-edit-buttons-large">
                        <button onClick={handleSaveNewClub} className="save-button-large">
                          Create Club
                        </button>
                        <button onClick={handleCancelCreateClub} className="cancel-button-large">
                          Cancel
                        </button>
                      </div>
                    </div>
                  )}

                  {error && <div className="error-message">{error}</div>}
                  {success && <div className="success-message">{success}</div>}
                </>
              ) : (
                <div className="no-club-selected">
                  <p>Select a club from the list to view and edit details</p>
                </div>
              )}
            </div>
          </>
        )}

        {/* PLAYER GENERATION TAB */}
        {/* CREATE PLAYERS TAB */}
        {activeTab === 'create-players' && (
          <div className="create-players-container">
            <h2>Create Players</h2>
            <p className="tab-description">Add players to the database with biographical information. Players can be assigned to clubs in the "Assign to Clubs" tab.</p>

            <div className="create-players-content">
              {/* Year Setting */}
              <div className="form-group">
                <label htmlFor="year-input">Reference Year (for age calculations):</label>
                <input
                  id="year-input"
                  type="number"
                  min="1857"
                  max="1900"
                  value={year}
                  onChange={(e) => setYear(parseInt(e.target.value))}
                  className="year-input"
                />
              </div>

              {/* Default Parish Selection */}
              <div className="form-group">
                <div style={{display: 'flex', alignItems: 'center', gap: '10px', marginBottom: '10px'}}>
                  <input
                    type="checkbox"
                    id="use-default-parish"
                    checked={useDefaultParish}
                    onChange={(e) => setUseDefaultParish(e.target.checked)}
                    style={{width: '18px', height: '18px', cursor: 'pointer'}}
                  />
                  <label htmlFor="use-default-parish" style={{cursor: 'pointer', fontWeight: 'bold'}}>
                    Use Default Parish (applies to all pasted players)
                  </label>
                </div>
                {useDefaultParish && (
                  <select
                    id="default-parish"
                    value={defaultParish}
                    onChange={(e) => setDefaultParish(e.target.value)}
                    className="parish-filter-select"
                  >
                    <option value="">None (leave blank)</option>
                  <option value="All Saints">All Saints</option>
                  <option value="Attercliffe">Attercliffe</option>
                  <option value="Attercliffe Christchurch">Attercliffe Christchurch</option>
                  <option value="Attercliffe cum Darnall">Attercliffe cum Darnall</option>
                  <option value="Brewery Field">Brewery Field</option>
                  <option value="Brightside">Brightside</option>
                  <option value="Brightside Bierlow">Brightside Bierlow</option>
                  <option value="Broomhall">Broomhall</option>
                  <option value="Carver Street">Carver Street</option>
                  <option value="Cathedral">Cathedral</option>
                  <option value="Christchurch">Christchurch</option>
                  <option value="Crookes">Crookes</option>
                  <option value="Crookes St Thomas">Crookes St Thomas</option>
                  <option value="Dore">Dore</option>
                  <option value="Ecclesall">Ecclesall</option>
                  <option value="Ecclesall Bierlow">Ecclesall Bierlow</option>
                  <option value="Ecclesfield">Ecclesfield</option>
                  <option value="Eldon">Eldon</option>
                  <option value="Eldon St Jude">Eldon St Jude</option>
                  <option value="Eldon Street">Eldon Street</option>
                  <option value="Fulwood">Fulwood</option>
                  <option value="Fulwood Christchurch">Fulwood Christchurch</option>
                  <option value="Handsworth">Handsworth</option>
                  <option value="Heeley">Heeley</option>
                  <option value="Holy Trinity">Holy Trinity</option>
                  <option value="Intake">Intake</option>
                  <option value="Manor">Manor</option>
                  <option value="Neepsend">Neepsend</option>
                  <option value="Nether Hallam">Nether Hallam</option>
                  <option value="Norton">Norton</option>
                  <option value="Park">Park</option>
                  <option value="Park Hill">Park Hill</option>
                  <option value="Pitsmoor">Pitsmoor</option>
                  <option value="Sharrow">Sharrow</option>
                  <option value="Sharrow St Andrew">Sharrow St Andrew</option>
                  <option value="Sheffield">Sheffield</option>
                  <option value="Sheffield Cathedral">Sheffield Cathedral</option>
                  <option value="St Andrew">St Andrew</option>
                  <option value="St Bartholomew">St Bartholomew</option>
                  <option value="St George">St George</option>
                  <option value="St James">St James</option>
                  <option value="St John">St John</option>
                  <option value="St Jude">St Jude</option>
                  <option value="St Mark">St Mark</option>
                  <option value="St Mary">St Mary</option>
                  <option value="St Matthew">St Matthew</option>
                  <option value="St Paul">St Paul</option>
                  <option value="St Peter">St Peter</option>
                  <option value="St Philip">St Philip</option>
                  <option value="St Silas">St Silas</option>
                  <option value="St Stephen">St Stephen</option>
                  <option value="St Thomas">St Thomas</option>
                  <option value="The Porter">The Porter</option>
                  <option value="The Wicker">The Wicker</option>
                  <option value="Tinsley">Tinsley</option>
                  <option value="Trinity">Trinity</option>
                  <option value="Upper Hallam">Upper Hallam</option>
                  <option value="Walkley">Walkley</option>
                  <option value="Walkley St Mary">Walkley St Mary</option>
                  <option value="Wicker">Wicker</option>
                  <option value="Wincobank">Wincobank</option>
                  <option value="Woodhouse">Woodhouse</option>
                  <option value="Wortley">Wortley</option>
                </select>
                )}
              </div>

              {/* Quick Action Button at Top */}
              {manualPlayers.length > 0 && (
                <div className="action-buttons-gen" style={{marginTop: '15px', marginBottom: '15px'}}>
                  <button
                    onClick={handleBulkCreatePlayersWithBackup}
                    disabled={loading || manualPlayers.length === 0}
                    className="save-players-button"
                  >
                    {loading ? (progressMessage || 'Creating...') : `Create ${manualPlayers.length} Player(s)`}
                  </button>
                  <button
                    onClick={() => setManualPlayers([])}
                    className="clear-players-button"
                  >
                    Clear All
                  </button>
                </div>
              )}

              {/* Census Data Paste Area */}
              <div className="form-group">
                <label htmlFor="census-paste">Paste Census Data:</label>
                <textarea
                  id="census-paste"
                  value={playerNamesText}
                  onChange={(e) => setPlayerNamesText(e.target.value)}
                  className="census-paste-input"
                  rows={10}
                  placeholder="Paste census records here...&#10;&#10;Supports formats:&#10;- Name: Albert Law&#10;  Age: 29&#10;  Estimated Birth Year: 1842&#10;  Where born: Sheffield, Yorkshire, England&#10;  Ecclesiastical parish: St Philip&#10;  Registration District: Ecclesall Bierlow&#10;  Sub-registration district: Nether Hallam&#10;  Household Members:&#10;&#10;- Tabular: Name    Age    Birth Year    Birth Place"
                />
                <button
                  onClick={() => handlePasteCensusData(playerNamesText)}
                  disabled={!playerNamesText.trim()}
                  className="parse-button"
                >
                  Parse & Add to Table
                </button>
              </div>

              {/* CSV File Import */}
              <div className="form-group" style={{ marginTop: '15px', padding: '15px', background: '#f0fff0', borderRadius: '8px', border: '2px dashed #4CAF50' }}>
                <label style={{ fontWeight: 'bold', color: '#2c3e50', marginBottom: '8px', display: 'block' }}>
                  📁 Import Census CSV Files
                </label>
                <p style={{ fontSize: '13px', color: '#666', marginBottom: '10px' }}>
                  Import historical census data from CSV files in the "sheffield census" directory.
                  Files should contain columns: NAME, AGE, BIRTH YEAR, ECCLESIASTICAL PARISH, etc.
                </p>
                <div style={{ display: 'flex', gap: '10px', marginBottom: '10px' }}>
                  <CsvImportButton
                    onFileLoaded={handleCsvImport}
                    accept=".csv,.txt,.tsv"
                    className="import-csv-button"
                    disabled={loading}
                  />
                  <button
                    onClick={handleBulkImportAllCensusFiles}
                    disabled={loading}
                    style={{
                      padding: '10px 20px',
                      background: '#FF9800',
                      color: 'white',
                      border: 'none',
                      borderRadius: '6px',
                      fontWeight: 'bold',
                      cursor: loading ? 'not-allowed' : 'pointer',
                      opacity: loading ? 0.6 : 1
                    }}
                  >
                    🚀 Bulk Import ALL 73 Census Files
                  </button>
                </div>
                <p style={{ fontSize: '11px', color: '#999', marginTop: '8px' }}>
                  Supported formats: Census CSV (tab-separated), regular CSV, TSV • Bulk import loads all files directly into database
                </p>
              </div>

              {/* Manual Player Table */}
              <div className="manual-player-section">
                <div className="manual-player-header">
                  <h3>Players to Create ({manualPlayers.length})</h3>
                  <button onClick={addManualPlayerRow} className="add-row-btn">
                    + Add Row
                  </button>
                </div>

                {manualPlayers.length === 0 ? (
                  <div className="empty-table-message">
                    No players added yet. Paste census data above or click "+ Add Row" to add manually.
                  </div>
                ) : (
                  <div className="manual-player-table-container">
                    <table className="manual-player-table">
                      <thead>
                        <tr>
                          <th>Name</th>
                          <th>Age</th>
                          <th>Birth Year</th>
                          <th>Relation</th>
                          <th>Gender</th>
                          <th>Where Born</th>
                          <th>Civil Parish</th>
                          <th>Eccl. Parish</th>
                          <th>Town</th>
                          <th>County</th>
                          <th>Country</th>
                          <th>Reg. District</th>
                          <th>Sub-Reg. District</th>
                          <th>ED</th>
                          <th>HH Sched.</th>
                          <th>Piece</th>
                          <th>Folio</th>
                          <th>Page</th>
                          <th></th>
                        </tr>
                      </thead>
                      <tbody>
                        {manualPlayers.map(player => (
                          <tr key={player.id}>
                            <td>
                              <input
                                type="text"
                                value={player.name}
                                onChange={(e) => updateManualPlayer(player.id, 'name', e.target.value)}
                                placeholder="Player Name"
                                className="table-input"
                              />
                            </td>
                            <td>
                              <input
                                type="number"
                                value={player.age || ''}
                                onChange={(e) => updateManualPlayer(player.id, 'age', parseInt(e.target.value))}
                                className="table-input year-input-small"
                                placeholder="34"
                              />
                            </td>
                            <td>
                              <input
                                type="number"
                                value={player.birthYear || ''}
                                onChange={(e) => updateManualPlayer(player.id, 'birthYear', parseInt(e.target.value))}
                                className="table-input year-input-small"
                                min="1800"
                                max="1900"
                              />
                            </td>
                            <td>
                              <input
                                type="text"
                                value={player.relation || ''}
                                onChange={(e) => updateManualPlayer(player.id, 'relation', e.target.value)}
                                placeholder="Head"
                                className="table-input"
                              />
                            </td>
                            <td>
                              <input
                                type="text"
                                value={player.gender || ''}
                                onChange={(e) => updateManualPlayer(player.id, 'gender', e.target.value)}
                                placeholder="Male"
                                className="table-input"
                              />
                            </td>
                            <td>
                              <input
                                type="text"
                                value={player.whereBorn || ''}
                                onChange={(e) => updateManualPlayer(player.id, 'whereBorn', e.target.value)}
                                placeholder="Sheffield, Yorkshire, England"
                                className="table-input"
                              />
                            </td>
                            <td>
                              <input
                                type="text"
                                value={player.civilParish || ''}
                                onChange={(e) => updateManualPlayer(player.id, 'civilParish', e.target.value)}
                                placeholder="Nether Hallam"
                                className="table-input"
                              />
                            </td>
                            <td>
                              <input
                                type="text"
                                value={player.ecclesiasticalParish || ''}
                                onChange={(e) => updateManualPlayer(player.id, 'ecclesiasticalParish', e.target.value)}
                                placeholder="St Philip"
                                className="table-input"
                              />
                            </td>
                            <td>
                              <input
                                type="text"
                                value={player.birthTown || ''}
                                onChange={(e) => updateManualPlayer(player.id, 'birthTown', e.target.value)}
                                placeholder="Sheffield"
                                className="table-input"
                              />
                            </td>
                            <td>
                              <input
                                type="text"
                                value={player.birthCounty || ''}
                                onChange={(e) => updateManualPlayer(player.id, 'birthCounty', e.target.value)}
                                placeholder="Yorkshire"
                                className="table-input"
                              />
                            </td>
                            <td>
                              <input
                                type="text"
                                value={player.birthCountry || ''}
                                onChange={(e) => updateManualPlayer(player.id, 'birthCountry', e.target.value)}
                                placeholder="England"
                                className="table-input"
                              />
                            </td>
                            <td>
                              <input
                                type="text"
                                value={player.registrationDistrict || ''}
                                onChange={(e) => updateManualPlayer(player.id, 'registrationDistrict', e.target.value)}
                                placeholder="Ecclesall Bierlow"
                                className="table-input"
                              />
                            </td>
                            <td>
                              <input
                                type="text"
                                value={player.subRegistrationDistrict || ''}
                                onChange={(e) => updateManualPlayer(player.id, 'subRegistrationDistrict', e.target.value)}
                                placeholder="Nether Hallam"
                                className="table-input"
                              />
                            </td>
                            <td>
                              <input
                                type="text"
                                value={player.edInstitution || ''}
                                onChange={(e) => updateManualPlayer(player.id, 'edInstitution', e.target.value)}
                                className="table-input"
                              />
                            </td>
                            <td>
                              <input
                                type="text"
                                value={player.householdScheduleNumber || ''}
                                onChange={(e) => updateManualPlayer(player.id, 'householdScheduleNumber', e.target.value)}
                                className="table-input"
                              />
                            </td>
                            <td>
                              <input
                                type="text"
                                value={player.piece || ''}
                                onChange={(e) => updateManualPlayer(player.id, 'piece', e.target.value)}
                                className="table-input"
                              />
                            </td>
                            <td>
                              <input
                                type="text"
                                value={player.folio || ''}
                                onChange={(e) => updateManualPlayer(player.id, 'folio', e.target.value)}
                                className="table-input"
                              />
                            </td>
                            <td>
                              <input
                                type="text"
                                value={player.pageNumber || ''}
                                onChange={(e) => updateManualPlayer(player.id, 'pageNumber', e.target.value)}
                                className="table-input"
                              />
                            </td>
                            <td>
                              <button
                                onClick={() => removeManualPlayer(player.id)}
                                className="remove-row-btn"
                                title="Remove"
                              >
                                ×
                              </button>
                            </td>
                          </tr>
                        ))}
                      </tbody>
                    </table>
                  </div>
                )}
              </div>

              {/* Action Buttons */}
              <div className="action-buttons-gen">
                <button
                  onClick={handleCreatePlayers}
                  disabled={loading || manualPlayers.length === 0}
                  className="save-players-button"
                >
                  {loading ? 'Creating...' : `Create ${manualPlayers.length} Player(s)`}
                </button>
                {manualPlayers.length > 0 && (
                  <button
                    onClick={() => setManualPlayers([])}
                    className="clear-players-button"
                  >
                    Clear All
                  </button>
                )}
              </div>

              {error && <div className="error-message">{error}</div>}
              {success && <div className="success-message">{success}</div>}
            </div>
          </div>
        )}

        {/* ASSIGN STATS TAB */}
        {activeTab === 'assign-stats' && (
          <div className="assign-stats-container">
            <div className="assign-stats-layout">
              {/* Left sidebar: Player list */}
              <div className="players-sidebar">
                <div className="sidebar-header">
                  <h2>Players Awaiting Stats</h2>
                  <button
                    className="refresh-players-btn"
                    onClick={loadPlayersWithoutStats}
                    title="Refresh player list"
                  >
                    ↻ Refresh ({playersWithoutStats.length})
                  </button>
                </div>

                {error && <div className="error-message">{error}</div>}

                {playersWithoutStats.length === 0 ? (
                  <div className="placeholder-content">
                    <p>No players awaiting stats.</p>
                    <p style={{fontSize: '0.9em', color: '#999', marginTop: '10px'}}>
                      Create players in the "Create Players" tab.
                    </p>
                  </div>
                ) : (
                  <div className="players-list">
                    {playersWithoutStats.map((player: any) => (
                      <div
                        key={player.id}
                        className={`player-item ${selectedPlayerForStats?.id === player.id ? 'selected' : ''}`}
                        onClick={() => setSelectedPlayerForStats(player)}
                      >
                        <div className="player-item-name">{player.name}</div>
                        <div className="player-item-details">
                          {player.birth_year} • {player.nationality || 'Unknown'}
                        </div>
                        <div className="player-item-location">
                          {player.birth_town && player.birth_county
                            ? `${player.birth_town}, ${player.birth_county}`
                            : player.where_born || 'Location unknown'}
                        </div>
                      </div>
                    ))}
                  </div>
                )}
              </div>

              {/* Right side: Player detail and stats editor */}
              <div className="player-stats-detail">
                {!selectedPlayerForStats ? (
                  <div className="placeholder-content">
                    <h3>No Player Selected</h3>
                    <p>Select a player from the list to assign their stats.</p>
                  </div>
                ) : (
                  <div className="stats-editor-panel">
                    <div className="player-detail-header">
                      <div>
                        <h2>{selectedPlayerForStats.name}</h2>
                        <div className="player-meta">
                          <span>Born: {selectedPlayerForStats.birth_year}</span>
                          {selectedPlayerForStats.nationality && (
                            <span> • {selectedPlayerForStats.nationality}</span>
                          )}
                        </div>
                        {selectedPlayerForStats.where_born && (
                          <div className="player-origin">
                            <strong>Origin:</strong> {selectedPlayerForStats.where_born}
                          </div>
                        )}
                        {(selectedPlayerForStats.ecclesiastical_parish ||
                          selectedPlayerForStats.registration_district) && (
                          <div className="player-census-data">
                            {selectedPlayerForStats.ecclesiastical_parish && (
                              <div><strong>Parish:</strong> {selectedPlayerForStats.ecclesiastical_parish}</div>
                            )}
                            {selectedPlayerForStats.registration_district && (
                              <div><strong>Registration District:</strong> {selectedPlayerForStats.registration_district}</div>
                            )}
                            {selectedPlayerForStats.sub_registration_district && (
                              <div><strong>Sub-district:</strong> {selectedPlayerForStats.sub_registration_district}</div>
                            )}
                          </div>
                        )}
                      </div>
                    </div>

                    <div className="stats-assignment-placeholder">
                      <h3>Stats Assignment Editor</h3>
                      <p>This section will contain the full stats editor with:</p>
                      <ul>
                        <li>Position selection</li>
                        <li>Physical attributes (Pace, Strength, Stamina, etc.)</li>
                        <li>Technical skills (Passing, Dribbling, Tackling, etc.)</li>
                        <li>Mental attributes (Leadership, Determination, etc.)</li>
                        <li>Preset buttons (Elite, Good, Average, Poor)</li>
                        <li>Club assignment dropdown</li>
                        <li>Save and Cancel buttons</li>
                      </ul>
                      <button
                        className="placeholder-action-btn"
                        onClick={() => console.log('TODO: Open stats editor for', selectedPlayerForStats.name)}
                      >
                        Open Stats Editor (Coming Soon)
                      </button>
                    </div>
                  </div>
                )}
              </div>
            </div>
          </div>
        )}

        {/* ASSIGN TO CLUBS TAB */}
        {activeTab === 'assign-clubs' && (
          <div className="assign-clubs-container">
            <h2>Assign Players to Clubs</h2>
            <p className="tab-description">Link players to one or more clubs (supports multi-club assignment)</p>

            <div className="assign-clubs-layout">
              {/* Left Panel: Club Selection */}
              <div className="club-selection-panel-gen">
                <h3>Select Clubs</h3>

                {/* Postcode Filter */}
                <div className="form-group">
                  <label htmlFor="postcode-filter-gen">Filter by Postcode:</label>
                  <input
                    id="postcode-filter-gen"
                    type="text"
                    value={playerGenPostcode}
                    onChange={(e) => setPlayerGenPostcode(e.target.value.toUpperCase())}
                    placeholder="e.g., S4"
                    className="filter-input"
                  />
                </div>

                {/* Postcode Quick Buttons */}
                <div className="postcode-buttons">
                  {uniquePostcodes.filter(pc => pc).map(pc => (
                    <button
                      key={pc}
                      className={`postcode-btn ${playerGenPostcode === pc ? 'active' : ''}`}
                      onClick={() => setPlayerGenPostcode(pc)}
                    >
                      {pc}
                    </button>
                  ))}
                </div>

                {/* Club List with Checkboxes */}
                <div className="club-checkbox-list">
                  {clubs
                    .filter(club => !playerGenPostcode ||
                      (club.where_from && club.where_from.includes(playerGenPostcode)))
                    .map(club => {
                      const isSelected = selectedClubIds.includes(club.id);
                      return (
                        <div
                          key={club.id}
                          className={`club-checkbox-item ${isSelected ? 'selected' : ''}`}
                          onClick={() => toggleClubSelection(club.id)}
                        >
                          <input
                            type="checkbox"
                            checked={isSelected}
                            onChange={() => {}}
                          />
                          <div className="club-checkbox-info">
                            <div className="club-checkbox-name">{club.name}</div>
                            <div className="club-checkbox-details">
                              {club.ground_name} • {club.region || club.city}
                            </div>
                          </div>
                        </div>
                      );
                    })}
                </div>

                <div className="selection-summary">
                  {selectedClubIds.length} club(s) selected
                </div>
              </div>

              {/* Right Panel: Player Selection and Assignment */}
              <div className="player-assignment-panel">
                <h3>Assign Players</h3>

                <div className="placeholder-content">
                  <p>This section will allow you to:</p>
                  <ul>
                    <li>Search for existing players in the database</li>
                    <li>Filter players by name, nationality, birth year</li>
                    <li>Select multiple players</li>
                    <li>Assign selected players to the selected clubs</li>
                    <li>View current club assignments</li>
                    <li>Remove players from clubs</li>
                  </ul>
                </div>
              </div>
            </div>
          </div>
        )}

        {/* PARISHES TAB */}
        {activeTab === 'parishes' && (
          <div className="parishes-container">
            <h2>Parish Management</h2>
            <p className="tab-description">Manage ecclesiastical parishes and assign postcodes</p>

            <button
              onClick={async () => {
                try {
                  const result = await invoke('db_check_club_assignments');
                  const stats = result as any;
                  alert(`Club Assignment Statistics:

Total Players: ${stats.total_players}
Assigned to Clubs: ${stats.assigned_players}
Unassigned: ${stats.unassigned_players}

Top 10 Clubs:
${stats.top_clubs.map((club: any) => `${club.club_name}: ${club.player_count} players`).join('\n')}`);
                } catch (err) {
                  setError(`Error checking assignments: ${err}`);
                }
              }}
              style={{ marginBottom: '20px' }}
            >
              Check Club Assignments
            </button>

            {error && <div className="error-message">{error}</div>}
            {success && <div className="success-message">{success}</div>}

            <div className="parishes-content" style={{marginTop: '20px'}}>
              <table className="player-database-table">
                <thead>
                  <tr>
                    <th>Parish Name</th>
                    <th>Assigned Postcode</th>
                    <th>Additional Postcode</th>
                    <th>Unassigned Players</th>
                    <th>Club Count</th>
                    <th>Actions</th>
                  </tr>
                </thead>
                <tbody>
                  {(() => {
                    console.log('[Parish Render] allPlayersForParishes.length:', allPlayersForParishes.length);

                    // Sample first 5 players to see their structure
                    if (allPlayersForParishes.length > 0) {
                      console.log('[Parish Render] First player sample:', allPlayersForParishes[0]);
                      console.log('[Parish Render] Player has club_id?', 'club_id' in allPlayersForParishes[0]);
                      console.log('[Parish Render] Player club_id value:', allPlayersForParishes[0].club_id);
                    }

                    // Get unique parishes and count ALL players
                    const parishMap = new Map<string, number>();
                    let playersWithoutParish = 0;

                    allPlayersForParishes.forEach(player => {
                      const parish = player.ecclesiastical_parish?.trim();
                      if (parish && parish !== '') {
                        parishMap.set(parish, (parishMap.get(parish) || 0) + 1);
                      } else {
                        playersWithoutParish++;
                      }
                    });

                    console.log('[Parish Render] Players without parish:', playersWithoutParish);
                    console.log('[Parish Render] Parish map size:', parishMap.size);

                    // Include "No Parish" entry for players without an ecclesiastical parish
                    if (playersWithoutParish > 0) {
                      parishMap.set('(No Parish)', playersWithoutParish);
                    }

                    // Sort parishes alphabetically
                    const sortedParishes = Array.from(parishMap.entries()).sort((a, b) =>
                      a[0].localeCompare(b[0])
                    );

                    console.log('[Parish Render] Sorted parishes count:', sortedParishes.length);

                    return sortedParishes.map(([parish, playerCount]) => {
                      // Find the parish postcode from sheffield_parishes table
                      const parishData = parishesFromDB.find(p => p.name === parish);
                      const parishPostcodeFromDB = parishData?.postcode || '';

                      // Count how many players in this parish are UNASSIGNED (no club)
                      const unassignedPlayers = allPlayersForParishes.filter(p =>
                        p.ecclesiastical_parish === parish &&
                        (p.club_id === 'UNASSIGNED' || !p.club_id || p.club_id.trim() === '')
                      ).length;

                      // Count how many players in this parish don't have postcodes (for the Assign Postcode button)
                      const playersWithPostcode = allPlayersForParishes.filter(p =>
                        p.ecclesiastical_parish === parish && p.postcode && p.postcode.trim() !== ''
                      ).length;
                      const playersWithoutPostcode = playerCount - playersWithPostcode;

                      // Count clubs matching the parish postcode area
                      const clubsWithPostcode = parishPostcodeFromDB ? clubs.filter(club => {
                        const clubRegion = club.region || '';
                        // Match on postcode area only (e.g., S6 matches S6, not S6 6FL)
                        return clubRegion.startsWith(parishPostcodeFromDB);
                      }) : [];

                      return (
                        <tr key={parish}>
                          <td style={{fontWeight: 'bold'}}>{parish}</td>
                          <td style={{color: parishPostcodeFromDB ? '#28a745' : '#999', fontWeight: 'bold'}}>
                            {parishPostcodeFromDB || 'Not in DB'}
                          </td>
                          <td>
                            <input
                              type="text"
                              placeholder="e.g., S10"
                              value={parishData?.additional_postcode || ''}
                              onChange={async (e) => {
                                const newValue = e.target.value.trim();
                                try {
                                  await invoke('db_update_parish_additional_postcode', {
                                    parishName: parish,
                                    additionalPostcode: newValue || null
                                  });
                                  await loadParishesFromDB();
                                } catch (err) {
                                  setError(`Failed to update additional postcode: ${err}`);
                                }
                              }}
                              style={{
                                width: '80px',
                                padding: '4px 8px',
                                border: '1px solid #ccc',
                                borderRadius: '4px'
                              }}
                            />
                          </td>
                          <td>{unassignedPlayers} / {playerCount}</td>
                          <td>{clubsWithPostcode.length} clubs</td>
                          <td>
                          <button
                            onClick={async () => {
                              if (!parishPostcodeFromDB || !parishPostcodeFromDB.trim()) {
                                setError(`Parish "${parish}" has no postcode in database`);
                                return;
                              }

                              if (playersWithoutPostcode === 0) {
                                setSuccess(`All players in ${parish} already have postcodes`);
                                return;
                              }

                              try {
                                setError('');

                                const affected = await invoke<number>('db_assign_postcode_to_parish', {
                                  parish: parish,
                                  postcode: parishPostcodeFromDB.trim()
                                });
                                setSuccess(`Assigned ${parishPostcodeFromDB} to ${affected} players in ${parish}`);

                                await loadAllPlayersForParishes();
                                setTimeout(() => setSuccess(''), 8000);
                              } catch (err) {
                                setError(`Failed to assign postcode: ${err}`);
                              }
                            }}
                            disabled={!parishPostcodeFromDB || playersWithoutPostcode === 0}
                            className="assign-postcode-btn"
                            style={{padding: '8px 16px'}}
                          >
                            Assign Postcode ({playersWithoutPostcode} need it)
                          </button>
                          {parishData?.additional_postcode && (
                            <button
                              onClick={async () => {
                                const additionalPostcode = parishData.additional_postcode.trim();
                                if (!additionalPostcode) {
                                  setError(`No additional postcode set for ${parish}`);
                                  return;
                                }

                                if (playersWithoutPostcode === 0) {
                                  setSuccess(`All players in ${parish} already have postcodes`);
                                  return;
                                }

                                try {
                                  setError('');
                                  const affected = await invoke<number>('db_assign_postcode_to_parish', {
                                    parish: parish,
                                    postcode: additionalPostcode
                                  });
                                  setSuccess(`Assigned ${additionalPostcode} to ${affected} players in ${parish}`);
                                  await loadAllPlayersForParishes();
                                  setTimeout(() => setSuccess(''), 8000);
                                } catch (err) {
                                  setError(`Failed to assign additional postcode: ${err}`);
                                }
                              }}
                              disabled={playersWithoutPostcode === 0}
                              className="assign-postcode-btn"
                              style={{padding: '8px 16px', marginLeft: '8px', backgroundColor: '#6c757d'}}
                            >
                              Assign Additional ({parishData.additional_postcode})
                            </button>
                          )}
                          <button
                            onClick={async () => {
                              if (!parishPostcodeFromDB || !parishPostcodeFromDB.trim()) {
                                setError(`Parish "${parish}" has no postcode in database`);
                                return;
                              }

                              if (clubsWithPostcode.length === 0) {
                                setError(`No clubs found with postcode area ${parishPostcodeFromDB}`);
                                return;
                              }

                              // Show confirmation
                              const message = `This will assign unassigned players from ${parish} to ${clubsWithPostcode.length} clubs with postcode area ${parishPostcodeFromDB}.\n\nContinue?`;
                              if (!confirm(message)) return;

                              try {
                                setError('');
                                setLoading(true);
                                setSuccess('Starting assignment process...');

                                const result = await invoke<{
                                  parish: string;
                                  assigned_count: number;
                                  skipped_no_postcode: number;
                                  skipped_no_clubs: number;
                                  assignments: Array<{
                                    assignment_number: number;
                                    player_name: string;
                                    postcode: string;
                                    postcode_area: string;
                                    club_id: string;
                                    club_name: string;
                                  }>;
                                }>('db_assign_parish_players_to_clubs', {
                                  parish: parish,
                                });

                                if (result.assigned_count === 0) {
                                  let message = `No players assigned from ${parish}. `;
                                  if (result.skipped_no_postcode > 0) {
                                    message += `${result.skipped_no_postcode} players have no postcode. `;
                                  }
                                  if (result.skipped_no_clubs > 0) {
                                    message += `${result.skipped_no_clubs} players' postcodes don't match any clubs.`;
                                  }
                                  setSuccess(message);
                                } else {
                                  // Show progress for each assignment with animated updates
                                  for (let i = 0; i < result.assignments.length; i++) {
                                    const assignment = result.assignments[i];
                                    const progress = `[${i + 1}/${result.assigned_count}] ${assignment.player_name} → ${assignment.club_name} (${assignment.postcode_area})`;
                                    setSuccess(progress);
                                    // Small delay for visual feedback (10ms per assignment, max 500ms total)
                                    if (i < result.assignments.length - 1) {
                                      await new Promise(resolve => setTimeout(resolve, Math.min(500 / result.assignments.length, 50)));
                                    }
                                  }

                                  // Create final summary
                                  const clubAssignments: {[clubName: string]: string[]} = {};
                                  result.assignments.forEach(a => {
                                    if (!clubAssignments[a.club_name]) {
                                      clubAssignments[a.club_name] = [];
                                    }
                                    clubAssignments[a.club_name].push(a.player_name);
                                  });

                                  const clubCount = Object.keys(clubAssignments).length;
                                  const summary = Object.entries(clubAssignments)
                                    .map(([club, players]) => `  • ${club}: ${players.length} player${players.length > 1 ? 's' : ''} (${players.join(', ')})`)
                                    .join('\n');

                                  setSuccess(`✓ Successfully assigned ${result.assigned_count} player(s) from ${parish} to ${clubCount} club${clubCount > 1 ? 's' : ''}:\n\n${summary}`);
                                  await loadAllPlayersForParishes(); // This will update both parish table and club capacity table
                                }
                                setTimeout(() => setSuccess(''), 10000);
                              } catch (err) {
                                setError(`Failed to assign players: ${err}`);
                              } finally {
                                setLoading(false);
                              }
                            }}
                            disabled={!parishPostcodeFromDB?.trim() || clubsWithPostcode.length === 0 || loading}
                            className="save-players-button"
                            style={{padding: '8px 16px', marginLeft: '10px', opacity: loading ? 0.6 : 1}}
                          >
                            {loading ? 'Assigning players...' : 'Assign to Clubs'}
                          </button>
                        </td>
                      </tr>
                    );
                    });
                  })()}
                </tbody>
              </table>

              {allPlayers.length === 0 && (
                <p style={{textAlign: 'center', padding: '20px', color: '#666'}}>
                  No players loaded. Switch to Player Database tab to load players.
                </p>
              )}
            </div>
          </div>
        )}

        {/* CLUB CAPACITY TAB */}
        {activeTab === 'club-capacity' && (
          <div className="parishes-management-container">
            <div className="parishes-table-container" style={{maxHeight: '600px', overflowY: 'auto'}}>
              <h2>Club Capacity Overview</h2>
              <p style={{color: '#666', marginBottom: '20px'}}>
                Shows which clubs have reached 15 players and which still have capacity
              </p>

              <table className="parishes-table">
                <thead>
                  <tr>
                    <th>Club Name</th>
                    <th>Postcode</th>
                    <th>Additional Postcode</th>
                    <th>Player Count</th>
                    <th>Status</th>
                  </tr>
                </thead>
                <tbody>
                  {(() => {
                    // Get player count for each club - use allPlayersForParishes to get ALL players, not just paginated
                    const clubPlayerCounts = new Map<string, number>();
                    allPlayersForParishes.forEach(player => {
                      const clubId = player.club_id;
                      if (clubId && clubId !== 'UNASSIGNED') {
                        clubPlayerCounts.set(clubId, (clubPlayerCounts.get(clubId) || 0) + 1);
                      }
                    });

                    // Sort clubs: full clubs first, then by player count descending
                    const sortedClubs = [...clubs].sort((a, b) => {
                      const countA = clubPlayerCounts.get(a.id) || 0;
                      const countB = clubPlayerCounts.get(b.id) || 0;
                      const fullA = countA >= 15 ? 1 : 0;
                      const fullB = countB >= 15 ? 1 : 0;

                      // Full clubs first
                      if (fullB !== fullA) return fullB - fullA;

                      // Then by count descending
                      return countB - countA;
                    });

                    return sortedClubs.map(club => {
                      const playerCount = clubPlayerCounts.get(club.id) || 0;
                      const isFull = playerCount >= 15;
                      const postcode = club.region || 'N/A';

                      return (
                        <tr key={club.id} style={{backgroundColor: isFull ? '#e8f5e9' : '#fff3e0'}}>
                          <td style={{fontWeight: 'bold', color: '#000'}}>{club.name}</td>
                          <td style={{color: '#000'}}>{postcode}</td>
                          <td>
                            <input
                              type="text"
                              value={club.additional_postcode || ''}
                              onChange={async (e) => {
                                const newPostcode = e.target.value.toUpperCase();
                                try {
                                  await invoke('db_update_club_additional_postcode', {
                                    clubId: club.id,
                                    additionalPostcode: newPostcode || null
                                  });
                                  // Update local state
                                  setClubs(clubs.map(c =>
                                    c.id === club.id ? {...c, additional_postcode: newPostcode || undefined} : c
                                  ));
                                } catch (err) {
                                  setError(`Failed to update postcode: ${err}`);
                                }
                              }}
                              placeholder="e.g., S4"
                              style={{
                                width: '80px',
                                padding: '4px',
                                border: '1px solid #ccc',
                                borderRadius: '3px'
                              }}
                            />
                          </td>
                          <td style={{fontWeight: 'bold', color: isFull ? '#2e7d32' : '#f57c00'}}>
                            {playerCount}/15 players
                          </td>
                          <td>
                            {isFull ? (
                              <span style={{color: '#2e7d32', fontWeight: 'bold'}}>✓ FULL</span>
                            ) : (
                              <span style={{color: '#f57c00', fontWeight: 'bold'}}>
                                {15 - playerCount} slots remaining
                              </span>
                            )}
                          </td>
                        </tr>
                      );
                    });
                  })()}
                </tbody>
              </table>

              {clubs.length === 0 && (
                <p style={{textAlign: 'center', padding: '20px', color: '#666'}}>
                  No clubs loaded. Please load clubs first.
                </p>
              )}
            </div>
          </div>
        )}

        {/* SQUAD MANAGEMENT TAB */}
        {activeTab === 'squads' && (
          <div className="squad-management-container">
            <div className="squad-clubs-panel">
              <h2>Select Club</h2>
              <div className="club-list">
                {squadClubs.map((club) => (
                  <div
                    key={club.id}
                    className={`club-item ${selectedSquadClubId === club.id ? 'selected' : ''}`}
                    onClick={() => handleSelectSquadClub(club.id)}
                  >
                    <div className="club-name">{club.name}</div>
                    <div className="club-founded">Founded: {club.founded_year}</div>
                    {club.division_name && (
                      <div className="club-division">{club.division_name}</div>
                    )}
                  </div>
                ))}
              </div>
            </div>

            <div className="squad-players-panel">
              <h2>
                {squadClubs.find((c) => c.id === selectedSquadClubId)?.name || 'Select a club'} - Squad
              </h2>

              <div className="squad-actions">
                <button
                  className="batch-stats-button"
                  onClick={handleGenerateStatsForThisClub}
                  disabled={!selectedSquadClubId || squadPlayers.length === 0 || loading}
                >
                  {loading ? 'Generating...' : 'Generate Stats for This Club'}
                </button>
                <div style={{ display: 'flex', gap: '10px', alignItems: 'center' }}>
                  <label htmlFor="start-club-input" style={{ fontSize: '14px', fontWeight: 'bold' }}>
                    Start from club #:
                  </label>
                  <input
                    id="start-club-input"
                    type="number"
                    min="1"
                    value={startFromClub}
                    onChange={(e) => setStartFromClub(parseInt(e.target.value) || 1)}
                    disabled={loading}
                    style={{
                      width: '80px',
                      padding: '8px',
                      border: '1px solid #ccc',
                      borderRadius: '4px',
                      fontSize: '14px'
                    }}
                  />
                  <button
                    className="batch-stats-button batch-stats-all"
                    onClick={handleGenerateStatsForAllClubs}
                    disabled={loading}
                  >
                    {loading ? 'Generating...' : 'Generate Stats for All Clubs'}
                  </button>
                </div>
              </div>

              {loading && progressMessage && (
                <div className="progress-message" style={{
                  padding: '10px',
                  margin: '10px 0',
                  backgroundColor: '#e3f2fd',
                  border: '1px solid #2196f3',
                  borderRadius: '4px',
                  color: '#1976d2',
                  fontWeight: 'bold'
                }}>
                  {progressMessage}
                </div>
              )}

              {squadPlayers.length === 0 ? (
                <div className="no-players">
                  No players found for this club. Use the Player Generation tab to add players.
                </div>
              ) : (
                <div className="players-list">
                  {squadPlayers.map((player) => (
                    <div
                      key={player.id}
                      className={`player-item ${selectedPlayerId === player.id ? 'selected' : ''}`}
                      onClick={() => setSelectedPlayerId(player.id)}
                    >
                      <div className="player-header">
                        <span className="player-name">{player.name}</span>
                        <span className="player-position">{player.position}</span>
                        <span className="player-age">Age: {player.age}</span>
                      </div>
                      <button
                        className="edit-player-button"
                        onClick={(e) => {
                          e.stopPropagation();
                          handleEditPlayer(player);
                        }}
                      >
                        Edit Stats
                      </button>
                    </div>
                  ))}
                </div>
              )}
            </div>

            {editingPlayer && (
              <div className="player-editor-panel">
                <div className="editor-header">
                  <h2>Edit Player: {editingPlayer.name}</h2>
                  <div className="player-info">
                    <div className="player-info-row">
                      <label>Position:</label>
                      <select
                        className="position-selector"
                        value={editingPlayer.position}
                        onChange={(e) =>
                          setEditingPlayer({ ...editingPlayer, position: e.target.value })
                        }
                      >
                        <option value="GK">GK - Goalkeeper</option>
                        <option value="CB">CB - Centre Back</option>
                        <option value="FB">FB - Full Back</option>
                        <option value="MID">MID - Midfielder</option>
                        <option value="FWD">FWD - Forward</option>
                        <option value="WG">WG - Winger</option>
                      </select>
                    </div>
                    <span>Age: {editingPlayer.age}</span>
                    <span>Nationality: {editingPlayer.nationality}</span>
                  </div>
                  <div className="editor-actions">
                    <button
                      className="randomize-button"
                      onClick={handleRandomizeStats}
                    >
                      Randomize Stats
                    </button>
                    <div className="preset-buttons">
                      <button
                        className="preset-button preset-elite"
                        onClick={() => handleApplyPreset('elite')}
                      >
                        Elite
                      </button>
                      <button
                        className="preset-button preset-good"
                        onClick={() => handleApplyPreset('good')}
                      >
                        Good
                      </button>
                      <button
                        className="preset-button preset-average"
                        onClick={() => handleApplyPreset('average')}
                      >
                        Average
                      </button>
                      <button
                        className="preset-button preset-poor"
                        onClick={() => handleApplyPreset('poor')}
                      >
                        Poor
                      </button>
                    </div>
                  </div>
                </div>

                <div className="stats-editor">
                  <div className="stats-section">
                    <h3>Physical Attributes</h3>
                    <div className="stats-grid">
                      <div className="stat-input">
                        <label>Pace</label>
                        <input
                          type="number"
                          min="1"
                          max="20"
                          value={editingPlayer.pace}
                          onChange={(e) =>
                            setEditingPlayer({ ...editingPlayer, pace: parseInt(e.target.value) || 1 })
                          }
                        />
                      </div>
                      <div className="stat-input">
                        <label>Strength</label>
                        <input
                          type="number"
                          min="1"
                          max="20"
                          value={editingPlayer.strength}
                          onChange={(e) =>
                            setEditingPlayer({ ...editingPlayer, strength: parseInt(e.target.value) || 1 })
                          }
                        />
                      </div>
                      <div className="stat-input">
                        <label>Stamina</label>
                        <input
                          type="number"
                          min="1"
                          max="20"
                          value={editingPlayer.stamina}
                          onChange={(e) =>
                            setEditingPlayer({ ...editingPlayer, stamina: parseInt(e.target.value) || 1 })
                          }
                        />
                      </div>
                      <div className="stat-input">
                        <label>Balance</label>
                        <input
                          type="number"
                          min="1"
                          max="20"
                          value={editingPlayer.balance}
                          onChange={(e) =>
                            setEditingPlayer({ ...editingPlayer, balance: parseInt(e.target.value) || 1 })
                          }
                        />
                      </div>
                      <div className="stat-input">
                        <label>Jumping</label>
                        <input
                          type="number"
                          min="1"
                          max="20"
                          value={editingPlayer.jumping}
                          onChange={(e) =>
                            setEditingPlayer({ ...editingPlayer, jumping: parseInt(e.target.value) || 1 })
                          }
                        />
                      </div>
                      <div className="stat-input">
                        <label>Agility</label>
                        <input
                          type="number"
                          min="1"
                          max="20"
                          value={editingPlayer.agility}
                          onChange={(e) =>
                            setEditingPlayer({ ...editingPlayer, agility: parseInt(e.target.value) || 1 })
                          }
                        />
                      </div>
                      <div className="stat-input">
                        <label>Acceleration</label>
                        <input
                          type="number"
                          min="1"
                          max="20"
                          value={editingPlayer.acceleration}
                          onChange={(e) =>
                            setEditingPlayer({ ...editingPlayer, acceleration: parseInt(e.target.value) || 1 })
                          }
                        />
                      </div>
                      <div className="stat-input">
                        <label>Natural Fitness</label>
                        <input
                          type="number"
                          min="1"
                          max="20"
                          value={editingPlayer.natural_fitness}
                          onChange={(e) =>
                            setEditingPlayer({ ...editingPlayer, natural_fitness: parseInt(e.target.value) || 1 })
                          }
                        />
                      </div>
                    </div>
                  </div>

                  <div className="stats-section">
                    <h3>Technical Attributes</h3>
                    <div className="stats-grid">
                      <div className="stat-input">
                        <label>Passing</label>
                        <input
                          type="number"
                          min="1"
                          max="20"
                          value={editingPlayer.passing}
                          onChange={(e) =>
                            setEditingPlayer({ ...editingPlayer, passing: parseInt(e.target.value) || 1 })
                          }
                        />
                      </div>
                      <div className="stat-input">
                        <label>Dribbling</label>
                        <input
                          type="number"
                          min="1"
                          max="20"
                          value={editingPlayer.dribbling}
                          onChange={(e) =>
                            setEditingPlayer({ ...editingPlayer, dribbling: parseInt(e.target.value) || 1 })
                          }
                        />
                      </div>
                      <div className="stat-input">
                        <label>Heading</label>
                        <input
                          type="number"
                          min="1"
                          max="20"
                          value={editingPlayer.heading}
                          onChange={(e) =>
                            setEditingPlayer({ ...editingPlayer, heading: parseInt(e.target.value) || 1 })
                          }
                        />
                      </div>
                      <div className="stat-input">
                        <label>Crossing</label>
                        <input
                          type="number"
                          min="1"
                          max="20"
                          value={editingPlayer.crossing}
                          onChange={(e) =>
                            setEditingPlayer({ ...editingPlayer, crossing: parseInt(e.target.value) || 1 })
                          }
                        />
                      </div>
                      <div className="stat-input">
                        <label>Tackling</label>
                        <input
                          type="number"
                          min="1"
                          max="20"
                          value={editingPlayer.tackling}
                          onChange={(e) =>
                            setEditingPlayer({ ...editingPlayer, tackling: parseInt(e.target.value) || 1 })
                          }
                        />
                      </div>
                      <div className="stat-input">
                        <label>Handling</label>
                        <input
                          type="number"
                          min="1"
                          max="20"
                          value={editingPlayer.handling}
                          onChange={(e) =>
                            setEditingPlayer({ ...editingPlayer, handling: parseInt(e.target.value) || 1 })
                          }
                        />
                      </div>
                      <div className="stat-input">
                        <label>Reflexes</label>
                        <input
                          type="number"
                          min="1"
                          max="20"
                          value={editingPlayer.reflexes}
                          onChange={(e) =>
                            setEditingPlayer({ ...editingPlayer, reflexes: parseInt(e.target.value) || 1 })
                          }
                        />
                      </div>
                      <div className="stat-input">
                        <label>First Touch</label>
                        <input
                          type="number"
                          min="1"
                          max="20"
                          value={editingPlayer.first_touch}
                          onChange={(e) =>
                            setEditingPlayer({ ...editingPlayer, first_touch: parseInt(e.target.value) || 1 })
                          }
                        />
                      </div>
                      <div className="stat-input">
                        <label>Technique</label>
                        <input
                          type="number"
                          min="1"
                          max="20"
                          value={editingPlayer.technique}
                          onChange={(e) =>
                            setEditingPlayer({ ...editingPlayer, technique: parseInt(e.target.value) || 1 })
                          }
                        />
                      </div>
                      <div className="stat-input">
                        <label>Long Passing</label>
                        <input
                          type="number"
                          min="1"
                          max="20"
                          value={editingPlayer.long_passing}
                          onChange={(e) =>
                            setEditingPlayer({ ...editingPlayer, long_passing: parseInt(e.target.value) || 1 })
                          }
                        />
                      </div>
                      <div className="stat-input">
                        <label>Long Shots</label>
                        <input
                          type="number"
                          min="1"
                          max="20"
                          value={editingPlayer.long_shots}
                          onChange={(e) =>
                            setEditingPlayer({ ...editingPlayer, long_shots: parseInt(e.target.value) || 1 })
                          }
                        />
                      </div>
                      <div className="stat-input">
                        <label>Corners</label>
                        <input
                          type="number"
                          min="1"
                          max="20"
                          value={editingPlayer.corners}
                          onChange={(e) =>
                            setEditingPlayer({ ...editingPlayer, corners: parseInt(e.target.value) || 1 })
                          }
                        />
                      </div>
                      <div className="stat-input">
                        <label>Free Kicks</label>
                        <input
                          type="number"
                          min="1"
                          max="20"
                          value={editingPlayer.free_kicks}
                          onChange={(e) =>
                            setEditingPlayer({ ...editingPlayer, free_kicks: parseInt(e.target.value) || 1 })
                          }
                        />
                      </div>
                      <div className="stat-input">
                        <label>Throw Ins</label>
                        <input
                          type="number"
                          min="1"
                          max="20"
                          value={editingPlayer.throw_ins}
                          onChange={(e) =>
                            setEditingPlayer({ ...editingPlayer, throw_ins: parseInt(e.target.value) || 1 })
                          }
                        />
                      </div>
                      <div className="stat-input">
                        <label>Vision</label>
                        <input
                          type="number"
                          min="1"
                          max="20"
                          value={editingPlayer.vision}
                          onChange={(e) =>
                            setEditingPlayer({ ...editingPlayer, vision: parseInt(e.target.value) || 1 })
                          }
                        />
                      </div>
                      <div className="stat-input">
                        <label>Left Foot</label>
                        <input
                          type="number"
                          min="1"
                          max="20"
                          value={editingPlayer.left_foot}
                          onChange={(e) =>
                            setEditingPlayer({ ...editingPlayer, left_foot: parseInt(e.target.value) || 1 })
                          }
                        />
                      </div>
                      <div className="stat-input">
                        <label>Right Foot</label>
                        <input
                          type="number"
                          min="1"
                          max="20"
                          value={editingPlayer.right_foot}
                          onChange={(e) =>
                            setEditingPlayer({ ...editingPlayer, right_foot: parseInt(e.target.value) || 1 })
                          }
                        />
                      </div>
                      <div className="stat-input">
                        <label>One on Ones</label>
                        <input
                          type="number"
                          min="1"
                          max="20"
                          value={editingPlayer.one_on_ones}
                          onChange={(e) =>
                            setEditingPlayer({ ...editingPlayer, one_on_ones: parseInt(e.target.value) || 1 })
                          }
                        />
                      </div>
                    </div>
                  </div>

                  <div className="stats-section">
                    <h3>Mental Attributes</h3>
                    <div className="stats-grid">
                      <div className="stat-input">
                        <label>Courage</label>
                        <input
                          type="number"
                          min="1"
                          max="20"
                          value={editingPlayer.courage}
                          onChange={(e) =>
                            setEditingPlayer({ ...editingPlayer, courage: parseInt(e.target.value) || 1 })
                          }
                        />
                      </div>
                      <div className="stat-input">
                        <label>Concentration</label>
                        <input
                          type="number"
                          min="1"
                          max="20"
                          value={editingPlayer.concentration}
                          onChange={(e) =>
                            setEditingPlayer({ ...editingPlayer, concentration: parseInt(e.target.value) || 1 })
                          }
                        />
                      </div>
                      <div className="stat-input">
                        <label>Leadership</label>
                        <input
                          type="number"
                          min="1"
                          max="20"
                          value={editingPlayer.leadership}
                          onChange={(e) =>
                            setEditingPlayer({ ...editingPlayer, leadership: parseInt(e.target.value) || 1 })
                          }
                        />
                      </div>
                      <div className="stat-input">
                        <label>Aggression</label>
                        <input
                          type="number"
                          min="1"
                          max="20"
                          value={editingPlayer.aggression}
                          onChange={(e) =>
                            setEditingPlayer({ ...editingPlayer, aggression: parseInt(e.target.value) || 1 })
                          }
                        />
                      </div>
                      <div className="stat-input">
                        <label>Determination</label>
                        <input
                          type="number"
                          min="1"
                          max="20"
                          value={editingPlayer.determination}
                          onChange={(e) =>
                            setEditingPlayer({ ...editingPlayer, determination: parseInt(e.target.value) || 1 })
                          }
                        />
                      </div>
                      <div className="stat-input">
                        <label>Flair</label>
                        <input
                          type="number"
                          min="1"
                          max="20"
                          value={editingPlayer.flair}
                          onChange={(e) =>
                            setEditingPlayer({ ...editingPlayer, flair: parseInt(e.target.value) || 1 })
                          }
                        />
                      </div>
                      <div className="stat-input">
                        <label>Influence</label>
                        <input
                          type="number"
                          min="1"
                          max="20"
                          value={editingPlayer.influence}
                          onChange={(e) =>
                            setEditingPlayer({ ...editingPlayer, influence: parseInt(e.target.value) || 1 })
                          }
                        />
                      </div>
                      <div className="stat-input">
                        <label>Bravery</label>
                        <input
                          type="number"
                          min="1"
                          max="20"
                          value={editingPlayer.bravery}
                          onChange={(e) =>
                            setEditingPlayer({ ...editingPlayer, bravery: parseInt(e.target.value) || 1 })
                          }
                        />
                      </div>
                      <div className="stat-input">
                        <label>Decision Making</label>
                        <input
                          type="number"
                          min="1"
                          max="20"
                          value={editingPlayer.decision_making}
                          onChange={(e) =>
                            setEditingPlayer({ ...editingPlayer, decision_making: parseInt(e.target.value) || 1 })
                          }
                        />
                      </div>
                      <div className="stat-input">
                        <label>Anticipation</label>
                        <input
                          type="number"
                          min="1"
                          max="20"
                          value={editingPlayer.anticipation}
                          onChange={(e) =>
                            setEditingPlayer({ ...editingPlayer, anticipation: parseInt(e.target.value) || 1 })
                          }
                        />
                      </div>
                      <div className="stat-input">
                        <label>Adaptability</label>
                        <input
                          type="number"
                          min="1"
                          max="20"
                          value={editingPlayer.adaptability}
                          onChange={(e) =>
                            setEditingPlayer({ ...editingPlayer, adaptability: parseInt(e.target.value) || 1 })
                          }
                        />
                      </div>
                      <div className="stat-input">
                        <label>Ambition</label>
                        <input
                          type="number"
                          min="1"
                          max="20"
                          value={editingPlayer.ambition}
                          onChange={(e) =>
                            setEditingPlayer({ ...editingPlayer, ambition: parseInt(e.target.value) || 1 })
                          }
                        />
                      </div>
                      <div className="stat-input">
                        <label>Loyalty</label>
                        <input
                          type="number"
                          min="1"
                          max="20"
                          value={editingPlayer.loyalty}
                          onChange={(e) =>
                            setEditingPlayer({ ...editingPlayer, loyalty: parseInt(e.target.value) || 1 })
                          }
                        />
                      </div>
                      <div className="stat-input">
                        <label>Pressure</label>
                        <input
                          type="number"
                          min="1"
                          max="20"
                          value={editingPlayer.pressure}
                          onChange={(e) =>
                            setEditingPlayer({ ...editingPlayer, pressure: parseInt(e.target.value) || 1 })
                          }
                        />
                      </div>
                      <div className="stat-input">
                        <label>Professionalism</label>
                        <input
                          type="number"
                          min="1"
                          max="20"
                          value={editingPlayer.professionalism}
                          onChange={(e) =>
                            setEditingPlayer({ ...editingPlayer, professionalism: parseInt(e.target.value) || 1 })
                          }
                        />
                      </div>
                      <div className="stat-input">
                        <label>Sportsmanship</label>
                        <input
                          type="number"
                          min="1"
                          max="20"
                          value={editingPlayer.sportsmanship}
                          onChange={(e) =>
                            setEditingPlayer({ ...editingPlayer, sportsmanship: parseInt(e.target.value) || 1 })
                          }
                        />
                      </div>
                      <div className="stat-input">
                        <label>Temperament</label>
                        <input
                          type="number"
                          min="1"
                          max="20"
                          value={editingPlayer.temperament}
                          onChange={(e) =>
                            setEditingPlayer({ ...editingPlayer, temperament: parseInt(e.target.value) || 1 })
                          }
                        />
                      </div>
                    </div>
                  </div>

                  <div className="stats-section">
                    <h3>Positioning Attributes</h3>
                    <div className="stats-grid">
                      <div className="stat-input">
                        <label>Awareness</label>
                        <input
                          type="number"
                          min="1"
                          max="20"
                          value={editingPlayer.awareness}
                          onChange={(e) =>
                            setEditingPlayer({ ...editingPlayer, awareness: parseInt(e.target.value) || 1 })
                          }
                        />
                      </div>
                      <div className="stat-input">
                        <label>Marking</label>
                        <input
                          type="number"
                          min="1"
                          max="20"
                          value={editingPlayer.marking}
                          onChange={(e) =>
                            setEditingPlayer({ ...editingPlayer, marking: parseInt(e.target.value) || 1 })
                          }
                        />
                      </div>
                      <div className="stat-input">
                        <label>Positioning</label>
                        <input
                          type="number"
                          min="1"
                          max="20"
                          value={editingPlayer.positioning}
                          onChange={(e) =>
                            setEditingPlayer({ ...editingPlayer, positioning: parseInt(e.target.value) || 1 })
                          }
                        />
                      </div>
                      <div className="stat-input">
                        <label>Work Rate</label>
                        <input
                          type="number"
                          min="1"
                          max="20"
                          value={editingPlayer.work_rate}
                          onChange={(e) =>
                            setEditingPlayer({ ...editingPlayer, work_rate: parseInt(e.target.value) || 1 })
                          }
                        />
                      </div>
                      <div className="stat-input">
                        <label>Off the Ball</label>
                        <input
                          type="number"
                          min="1"
                          max="20"
                          value={editingPlayer.off_the_ball}
                          onChange={(e) =>
                            setEditingPlayer({ ...editingPlayer, off_the_ball: parseInt(e.target.value) || 1 })
                          }
                        />
                      </div>
                      <div className="stat-input">
                        <label>Movement</label>
                        <input
                          type="number"
                          min="1"
                          max="20"
                          value={editingPlayer.movement}
                          onChange={(e) =>
                            setEditingPlayer({ ...editingPlayer, movement: parseInt(e.target.value) || 1 })
                          }
                        />
                      </div>
                      <div className="stat-input">
                        <label>Teamwork</label>
                        <input
                          type="number"
                          min="1"
                          max="20"
                          value={editingPlayer.teamwork}
                          onChange={(e) =>
                            setEditingPlayer({ ...editingPlayer, teamwork: parseInt(e.target.value) || 1 })
                          }
                        />
                      </div>
                    </div>
                  </div>

                  <div className="stats-section">
                    <h3>Specialization Attributes</h3>
                    <div className="stats-grid">
                      <div className="stat-input">
                        <label>Finishing</label>
                        <input
                          type="number"
                          min="1"
                          max="20"
                          value={editingPlayer.finishing}
                          onChange={(e) =>
                            setEditingPlayer({ ...editingPlayer, finishing: parseInt(e.target.value) || 1 })
                          }
                        />
                      </div>
                      <div className="stat-input">
                        <label>Penalties</label>
                        <input
                          type="number"
                          min="1"
                          max="20"
                          value={editingPlayer.penalties}
                          onChange={(e) =>
                            setEditingPlayer({ ...editingPlayer, penalties: parseInt(e.target.value) || 1 })
                          }
                        />
                      </div>
                      <div className="stat-input">
                        <label>Set Pieces</label>
                        <input
                          type="number"
                          min="1"
                          max="20"
                          value={editingPlayer.set_pieces}
                          onChange={(e) =>
                            setEditingPlayer({ ...editingPlayer, set_pieces: parseInt(e.target.value) || 1 })
                          }
                        />
                      </div>
                    </div>
                  </div>

                  <div className="stats-section">
                    <h3>Hidden Attributes</h3>
                    <div className="stats-grid">
                      <div className="stat-input">
                        <label>Consistency</label>
                        <input
                          type="number"
                          min="1"
                          max="20"
                          value={editingPlayer.consistency}
                          onChange={(e) =>
                            setEditingPlayer({ ...editingPlayer, consistency: parseInt(e.target.value) || 1 })
                          }
                        />
                      </div>
                      <div className="stat-input">
                        <label>Dirtiness</label>
                        <input
                          type="number"
                          min="1"
                          max="20"
                          value={editingPlayer.dirtiness}
                          onChange={(e) =>
                            setEditingPlayer({ ...editingPlayer, dirtiness: parseInt(e.target.value) || 1 })
                          }
                        />
                      </div>
                      <div className="stat-input">
                        <label>Versatility</label>
                        <input
                          type="number"
                          min="1"
                          max="20"
                          value={editingPlayer.versatility}
                          onChange={(e) =>
                            setEditingPlayer({ ...editingPlayer, versatility: parseInt(e.target.value) || 1 })
                          }
                        />
                      </div>
                      <div className="stat-input">
                        <label>Injury Proneness</label>
                        <input
                          type="number"
                          min="1"
                          max="20"
                          value={editingPlayer.injury_proneness}
                          onChange={(e) =>
                            setEditingPlayer({ ...editingPlayer, injury_proneness: parseInt(e.target.value) || 1 })
                          }
                        />
                      </div>
                      <div className="stat-input">
                        <label>Important Matches</label>
                        <input
                          type="number"
                          min="1"
                          max="20"
                          value={editingPlayer.important_matches}
                          onChange={(e) =>
                            setEditingPlayer({ ...editingPlayer, important_matches: parseInt(e.target.value) || 1 })
                          }
                        />
                      </div>
                    </div>
                  </div>

                  <div className="stats-section">
                    <h3>Ability & Reputation</h3>
                    <div className="stats-grid">
                      <div className="stat-input">
                        <label>Current Ability (1-200)</label>
                        <input
                          type="number"
                          min="1"
                          max="200"
                          value={editingPlayer.current_ability}
                          onChange={(e) =>
                            setEditingPlayer({ ...editingPlayer, current_ability: parseInt(e.target.value) || 1 })
                          }
                        />
                      </div>
                      <div className="stat-input">
                        <label>Potential Ability (1-200)</label>
                        <input
                          type="number"
                          min="1"
                          max="200"
                          value={editingPlayer.potential_ability}
                          onChange={(e) =>
                            setEditingPlayer({ ...editingPlayer, potential_ability: parseInt(e.target.value) || 1 })
                          }
                        />
                      </div>
                      <div className="stat-input">
                        <label>Current Reputation (1-200)</label>
                        <input
                          type="number"
                          min="1"
                          max="200"
                          value={editingPlayer.current_reputation}
                          onChange={(e) =>
                            setEditingPlayer({ ...editingPlayer, current_reputation: parseInt(e.target.value) || 1 })
                          }
                        />
                      </div>
                    </div>
                  </div>
                </div>

                <div className="editor-actions">
                  <button onClick={handleSavePlayer} className="save-button">
                    Save Changes
                  </button>
                  <button onClick={handleCancelEditPlayer} className="cancel-button">
                    Cancel
                  </button>
                </div>
              </div>
            )}
          </div>
        )}

        {/* PLAYER DATABASE TAB */}
        {activeTab === 'player-database' && (
          <div className="player-database-container">
            <h2>Player Database</h2>
            <p className="tab-description">All players in the database</p>

            <div className="player-database-toolbar">
              <input
                type="text"
                placeholder="Search by name..."
                value={searchInputValue}
                onChange={(e) => setSearchInputValue(e.target.value)}
                className="player-search-input"
              />
              <button
                onClick={() => setShowDuplicatesOnly(!showDuplicatesOnly)}
                className={`duplicates-toggle-btn ${showDuplicatesOnly ? 'active' : ''}`}
              >
                {showDuplicatesOnly ? 'Show All' : 'Show Duplicates Only'}
              </button>
              <button onClick={loadAllPlayers} className="refresh-players-btn" disabled={isLoadingPlayers}>
                {isLoadingPlayers ? 'Loading...' : `Refresh (${totalPlayers.toLocaleString()} total)`}
              </button>
            </div>

            <div className="player-database-toolbar" style={{marginTop: '10px'}}>
              <button
                onClick={() => setShowAgeStats(!showAgeStats)}
                className={`duplicates-toggle-btn ${showAgeStats ? 'active' : ''}`}
              >
                {showAgeStats ? 'Hide' : 'Show'} Age Stats
              </button>
              <button
                onClick={async () => {
                  if (!confirm('This will expand common name abbreviations (e.g., Thos. → Thomas, Wm. → William) for all players.\n\nThis action cannot be undone.\n\nContinue?')) {
                    return;
                  }
                  try {
                    setError('');
                    const updated = await invoke<number>('db_expand_name_abbreviations');
                    setSuccess(`Expanded abbreviations in ${updated} player names`);
                    await loadAllPlayers();
                    setTimeout(() => setSuccess(''), 8000);
                  } catch (err) {
                    setError(`Failed to expand abbreviations: ${err}`);
                  }
                }}
                className="duplicates-toggle-btn"
                style={{backgroundColor: '#6c757d'}}
              >
                Expand Name Abbreviations
              </button>
              <button
                onClick={() => setShowFirstNameStats(!showFirstNameStats)}
                className={`duplicates-toggle-btn ${showFirstNameStats ? 'active' : ''}`}
              >
                {showFirstNameStats ? 'Hide' : 'Show'} First Names
              </button>
              <button
                onClick={() => setShowMiddleNameStats(!showMiddleNameStats)}
                className={`duplicates-toggle-btn ${showMiddleNameStats ? 'active' : ''}`}
              >
                {showMiddleNameStats ? 'Hide' : 'Show'} Middle Names
              </button>
              <button
                onClick={() => setShowSurnameStats(!showSurnameStats)}
                className={`duplicates-toggle-btn ${showSurnameStats ? 'active' : ''}`}
              >
                {showSurnameStats ? 'Hide' : 'Show'} Surnames
              </button>
            </div>

            <div className="parish-filter-section">
              <div className="parish-filter-controls">
                <select
                  value={selectedParish}
                  onChange={(e) => {
                    setSelectedParish(e.target.value);
                    setCurrentPage(0); // Reset to first page
                  }}
                  className="parish-filter-select"
                >
                  <option value="">All Parishes</option>
                  {allParishes.map(parish => (
                    <option key={parish} value={parish}>{parish}</option>
                  ))}
                </select>

                <select
                  value={selectedBirthYear}
                  onChange={(e) => {
                    setSelectedBirthYear(e.target.value);
                    setCurrentPage(0); // Reset to first page
                  }}
                  className="parish-filter-select"
                >
                  <option value="">All Birth Years</option>
                  {allBirthYears.map(year => (
                    <option key={year} value={year}>{year}</option>
                  ))}
                </select>

                <select
                  value={assignmentFilter}
                  onChange={(e) => {
                    setAssignmentFilter(e.target.value);
                    setCurrentPage(0); // Reset to first page
                  }}
                  className="parish-filter-select"
                >
                  <option value="all">All Players</option>
                  <option value="assigned">Assigned Only</option>
                  <option value="unassigned">Unassigned Only</option>
                </select>

                {selectedParish && (
                  <div className="parish-postcode-assignment">
                    <input
                      type="text"
                      placeholder="Postcode for parish..."
                      value={parishPostcode}
                      onChange={(e) => setParishPostcode(e.target.value)}
                      className="parish-postcode-input"
                    />
                    <button
                      onClick={assignPostcodeToParish}
                      className="assign-postcode-btn"
                      disabled={!parishPostcode.trim()}
                    >
                      Assign to {allPlayers.filter(p => p.ecclesiastical_parish === selectedParish).length} players
                    </button>
                  </div>
                )}
              </div>
            </div>

            {error && <div className="error-message">{error}</div>}
            {success && <div className="success-message">{success}</div>}

            {/* Age Statistics Table */}
            {showAgeStats && (
              <div className="age-stats-container" style={{marginBottom: '20px', background: '#f5f5f5', padding: '15px', borderRadius: '8px'}}>
                <h3 style={{marginBottom: '15px'}}>Player Age Distribution (as of {year})</h3>
                <div style={{maxHeight: '400px', overflow: 'auto'}}>
                  <table className="player-database-table">
                    <thead>
                      <tr>
                        <th>Age</th>
                        <th>Birth Year</th>
                        <th>Count</th>
                      </tr>
                    </thead>
                    <tbody>
                      {ageStats.map(stat => (
                        <tr key={stat.age}>
                          <td style={{fontWeight: 'bold'}}>{stat.age} years old</td>
                          <td>{stat.birth_year}</td>
                          <td style={{fontWeight: 'bold', color: '#2c3e50'}}>{stat.count}</td>
                        </tr>
                      ))}
                    </tbody>
                  </table>
                </div>
                <p style={{marginTop: '10px', fontSize: '0.9em', color: '#666'}}>
                  Total players with birth year data: {ageStats.reduce((sum, stat) => sum + stat.count, 0)}
                </p>
              </div>
            )}

            {/* First Name Statistics */}
            {showFirstNameStats && (
              <div className="name-stats-container" style={{marginBottom: '20px', background: '#e8f4f8', padding: '15px', borderRadius: '8px'}}>
                <h3 style={{marginBottom: '15px'}}>First Name Distribution</h3>
                <div style={{maxHeight: '400px', overflow: 'auto'}}>
                  <table className="player-database-table">
                    <thead>
                      <tr>
                        <th>First Name</th>
                        <th>Count</th>
                      </tr>
                    </thead>
                    <tbody>
                      {firstNameStats.map(stat => (
                        <tr key={stat.name}>
                          <td style={{fontWeight: 'bold'}}>{stat.name}</td>
                          <td style={{fontWeight: 'bold', color: '#2c3e50'}}>{stat.count}</td>
                        </tr>
                      ))}
                    </tbody>
                  </table>
                </div>
                <p style={{marginTop: '10px', fontSize: '0.9em', color: '#666'}}>
                  Total unique first names: {firstNameStats.length}
                </p>
              </div>
            )}

            {/* Middle Name Statistics */}
            {showMiddleNameStats && (
              <div className="name-stats-container" style={{marginBottom: '20px', background: '#f8e8f4', padding: '15px', borderRadius: '8px'}}>
                <h3 style={{marginBottom: '15px'}}>Middle Name Distribution</h3>
                <div style={{maxHeight: '400px', overflow: 'auto'}}>
                  <table className="player-database-table">
                    <thead>
                      <tr>
                        <th>Middle Name</th>
                        <th>Count</th>
                      </tr>
                    </thead>
                    <tbody>
                      {(() => {
                        const nameCounts: { [name: string]: number } = {};

                        allPlayers.forEach(player => {
                          const middleName = player.middle_name?.trim();
                          if (middleName) {
                            nameCounts[middleName] = (nameCounts[middleName] || 0) + 1;
                          }
                        });

                        return Object.entries(nameCounts)
                          .sort((a, b) => b[1] - a[1])
                          .map(([name, count]) => (
                            <tr key={name}>
                              <td style={{fontWeight: 'bold'}}>{name}</td>
                              <td style={{fontWeight: 'bold', color: '#2c3e50'}}>{count}</td>
                            </tr>
                          ));
                      })()}
                    </tbody>
                  </table>
                </div>
                <p style={{marginTop: '10px', fontSize: '0.9em', color: '#666'}}>
                  Total unique middle names: {Object.keys(allPlayers.reduce((acc: any, p) => {
                    if (p.middle_name?.trim()) acc[p.middle_name.trim()] = true;
                    return acc;
                  }, {})).length} | Players with middle names: {allPlayers.filter(p => p.middle_name?.trim()).length}
                </p>
              </div>
            )}

            {/* Surname Statistics */}
            {showSurnameStats && (
              <div className="name-stats-container" style={{marginBottom: '20px', background: '#f4f8e8', padding: '15px', borderRadius: '8px'}}>
                <h3 style={{marginBottom: '15px'}}>Surname Distribution</h3>
                <div style={{maxHeight: '400px', overflow: 'auto'}}>
                  <table className="player-database-table">
                    <thead>
                      <tr>
                        <th>Surname</th>
                        <th>Count</th>
                      </tr>
                    </thead>
                    <tbody>
                      {surnameStats.map(stat => (
                        <tr key={stat.name}>
                          <td style={{fontWeight: 'bold'}}>{stat.name}</td>
                          <td style={{fontWeight: 'bold', color: '#2c3e50'}}>{stat.count}</td>
                        </tr>
                      ))}
                    </tbody>
                  </table>
                </div>
                <p style={{marginTop: '10px', fontSize: '0.9em', color: '#666'}}>
                  Total unique surnames: {surnameStats.length}
                </p>
              </div>
            )}

            {/* Pagination Controls */}
            <div className="pagination-controls" style={{
              display: 'flex',
              justifyContent: 'space-between',
              alignItems: 'center',
              padding: '15px 0',
              borderTop: '1px solid #ddd',
              borderBottom: '1px solid #ddd',
              marginBottom: '10px',
              background: '#f8f8f8'
            }}>
              <div className="pagination-info" style={{fontSize: '14px', fontWeight: 'bold'}}>
                Showing {totalPlayers > 0 ? (currentPage * pageSize + 1).toLocaleString() : 0}-{Math.min((currentPage + 1) * pageSize, totalPlayers).toLocaleString()} of {totalPlayers.toLocaleString()} players
              </div>

              <div className="pagination-buttons" style={{display: 'flex', gap: '5px', alignItems: 'center'}}>
                <button
                  onClick={() => setCurrentPage(0)}
                  disabled={currentPage === 0 || isLoadingPlayers}
                  className="pagination-btn"
                  style={{padding: '5px 10px', opacity: currentPage === 0 || isLoadingPlayers ? 0.5 : 1}}
                >
                  ‹‹ First
                </button>
                <button
                  onClick={() => setCurrentPage(prev => Math.max(0, prev - 1))}
                  disabled={currentPage === 0 || isLoadingPlayers}
                  className="pagination-btn"
                  style={{padding: '5px 10px', opacity: currentPage === 0 || isLoadingPlayers ? 0.5 : 1}}
                >
                  ‹ Previous
                </button>
                <span className="page-indicator" style={{padding: '0 15px', fontWeight: 'bold'}}>
                  Page {currentPage + 1} of {Math.max(1, Math.ceil(totalPlayers / pageSize))}
                </span>
                <button
                  onClick={() => setCurrentPage(prev => prev + 1)}
                  disabled={(currentPage + 1) * pageSize >= totalPlayers || isLoadingPlayers}
                  className="pagination-btn"
                  style={{padding: '5px 10px', opacity: (currentPage + 1) * pageSize >= totalPlayers || isLoadingPlayers ? 0.5 : 1}}
                >
                  Next ›
                </button>
                <button
                  onClick={() => setCurrentPage(Math.ceil(totalPlayers / pageSize) - 1)}
                  disabled={(currentPage + 1) * pageSize >= totalPlayers || isLoadingPlayers}
                  className="pagination-btn"
                  style={{padding: '5px 10px', opacity: (currentPage + 1) * pageSize >= totalPlayers || isLoadingPlayers ? 0.5 : 1}}
                >
                  Last ››
                </button>
              </div>

              <div className="page-size-selector" style={{display: 'flex', alignItems: 'center', gap: '8px'}}>
                <label style={{fontWeight: 'bold'}}>Per page:</label>
                <select
                  value={pageSize}
                  onChange={(e) => {
                    setPageSize(parseInt(e.target.value));
                    setCurrentPage(0); // Reset to first page
                  }}
                  className="page-size-select"
                  style={{padding: '5px'}}
                >
                  <option value="50">50</option>
                  <option value="100">100</option>
                  <option value="250">250</option>
                  <option value="500">500</option>
                </select>
              </div>
            </div>

            {/* Horizontal Scroll Controls */}
            <div className="table-scroll-controls" style={{
              display: 'flex',
              justifyContent: 'center',
              alignItems: 'center',
              padding: '10px 0',
              gap: '10px',
              background: '#f0f0f0',
              borderBottom: '1px solid #ddd'
            }}>
              <button
                onClick={() => scrollTable('left')}
                disabled={!canScrollLeft}
                className="scroll-arrow-btn"
                style={{
                  padding: '8px 16px',
                  fontSize: '16px',
                  fontWeight: 'bold',
                  cursor: canScrollLeft ? 'pointer' : 'not-allowed',
                  opacity: canScrollLeft ? 1 : 0.3,
                  background: canScrollLeft ? '#4CAF50' : '#ccc',
                  color: 'white',
                  border: 'none',
                  borderRadius: '4px',
                  transition: 'all 0.2s'
                }}
              >
                ← Scroll Left
              </button>
              <span style={{fontSize: '14px', color: '#666'}}>Scroll table horizontally</span>
              <button
                onClick={() => scrollTable('right')}
                disabled={!canScrollRight}
                className="scroll-arrow-btn"
                style={{
                  padding: '8px 16px',
                  fontSize: '16px',
                  fontWeight: 'bold',
                  cursor: canScrollRight ? 'pointer' : 'not-allowed',
                  opacity: canScrollRight ? 1 : 0.3,
                  background: canScrollRight ? '#4CAF50' : '#ccc',
                  color: 'white',
                  border: 'none',
                  borderRadius: '4px',
                  transition: 'all 0.2s'
                }}
              >
                Scroll Right →
              </button>
            </div>

            <div className="player-database-table-container" style={{position: 'relative'}}>
              {isLoadingPlayers && (
                <div style={{
                  position: 'absolute',
                  top: 0,
                  left: 0,
                  right: 0,
                  bottom: 0,
                  background: 'rgba(255, 255, 255, 0.8)',
                  display: 'flex',
                  alignItems: 'center',
                  justifyContent: 'center',
                  zIndex: 10
                }}>
                  <div style={{fontSize: '18px', fontWeight: 'bold'}}>Loading players...</div>
                </div>
              )}

              <div
                ref={tableScrollRef}
                onScroll={updateScrollButtons}
                className="player-table-scroll-wrapper"
                style={{
                  overflowX: 'auto',
                  overflowY: 'visible',
                  width: '100%',
                  border: '1px solid #ddd',
                  borderRadius: '4px'
                }}
              >
                <table className="player-database-table">
                <thead>
                  <tr>
                    <th>First Name</th>
                    <th>Middle Name</th>
                    <th>Surname</th>
                    <th>Club</th>
                    <th>Position</th>
                    <th>Birth Year</th>
                    <th>Nationality</th>
                    <th>Location</th>
                    <th>Parish</th>
                    <th>Assigned Postcode</th>
                    <th>Stats</th>
                    <th>Actions</th>
                  </tr>
                </thead>
                <tbody>
                  {isLoadingPlayers && allPlayers.length === 0 ? (
                    <tr>
                      <td colSpan={12} style={{textAlign: 'center', padding: '40px'}}>
                        Loading players...
                      </td>
                    </tr>
                  ) : allPlayers.length === 0 ? (
                    <tr>
                      <td colSpan={12} style={{textAlign: 'center', padding: '40px'}}>
                        No players found matching filters
                      </td>
                    </tr>
                  ) : (
                    allPlayers.map((player: any) => (
                      <tr key={player.id}>
                        <td className="player-name-cell">{player.first_name || ''}</td>
                        <td className="player-middle-name-cell">{player.middle_name || ''}</td>
                        <td className="player-name-cell">{player.surname || ''}</td>
                        <td>
                          {player.club_id === 'UNASSIGNED' ? (
                            <span className="stats-badge no-stats">Unassigned</span>
                          ) : (
                            <span className="stats-badge has-stats">
                              {clubs.find(c => c.id === player.club_id)?.name || player.club_id}
                            </span>
                          )}
                        </td>
                        <td>{player.position}</td>
                        <td>{player.birth_year}</td>
                        <td>{player.nationality || 'N/A'}</td>
                        <td className="player-location-cell">
                          {player.birth_town && player.birth_county
                            ? `${player.birth_town}, ${player.birth_county}`
                            : 'N/A'}
                        </td>
                        <td className="player-parish-cell">
                          {player.ecclesiastical_parish || 'N/A'}
                        </td>
                        <td>{player.postcode || 'N/A'}</td>
                        <td>
                          <span className={`stats-badge ${player.has_stats ? 'has-stats' : 'no-stats'}`}>
                            {player.has_stats ? 'Yes' : 'No'}
                          </span>
                        </td>
                        <td>
                          <button
                            onClick={() => handleDeletePlayerWithConfirm(player.id, player.name)}
                            className="delete-player-btn"
                          >
                            Delete
                          </button>
                        </td>
                      </tr>
                    ))
                  )}
                </tbody>
              </table>
              </div>
            </div>
          </div>
        )}

        {/* LEAGUE MANAGEMENT TAB */}
        {activeTab === 'leagues' && (
          <LeagueManagementScreen />
        )}

        {/* CUP MANAGEMENT TAB */}
        {activeTab === 'cups' && (
          <CupManagement season={cupSeason} />
        )}

      </div>

      {/* Confirmation Dialog */}
      <ConfirmationDialog
        isOpen={confirmDialog.isOpen}
        title={confirmDialog.title}
        message={confirmDialog.message}
        onConfirm={confirmDialog.onConfirm}
        onCancel={() => setConfirmDialog(prev => ({ ...prev, isOpen: false }))}
        danger={confirmDialog.danger}
      />
    </div>
  );
};

export default DatabaseEditorScreen;
