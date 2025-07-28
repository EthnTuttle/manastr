import type { Unit, RoundResult } from '@/types/game';

// WASM module interface
export interface GameLogicWasm {
  wasm_generate_units_from_token_secret: (tokenSecret: string, leagueId: number) => any;
  wasm_process_combat: (unit1: any, unit2: any, player1Npub: string, player2Npub: string) => any;
  wasm_apply_league_modifiers: (baseUnit: any, leagueId: number) => any;
  wasm_test_connection: () => string;
}

let wasmModule: GameLogicWasm | null = null;
let initPromise: Promise<GameLogicWasm> | null = null;

export async function loadGameLogic(): Promise<GameLogicWasm> {
  if (wasmModule) return wasmModule;
  
  // Prevent multiple concurrent initializations
  if (initPromise) return initPromise;
  
  initPromise = initializeWasm();
  return initPromise;
}

async function initializeWasm(): Promise<GameLogicWasm> {
  try {
    console.log('🔄 Loading WASM game logic module...');
    
    // Dynamic import of the WASM module
    const wasm = await import('@manastr/shared-game-logic');
    
    // Initialize the WASM module
    await wasm.default();
    
    wasmModule = {
      wasm_generate_units_from_token_secret: wasm.wasm_generate_units_from_token_secret,
      wasm_process_combat: wasm.wasm_process_combat,
      wasm_apply_league_modifiers: wasm.wasm_apply_league_modifiers,
      wasm_test_connection: wasm.wasm_test_connection,
    };
    
    // Test the connection
    const testResult = wasmModule.wasm_test_connection();
    console.log('✅ WASM game logic loaded successfully:', testResult);
    
    return wasmModule;
    
  } catch (error) {
    console.error('❌ Failed to load WASM game logic:', error);
    throw new Error(`WASM module initialization failed: ${error}`);
  }
}

// Typed wrapper functions for easier usage
export class GameLogic {
  private static wasm: GameLogicWasm | null = null;
  
  static async initialize(): Promise<void> {
    this.wasm = await loadGameLogic();
  }
  
  static async generateUnits(tokenSecret: string, leagueId: number): Promise<Unit[]> {
    if (!this.wasm) {
      this.wasm = await loadGameLogic();
    }
    
    try {
      const result = this.wasm.wasm_generate_units_from_token_secret(tokenSecret, leagueId);
      
      // Parse the WASM result (it comes as a serialized JSON string)
      const units = JSON.parse(result) as Unit[];
      
      return units;
    } catch (error) {
      console.error('Failed to generate units:', error);
      throw new Error(`Unit generation failed: ${error}`);
    }
  }
  
  static async processCombat(
    unit1: Unit, 
    unit2: Unit, 
    player1Npub: string, 
    player2Npub: string
  ): Promise<RoundResult> {
    if (!this.wasm) {
      this.wasm = await loadGameLogic();
    }
    
    try {
      const result = this.wasm.wasm_process_combat(
        JSON.stringify(unit1),
        JSON.stringify(unit2),
        player1Npub,
        player2Npub
      );
      
      // Parse the WASM result
      const roundResult = JSON.parse(result) as RoundResult;
      
      return roundResult;
    } catch (error) {
      console.error('Failed to process combat:', error);
      throw new Error(`Combat processing failed: ${error}`);
    }
  }
  
  static async applyLeagueModifiers(baseUnit: Unit, leagueId: number): Promise<Unit> {
    if (!this.wasm) {
      this.wasm = await loadGameLogic();
    }
    
    try {
      const result = this.wasm.wasm_apply_league_modifiers(
        JSON.stringify(baseUnit),
        leagueId
      );
      
      // Parse the WASM result
      const modifiedUnit = JSON.parse(result) as Unit;
      
      return modifiedUnit;
    } catch (error) {
      console.error('Failed to apply league modifiers:', error);
      throw new Error(`League modifier application failed: ${error}`);
    }
  }
  
  static async testConnection(): Promise<string> {
    if (!this.wasm) {
      this.wasm = await loadGameLogic();
    }
    
    return this.wasm.wasm_test_connection();
  }
}

// Pre-load the WASM module for faster access
export const preloadGameLogic = () => {
  loadGameLogic().catch(console.error);
};

// Auto-initialize when module is imported
preloadGameLogic();