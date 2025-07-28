import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import type { 
  Unit, 
  LeagueModifier, 
  MatchData, 
  BattleSimulation,
  GameConfig 
} from '@/types/game';

interface GameState {
  // Configuration
  config: GameConfig;
  
  // League selection
  selectedLeague: number;
  availableLeagues: LeagueModifier[];
  
  // Battle units
  playerUnits: Unit[];
  
  // Match state
  currentMatch: MatchData | null;
  matchHistory: MatchData[];
  
  // Battle simulation
  battleSim: BattleSimulation;
  
  // UI state
  isLoading: boolean;
  error: string | null;
  
  // Actions
  setConfig: (config: GameConfig) => void;
  selectLeague: (leagueId: number) => void;
  loadLeagueInfo: () => Promise<void>;
  setPlayerUnits: (units: Unit[]) => void;
  updateMatch: (match: MatchData) => void;
  setBattleSimulation: (simulation: BattleSimulation) => void;
  clearError: () => void;
}

const defaultConfig: GameConfig = {
  mintUrl: 'http://localhost:3333',
  nostrRelays: ['ws://localhost:7777'],
  gameEngineUrl: 'http://localhost:4444',
};

const defaultLeagues: LeagueModifier[] = [
  {
    id: 0,
    name: 'Fire League',
    attack_bonus: 10,
    defense_bonus: 0,
    health_bonus: 0,
  },
  {
    id: 1,
    name: 'Ice League',
    attack_bonus: 0,
    defense_bonus: 0,
    health_bonus: 20,
  },
  {
    id: 2,
    name: 'Shadow League',
    attack_bonus: 5,
    defense_bonus: 5,
    health_bonus: 0,
  },
  {
    id: 3,
    name: 'Nature League',
    attack_bonus: 0,
    defense_bonus: 5,
    health_bonus: 15,
  },
];

const defaultBattleSimulation: BattleSimulation = {
  unit1: {
    attack: 15,
    defense: 8,
    health: 30,
    max_health: 30,
    ability: 'None',
  },
  unit2: {
    attack: 12,
    defense: 10,
    health: 35,
    max_health: 35,
    ability: 'None',
  },
  result: null,
  isSimulating: false,
};

export const useGameStore = create<GameState>()(
  persist(
    (set, get) => ({
      // Initial state
      config: defaultConfig,
      selectedLeague: 0,
      availableLeagues: defaultLeagues,
      playerUnits: [],
      currentMatch: null,
      matchHistory: [],
      battleSim: defaultBattleSimulation,
      isLoading: false,
      error: null,

      // Actions
      setConfig: (config) => {
        set({ config });
      },

      selectLeague: (leagueId) => {
        set({ selectedLeague: leagueId });
        console.log(`🏆 Selected league: ${defaultLeagues[leagueId]?.name || 'Unknown'}`);
      },

      loadLeagueInfo: async () => {
        set({ isLoading: true, error: null });
        
        try {
          // In a real implementation, this would fetch from the mint's keysets
          // For now, we use the default leagues
          const leagues = defaultLeagues;
          
          set({ 
            availableLeagues: leagues,
            isLoading: false 
          });
          
          console.log('📋 Loaded league information:', leagues.length, 'leagues');
        } catch (error) {
          const errorMessage = error instanceof Error ? error.message : 'Failed to load league info';
          set({ 
            error: errorMessage,
            isLoading: false 
          });
          console.error('❌ Failed to load league info:', errorMessage);
        }
      },

      setPlayerUnits: (units) => {
        set({ playerUnits: units });
        console.log(`⚔️ Updated player units:`, units.length, 'units');
      },

      updateMatch: (match) => {
        set({ currentMatch: match });
        
        // Add to history if completed
        if (match.status === 'completed') {
          const history = get().matchHistory;
          const existingIndex = history.findIndex(m => m.id === match.id);
          
          if (existingIndex >= 0) {
            // Update existing match in history
            const newHistory = [...history];
            newHistory[existingIndex] = match;
            set({ matchHistory: newHistory });
          } else {
            // Add new match to history
            set({ matchHistory: [match, ...history] });
          }
        }
        
        console.log(`🎮 Updated match ${match.id}:`, match.status);
      },

      setBattleSimulation: (simulation) => {
        set({ battleSim: simulation });
      },

      clearError: () => {
        set({ error: null });
      },
    }),
    {
      name: 'mana-game-storage',
      partialize: (state) => ({
        // Persist important state but not temporary UI state
        config: state.config,
        selectedLeague: state.selectedLeague,
        matchHistory: state.matchHistory,
      }),
    }
  )
);

// Helper functions
export const getLeagueDisplayName = (leagueId: number): string => {
  const league = defaultLeagues.find(l => l.id === leagueId);
  if (!league) return `League ${leagueId}`;
  
  const bonuses = [];
  if (league.attack_bonus > 0) bonuses.push(`+${league.attack_bonus} ATK`);
  if (league.defense_bonus > 0) bonuses.push(`+${league.defense_bonus} DEF`);
  if (league.health_bonus > 0) bonuses.push(`+${league.health_bonus} HP`);
  
  return bonuses.length > 0 
    ? `${league.name} (${bonuses.join(', ')})`
    : league.name;
};

export const getAbilityDisplayName = (ability: string): string => {
  switch (ability) {
    case 'None': return 'None';
    case 'Boost': return 'Boost (2x ATK)';
    case 'Shield': return 'Shield (No DMG)';
    case 'Heal': return 'Heal (50% HP)';
    default: return ability;
  }
};

export const getAbilityDescription = (ability: string): string => {
  switch (ability) {
    case 'None': return 'No special ability';
    case 'Boost': return 'Double attack damage this round';
    case 'Shield': return 'Negate all damage this round';
    case 'Heal': return 'Restore 50% max health after combat';
    default: return 'Unknown ability';
  }
};