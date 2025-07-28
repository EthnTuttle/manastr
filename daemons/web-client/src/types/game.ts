// Game types that mirror the WASM/Rust types
export interface Unit {
  attack: number;
  defense: number;
  health: number;
  max_health: number;
  ability: Ability;
}

export type Ability = 'None' | 'Boost' | 'Shield' | 'Heal';

export interface RoundResult {
  round: number;
  player1_unit: Unit;
  player2_unit: Unit;
  damage_dealt: [number, number]; // [damage to unit2, damage to unit1]
  winner: string | null;
}

export interface LeagueModifier {
  id: number;
  name: string;
  attack_bonus: number;
  defense_bonus: number;
  health_bonus: number;
}

// Mock Cashu CDK types (would be imported from @cashu/cdk in real implementation)
export interface Proof {
  secret: string;
  amount: number;
  keysetId: string;
  signature: string;
}

export interface MintQuote {
  quote: string;
  request: string; // Lightning payment request
  paid: boolean;
  expiry: number;
}

export interface ManaTokens {
  proofs: Proof[];
  leagueId: number;
  totalAmount: number;
  battleUnits: Unit[];
}

export interface LootTokens {
  proofs: Proof[];
  lockedToNpub: string;
  matchId: string;
  totalAmount: number;
}

// Game state types
export interface MatchData {
  id: string;
  players: [string, string];
  status: 'waiting' | 'active' | 'completed';
  rounds: RoundResult[];
  current_round: number;
}

export interface GameConfig {
  mintUrl: string;
  nostrRelays: string[];
  gameEngineUrl: string;
}

// UI state types
export interface WalletBalance {
  mana: number;
  loot: number;
}

export interface BattleSimulation {
  unit1: Unit;
  unit2: Unit;
  result: RoundResult | null;
  isSimulating: boolean;
}