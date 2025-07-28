import { create } from 'zustand';
import { GameLogic } from '@/wasm/gameLogic';
import type { Unit, RoundResult } from '@/types/game';

interface WasmState {
  isInitialized: boolean;
  isLoading: boolean;
  error: string | null;
  
  // Actions
  initialize: () => Promise<void>;
  generateUnits: (tokenSecret: string, leagueId: number) => Promise<Unit[]>;
  processCombat: (unit1: Unit, unit2: Unit, player1: string, player2: string) => Promise<RoundResult>;
  applyLeagueModifiers: (unit: Unit, leagueId: number) => Promise<Unit>;
  testConnection: () => Promise<string>;
}

export const useWasmStore = create<WasmState>((set, get) => ({
  isInitialized: false,
  isLoading: false,
  error: null,

  initialize: async () => {
    const { isInitialized, isLoading } = get();
    
    if (isInitialized || isLoading) {
      return;
    }
    
    set({ isLoading: true, error: null });
    
    try {
      await GameLogic.initialize();
      
      // Test the connection to ensure everything works
      const testResult = await GameLogic.testConnection();
      console.log('🎮 WASM store initialized:', testResult);
      
      set({ 
        isInitialized: true, 
        isLoading: false, 
        error: null 
      });
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : 'Unknown error';
      console.error('❌ WASM store initialization failed:', errorMessage);
      
      set({ 
        isInitialized: false, 
        isLoading: false, 
        error: errorMessage 
      });
      
      throw error;
    }
  },

  generateUnits: async (tokenSecret: string, leagueId: number) => {
    const { isInitialized } = get();
    
    if (!isInitialized) {
      await get().initialize();
    }
    
    try {
      const units = await GameLogic.generateUnits(tokenSecret, leagueId);
      console.log(`🎲 Generated ${units.length} units for league ${leagueId}`);
      return units;
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : 'Unit generation failed';
      set({ error: errorMessage });
      throw error;
    }
  },

  processCombat: async (unit1: Unit, unit2: Unit, player1: string, player2: string) => {
    const { isInitialized } = get();
    
    if (!isInitialized) {
      await get().initialize();
    }
    
    try {
      const result = await GameLogic.processCombat(unit1, unit2, player1, player2);
      console.log('⚔️ Processed combat:', result.winner ? `Winner: ${result.winner}` : 'Tie');
      return result;
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : 'Combat processing failed';
      set({ error: errorMessage });
      throw error;
    }
  },

  applyLeagueModifiers: async (unit: Unit, leagueId: number) => {
    const { isInitialized } = get();
    
    if (!isInitialized) {
      await get().initialize();
    }
    
    try {
      const modifiedUnit = await GameLogic.applyLeagueModifiers(unit, leagueId);
      return modifiedUnit;
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : 'League modifier application failed';
      set({ error: errorMessage });
      throw error;
    }
  },

  testConnection: async () => {
    const { isInitialized } = get();
    
    if (!isInitialized) {
      await get().initialize();
    }
    
    try {
      const result = await GameLogic.testConnection();
      return result;
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : 'Connection test failed';
      set({ error: errorMessage });
      throw error;
    }
  }
}));

// Auto-initialize when the store is first accessed
export const initializeWasmStore = () => {
  const store = useWasmStore.getState();
  if (!store.isInitialized && !store.isLoading) {
    store.initialize().catch(console.error);
  }
};