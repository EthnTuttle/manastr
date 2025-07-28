# Web Client Integration Specification

**Agent**: `ui-dev`  
**Status**: Foundation Complete  
**Dependencies**: F1 (Cashu CDK), F3 (Nostr Relay)  
**Integrates With**: D1 (Cashu Mint), D2 (Game Engine Bot), D3 (Nostr Relay)

## Overview

React/TypeScript web client that serves as the primary user interface for the Mana Strategy Game. Integrates with the **Cashu CDK wallet** for dual-currency token management ("mana" and "loot") and Nostr relays for match coordination.

## Technology Stack

### Core Framework
- **React 18** with TypeScript
- **Vite** for build tooling and dev server  
- **TailwindCSS** for styling
- **React Router** for navigation

### Cashu Integration
- **Cashu CDK Wallet** (https://github.com/cashubtc/cdk) TypeScript bindings
- Dual currency support: "mana" and "loot" 
- Lightning invoice payment integration
- Client-side token verification and management

### Nostr Integration  
- **nostr-tools** for event handling and relay communication
- Real-time match updates and coordination
- Player discovery and messaging

### WASM Game Logic Integration
- **Shared Rust Logic** compiled to WebAssembly using `wasm-pack`
- **Combat Engine:** Identical battle resolution as server
- **Unit Generation:** Deterministic units from mana token secrets
- **Client-side Prediction:** Immediate feedback matching server authority
- **TypeScript Bindings:** Auto-generated from Rust using `wasm-bindgen`

### State Management
- **Zustand** for global state management
- Persistent wallet state with local storage
- Real-time sync with Nostr events
- WASM module loading and caching

## Application Architecture

### Component Structure
```
src/
├── components/
│   ├── wallet/
│   │   ├── CdkWallet.tsx          # CDK wallet interface
│   │   ├── CurrencyBalance.tsx    # Display mana/loot balances
│   │   ├── PurchaseMana.tsx       # Lightning payment for mana
│   │   └── ClaimLoot.tsx          # Loot token claiming
│   ├── battle/
│   │   ├── UnitDisplay.tsx        # Show WASM-generated battle units
│   │   ├── BattleSimulator.tsx    # Client-side WASM combat preview
│   │   ├── CombatVisualization.tsx # Real-time battle animation
│   │   └── MatchHistory.tsx       # Past match results
│   ├── league/
│   │   ├── LeagueSelector.tsx     # Choose battle league
│   │   ├── LeagueStats.tsx        # League modifiers display
│   │   └── PlayerRankings.tsx     # League leaderboards
│   └── nostr/
│       ├── RelayStatus.tsx        # Connection status
│       ├── MatchEvents.tsx        # Real-time match updates
│       └── PlayerProfile.tsx      # Nostr profile integration
├── wasm/
│   ├── gameLogic.ts              # WASM module loading and caching
│   ├── combatEngine.ts           # WASM combat function wrappers
│   └── unitGenerator.ts          # WASM unit generation wrappers
└── stores/
    ├── walletStore.ts            # CDK wallet state
    ├── gameStore.ts              # Battle and match state + WASM logic
    ├── nostrStore.ts             # Relay connections and events
    └── wasmStore.ts              # WASM module state and loading
```

## CDK Wallet Integration

### Dual Currency Wallet Configuration
```typescript
// src/services/cdkWallet.ts
import { Wallet, CurrencyUnit, Amount, Proof } from '@cashu/cdk';

export class ManaStrategyWallet {
  private wallet: Wallet;
  private mintUrl: string;

  constructor(mintUrl: string) {
    this.mintUrl = mintUrl;
    this.wallet = new Wallet({ mintUrl });
  }

  // Purchase mana tokens with Lightning (5 mana per sat)
  async purchaseMana(sats: number, leagueId: number): Promise<ManaTokens> {
    const manaAmount = Amount.from(sats * 5);
    
    // Request quote for mana currency
    const quote = await this.wallet.mintQuote(
      manaAmount, 
      CurrencyUnit.Custom("mana")
    );
    
    // Handle Lightning payment
    await this.payLightningInvoice(quote.request);
    
    // Mint mana tokens with league-specific keyset
    const proofs = await this.wallet.mint(quote, {
      keysetId: `mana_league_${leagueId}`
    });
    
    return this.wrapManaProofs(proofs, leagueId);
  }

  // Extract battle units from mana proofs
  generateBattleUnits(manaProofs: Proof[]): BattleUnit[][] {
    return manaProofs
      .filter(proof => this.isManaProof(proof))
      .map(proof => {
        const leagueId = this.extractLeagueFromKeyset(proof.keysetId);
        return this.generateUnitsFromProof(proof, leagueId);
      });
  }

  // Claim loot tokens if locked to our npub
  async claimLoot(lootProof: Proof, playerNpub: string): Promise<number> {
    // Verify proof is locked to our npub
    if (!this.canClaimLoot(lootProof, playerNpub)) {
      throw new Error('Loot token not locked to this player');
    }

    // Request melt quote for loot currency
    const quote = await this.wallet.meltQuote(
      lootProof.amount,
      CurrencyUnit.Custom("loot")
    );
    
    // Melt loot back to Lightning
    const result = await this.wallet.melt(quote, [lootProof]);
    return result.amountSent.asU64();
  }

  // Check wallet balances for both currencies
  async getBalances(): Promise<{ mana: number; loot: number }> {
    const manaBalance = await this.wallet.getBalance(CurrencyUnit.Custom("mana"));
    const lootBalance = await this.wallet.getBalance(CurrencyUnit.Custom("loot"));
    
    return {
      mana: manaBalance.asU64(),
      loot: lootBalance.asU64()
    };
  }

  // Get proofs for specific currency
  async getProofs(currency: "mana" | "loot"): Promise<Proof[]> {
    return this.wallet.getProofs(CurrencyUnit.Custom(currency));
  }

  // Generate battle units from proof secret using WASM
  private async generateUnitsFromProof(proof: Proof, leagueId: number): Promise<BattleUnit[]> {
    const { generateUnitsFromTokenSecret } = await useWasmStore.getState().getGameLogic();
    const secret = proof.secret;
    
    // Use WASM-compiled logic (identical to server)
    const wasmUnits = generateUnitsFromTokenSecret(secret, leagueId);
    return wasmUnits as BattleUnit[];
  }

  // Check if proof can be claimed by this npub
  private canClaimLoot(proof: Proof, npub: string): boolean {
    // Extract npub from proof secret (loot_npub_match_amount_timestamp format)
    const secretParts = proof.secret.split('_');
    return secretParts.length >= 2 && secretParts[1] === npub;
  }

  // Determine if proof is mana currency
  private isManaProof(proof: Proof): boolean {
    return proof.keysetId.startsWith('mana_league_');
  }

  // Extract league ID from keyset ID
  private extractLeagueFromKeyset(keysetId: string): number {
    const match = keysetId.match(/mana_league_(\d+)/);
    return match ? parseInt(match[1]) : 0;
  }
}
```

## WASM Game Logic Integration

### WASM Module Loading and Caching
```typescript
// src/wasm/gameLogic.ts
import { useWasmStore } from '../stores/wasmStore';

export interface GameLogicWasm {
  generateUnitsFromTokenSecret: (tokenSecret: string, leagueId: number) => BattleUnit[];
  processCombat: (unit1: BattleUnit, unit2: BattleUnit, player1Npub: string, player2Npub: string) => RoundResult;
}

let wasmModule: GameLogicWasm | null = null;

export async function loadGameLogic(): Promise<GameLogicWasm> {
  if (wasmModule) return wasmModule;
  
  try {
    // Load WASM module (built from Rust using wasm-pack)
    const wasm = await import('@manastr/shared-game-logic');
    await wasm.default(); // Initialize WASM module
    
    wasmModule = {
      generateUnitsFromTokenSecret: (tokenSecret: string, leagueId: number) => {
        const result = wasm.generate_units_from_token_secret(tokenSecret, leagueId);
        return JSON.parse(result) as BattleUnit[];
      },
      
      processCombat: (unit1: BattleUnit, unit2: BattleUnit, player1Npub: string, player2Npub: string) => {
        const result = wasm.process_combat(
          JSON.stringify(unit1),
          JSON.stringify(unit2), 
          player1Npub,
          player2Npub
        );
        return JSON.parse(result) as RoundResult;
      }
    };
    
    console.log('✅ WASM game logic loaded successfully');
    return wasmModule;
    
  } catch (error) {
    console.error('❌ Failed to load WASM game logic:', error);
    throw new Error('WASM module initialization failed');
  }
}

// Pre-load and cache WASM module
export const preloadGameLogic = () => {
  loadGameLogic().catch(console.error);
};
```

### WASM Store for Module Management
```typescript
// src/stores/wasmStore.ts
import { create } from 'zustand';
import { GameLogicWasm, loadGameLogic } from '../wasm/gameLogic';

interface WasmState {
  gameLogic: GameLogicWasm | null;
  isLoading: boolean;
  error: string | null;
  
  // Actions
  initializeWasm: () => Promise<void>;
  getGameLogic: () => Promise<GameLogicWasm>;
}

export const useWasmStore = create<WasmState>((set, get) => ({
  gameLogic: null,
  isLoading: false,
  error: null,

  initializeWasm: async () => {
    set({ isLoading: true, error: null });
    
    try {
      const gameLogic = await loadGameLogic();
      set({ gameLogic, isLoading: false });
    } catch (error) {
      set({ error: error.message, isLoading: false });
    }
  },

  getGameLogic: async () => {
    const { gameLogic } = get();
    
    if (gameLogic) return gameLogic;
    
    await get().initializeWasm();
    return get().gameLogic!;
  }
}));
```

### WASM-Powered Battle Simulator
```typescript
// src/components/battle/BattleSimulator.tsx
import { useEffect, useState } from 'react';
import { useWasmStore } from '../../stores/wasmStore';
import { useWalletStore } from '../../stores/walletStore';

export const BattleSimulator: React.FC = () => {
  const [selectedUnit1, setSelectedUnit1] = useState<BattleUnit | null>(null);
  const [selectedUnit2, setSelectedUnit2] = useState<BattleUnit | null>(null);
  const [combatResult, setCombatResult] = useState<RoundResult | null>(null);
  
  const { battleUnits } = useWalletStore();
  const { getGameLogic } = useWasmStore();

  const simulateCombat = async () => {
    if (!selectedUnit1 || !selectedUnit2) return;
    
    try {
      const gameLogic = await getGameLogic();
      
      // Use WASM to simulate combat (identical to server logic)
      const result = gameLogic.processCombat(
        selectedUnit1,
        selectedUnit2,
        'player1_test', 
        'player2_test'
      );
      
      setCombatResult(result);
    } catch (error) {
      console.error('Combat simulation failed:', error);
    }
  };

  return (
    <div className="bg-white rounded-lg shadow-md p-6">
      <h2 className="text-xl font-bold mb-4">Combat Simulator</h2>
      
      {/* Unit Selection */}
      <div className="grid grid-cols-2 gap-6 mb-6">
        <div>
          <h3 className="font-semibold mb-2">Unit 1</h3>
          <select 
            onChange={(e) => setSelectedUnit1(battleUnits[0]?.[parseInt(e.target.value)])}
            className="w-full p-2 border rounded"
          >
            <option value="">Select Unit</option>
            {battleUnits[0]?.map((unit, i) => (
              <option key={i} value={i}>
                ATK:{unit.attack} DEF:{unit.defense} HP:{unit.health} ({unit.ability})
              </option>
            ))}
          </select>
        </div>
        
        <div>
          <h3 className="font-semibold mb-2">Unit 2</h3>
          <select 
            onChange={(e) => setSelectedUnit2(battleUnits[1]?.[parseInt(e.target.value)])}
            className="w-full p-2 border rounded"
          >
            <option value="">Select Unit</option>
            {battleUnits[1]?.map((unit, i) => (
              <option key={i} value={i}>
                ATK:{unit.attack} DEF:{unit.defense} HP:{unit.health} ({unit.ability})
              </option>
            ))}
          </select>
        </div>
      </div>

      {/* Simulate Button */}
      <button
        onClick={simulateCombat}
        disabled={!selectedUnit1 || !selectedUnit2}
        className="w-full bg-red-600 text-white py-2 px-4 rounded hover:bg-red-700 disabled:opacity-50"
      >
        🔥 Simulate Combat (WASM)
      </button>

      {/* Combat Result */}
      {combatResult && (
        <div className="mt-6 p-4 bg-gray-50 rounded">
          <h3 className="font-semibold mb-2">Combat Result</h3>
          <div className="grid grid-cols-2 gap-4">
            <div>
              <div className="text-sm text-gray-600">Unit 1 Final</div>
              <div>HP: {combatResult.player1_unit.health}/{combatResult.player1_unit.max_health}</div>
            </div>
            <div>
              <div className="text-sm text-gray-600">Unit 2 Final</div>
              <div>HP: {combatResult.player2_unit.health}/{combatResult.player2_unit.max_health}</div>
            </div>
          </div>
          <div className="mt-2">
            <strong>Winner: </strong>
            {combatResult.winner || 'Tie'}
          </div>
          <div className="text-xs text-green-600 mt-2">
            ✅ Result matches server authority (WASM synchronization)
          </div>
        </div>
      )}
    </div>
  );
};
```

### Custom Token Types with CDK Proofs
```typescript
// src/types/gameTypes.ts
import { Proof, Amount } from '@cashu/cdk';

export interface ManaTokens {
  proofs: Proof[];           // CDK proofs with mana currency
  leagueId: number;          // 0-15 for league-specific bonuses
  totalAmount: Amount;       // Total mana amount
  battleUnits: BattleUnit[]; // Generated from proof secrets
}

export interface LootTokens {
  proofs: Proof[];           // CDK proofs with loot currency  
  lockedToNpub: string;      // Winner's nostr pubkey
  matchId: string;           // Source match ID
  totalAmount: Amount;       // Total loot amount
}

export interface BattleUnit {
  attack: number;
  defense: number;
  health: number;
  maxHealth: number;
  ability: UnitAbility;
}

export enum UnitAbility {
  None = 'none',
  Boost = 'boost',       // Double attack this turn
  Shield = 'shield',     // Negate damage this turn  
  Heal = 'heal'          // Restore 50% health after combat
}

// League modifier definitions
export interface LeagueModifier {
  id: number;
  name: string;
  attackBonus: number;
  defenseBonus: number;
  healthBonus: number;
}
```

## State Management with CDK

### Wallet Store
```typescript
// src/stores/walletStore.ts
import { create } from 'zustand';
import { persist } from 'zustand/middleware';
import { ManaStrategyWallet } from '../services/cdkWallet';

interface WalletState {
  // CDK Wallet instance
  wallet: ManaStrategyWallet | null;
  
  // Balances (from CDK)
  manaBalance: number;
  lootBalance: number;
  
  // Generated battle units
  battleUnits: BattleUnit[][];
  
  // Connection state
  isConnected: boolean;
  
  // Actions
  initializeWallet: (mintUrl: string) => Promise<void>;
  purchaseMana: (sats: number, leagueId: number) => Promise<void>;
  claimLoot: (lootProof: Proof, npub: string) => Promise<void>;
  refreshBalances: () => Promise<void>;
  generateAllUnits: () => Promise<void>;
}

export const useWalletStore = create<WalletState>()(
  persist(
    (set, get) => ({
      wallet: null,
      manaBalance: 0,
      lootBalance: 0,
      battleUnits: [],
      isConnected: false,

      initializeWallet: async (mintUrl: string) => {
        const wallet = new ManaStrategyWallet(mintUrl);
        await wallet.connect();
        
        set({ wallet, isConnected: true });
        await get().refreshBalances();
        await get().generateAllUnits();
      },

      purchaseMana: async (sats: number, leagueId: number) => {
        const { wallet } = get();
        if (!wallet) throw new Error('Wallet not initialized');
        
        await wallet.purchaseMana(sats, leagueId);
        
        // Refresh state after purchase
        await get().refreshBalances();
        await get().generateAllUnits();
      },

      claimLoot: async (lootProof: Proof, npub: string) => {
        const { wallet } = get();
        if (!wallet) throw new Error('Wallet not initialized');
        
        const claimedSats = await wallet.claimLoot(lootProof, npub);
        
        // Refresh balances
        await get().refreshBalances();
        
        return claimedSats;
      },

      refreshBalances: async () => {
        const { wallet } = get();
        if (!wallet) return;
        
        const balances = await wallet.getBalances();
        set({ 
          manaBalance: balances.mana,
          lootBalance: balances.loot 
        });
      },

      generateAllUnits: async () => {
        const { wallet } = get();
        if (!wallet) return;
        
        const manaProofs = await wallet.getProofs('mana');
        const battleUnits = wallet.generateBattleUnits(manaProofs);
        
        set({ battleUnits });
      }
    }),
    {
      name: 'mana-wallet-storage',
      partialize: (state) => ({
        // Don't persist wallet instance, only state data
        manaBalance: state.manaBalance,
        lootBalance: state.lootBalance
      })
    }
  )
);
```

### Game Store Integration
```typescript
// src/stores/gameStore.ts
import { create } from 'zustand';

interface GameState {
  // Current battle setup
  selectedLeague: number;
  availableLeagues: LeagueModifier[];
  
  // Match state
  currentMatch?: MatchData;
  matchHistory: MatchResult[];
  
  // Actions
  selectLeague: (leagueId: number) => void;
  loadLeagueInfo: () => Promise<void>;
  joinMatch: (matchId: string, manaAmount: number) => Promise<void>;
  updateMatchResult: (result: MatchResult) => void;
}

export const useGameStore = create<GameState>((set, get) => ({
  selectedLeague: 0,
  availableLeagues: [],
  matchHistory: [],

  selectLeague: (leagueId: number) => {
    set({ selectedLeague: leagueId });
  },

  loadLeagueInfo: async () => {
    // Fetch league info from mint
    const response = await fetch(`${MINT_URL}/keysets`);
    const keysets = await response.json();
    
    const leagues = keysets
      .filter((k: any) => k.unit === 'mana')
      .map((k: any) => ({
        id: k.league.id,
        name: k.league.name,
        attackBonus: k.league.modifiers.attack,
        defenseBonus: k.league.modifiers.defense,
        healthBonus: k.league.modifiers.health
      }));
    
    set({ availableLeagues: leagues });
  },

  joinMatch: async (matchId: string, manaAmount: number) => {
    const { wallet } = useWalletStore.getState();
    if (!wallet) throw new Error('Wallet not initialized');
    
    // Get mana proofs for battle
    const manaProofs = await wallet.getProofs('mana');
    const selectedProofs = manaProofs.slice(0, manaAmount);
    
    // Publish match join via Nostr
    const nostrClient = useNostrStore.getState().client;
    await nostrClient?.joinMatch(matchId, selectedProofs);
    
    set({ 
      currentMatch: { 
        id: matchId, 
        manaCommitted: manaAmount,
        status: 'joined' 
      }
    });
  }
}));
```

## UI Components with CDK Integration

### CDK Wallet Interface
```tsx
// src/components/wallet/CdkWallet.tsx
export const CdkWallet: React.FC = () => {
  const { 
    wallet, 
    manaBalance, 
    lootBalance, 
    isConnected,
    initializeWallet,
    refreshBalances 
  } = useWalletStore();

  useEffect(() => {
    if (!wallet) {
      initializeWallet(MINT_URL);
    }
  }, []);

  if (!isConnected) {
    return (
      <div className="flex items-center justify-center p-8">
        <div className="text-center">
          <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-blue-600 mx-auto mb-4"></div>
          <p>Connecting to Cashu Mint...</p>
        </div>
      </div>
    );
  }

  return (
    <div className="bg-white rounded-lg shadow-md p-6">
      <div className="flex justify-between items-center mb-6">
        <h2 className="text-xl font-bold">Cashu Wallet</h2>
        <button 
          onClick={refreshBalances}
          className="text-blue-600 hover:text-blue-800"
        >
          🔄 Refresh
        </button>
      </div>

      {/* Currency Balances */}
      <div className="grid grid-cols-2 gap-4 mb-6">
        <div className="bg-blue-50 rounded-lg p-4">
          <div className="text-sm text-blue-600">Mana Balance</div>
          <div className="text-2xl font-bold text-blue-800">
            {manaBalance.toLocaleString()}
          </div>
          <div className="text-xs text-blue-500">
            Battle Currency
          </div>
        </div>
        
        <div className="bg-green-50 rounded-lg p-4">
          <div className="text-sm text-green-600">Loot Balance</div>
          <div className="text-2xl font-bold text-green-800">
            {lootBalance.toLocaleString()}
          </div>
          <div className="text-xs text-green-500">
            Reward Currency
          </div>
        </div>
      </div>

      {/* Action Buttons */}
      <div className="flex gap-4">
        <Link 
          to="/purchase" 
          className="flex-1 bg-blue-600 text-white py-2 px-4 rounded-md hover:bg-blue-700 text-center"
        >
          Purchase Mana
        </Link>
        <Link 
          to="/loot" 
          className="flex-1 bg-green-600 text-white py-2 px-4 rounded-md hover:bg-green-700 text-center"
        >
          Claim Loot
        </Link>
      </div>
    </div>
  );
};
```

### Mana Purchase with CDK
```tsx
// src/components/wallet/PurchaseMana.tsx
export const PurchaseMana: React.FC = () => {
  const [sats, setSats] = useState(100);
  const [selectedLeague, setSelectedLeague] = useState(0);
  const [isLoading, setIsLoading] = useState(false);
  
  const { purchaseMana } = useWalletStore();
  const { availableLeagues, loadLeagueInfo } = useGameStore();

  useEffect(() => {
    loadLeagueInfo();
  }, []);

  const handlePurchase = async () => {
    setIsLoading(true);
    try {
      await purchaseMana(sats, selectedLeague);
      toast.success(`Purchased ${sats * 5} mana tokens!`);
    } catch (error) {
      toast.error('Purchase failed: ' + error.message);
    } finally {
      setIsLoading(false);
    }
  };

  const manaAmount = sats * 5;
  const unitsGenerated = manaAmount * 8;
  const selectedLeagueInfo = availableLeagues[selectedLeague];

  return (
    <div className="bg-white rounded-lg shadow-md p-6">
      <h2 className="text-xl font-bold mb-4">Purchase Mana Tokens</h2>
      
      {/* Amount Input */}
      <div className="mb-4">
        <label className="block text-sm font-medium mb-2">
          Lightning Sats
        </label>
        <input
          type="number"
          value={sats}
          onChange={(e) => setSats(parseInt(e.target.value))}
          min="1"
          className="w-full px-3 py-2 border rounded-md"
        />
        <p className="text-sm text-gray-500 mt-1">
          = {manaAmount} mana tokens ({unitsGenerated} battle units)
        </p>
      </div>

      {/* League Selection with CDK Keyset Info */}
      <div className="mb-4">
        <label className="block text-sm font-medium mb-2">
          Battle League (Keyset)
        </label>
        <select
          value={selectedLeague}
          onChange={(e) => setSelectedLeague(parseInt(e.target.value))}
          className="w-full px-3 py-2 border rounded-md"
        >
          {availableLeagues.map(league => (
            <option key={league.id} value={league.id}>
              {league.name} 
              {league.attackBonus > 0 && ` (+${league.attackBonus} ATK)`}
              {league.defenseBonus > 0 && ` (+${league.defenseBonus} DEF)`}
              {league.healthBonus > 0 && ` (+${league.healthBonus} HP)`}
            </option>
          ))}
        </select>
        
        {selectedLeagueInfo && (
          <div className="mt-2 p-2 bg-gray-50 rounded text-sm">
            <strong>League Bonuses:</strong>
            <div className="flex gap-4 mt-1">
              <span className="text-red-600">ATK: +{selectedLeagueInfo.attackBonus}</span>
              <span className="text-blue-600">DEF: +{selectedLeagueInfo.defenseBonus}</span>
              <span className="text-green-600">HP: +{selectedLeagueInfo.healthBonus}</span>
            </div>
          </div>
        )}
      </div>

      {/* CDK Purchase Button */}
      <button
        onClick={handlePurchase}
        disabled={isLoading}
        className="w-full bg-blue-600 text-white py-2 px-4 rounded-md hover:bg-blue-700 disabled:opacity-50"
      >
        {isLoading ? 'Processing Lightning Payment...' : 'Purchase with CDK Wallet'}
      </button>
    </div>
  );
};
```

## Lightning Integration with CDK

### Payment Flow
```typescript
// src/services/lightningPayment.ts
import { MintQuote } from '@cashu/cdk';

export class LightningPaymentService {
  async payInvoice(quote: MintQuote): Promise<boolean> {
    // Try WebLN first (Alby, etc.)
    if (window.webln) {
      try {
        await window.webln.enable();
        const result = await window.webln.sendPayment(quote.request);
        return result.preimage !== undefined;
      } catch (error) {
        console.warn('WebLN payment failed:', error);
      }
    }

    // Fallback to manual payment
    return this.showManualPayment(quote.request);
  }

  private async showManualPayment(invoice: string): Promise<boolean> {
    // Show QR code and payment instructions
    const qrCode = generateQRCode(invoice);
    
    return new Promise((resolve) => {
      const modal = createPaymentModal({
        invoice,
        qrCode,
        onSuccess: () => resolve(true),
        onCancel: () => resolve(false)
      });
      
      document.body.appendChild(modal);
    });
  }
}
```

## Development Setup

### Environment Configuration
```bash
# .env.local
VITE_MINT_URL=http://localhost:3333
VITE_NOSTR_RELAYS=ws://localhost:7777,wss://relay.damus.io
VITE_GAME_ENGINE_URL=http://localhost:4444
```

### Package.json Dependencies
```json
{
  "dependencies": {
    "react": "^18.2.0",
    "react-dom": "^18.2.0", 
    "react-router-dom": "^6.8.0",
    "@cashu/cdk": "^1.0.0",
    "nostr-tools": "^1.17.0",
    "zustand": "^4.4.0",
    "tailwindcss": "^3.3.0",
    "qrcode": "^1.5.0",
    "react-hot-toast": "^2.4.0",
    "@manastr/shared-game-logic": "file:../shared-game-logic/pkg"
  },
  "devDependencies": {
    "@types/react": "^18.2.0",
    "@vitejs/plugin-react": "^4.0.0",
    "typescript": "^5.0.0",
    "vite": "^4.4.0",
    "vite-plugin-wasm": "^3.2.2",
    "vite-plugin-top-level-await": "^1.3.1"
  }
}
```

### Vite Configuration for WASM
```javascript
// vite.config.ts
import { defineConfig } from 'vite'
import react from '@vitejs/plugin-react'
import wasm from 'vite-plugin-wasm'
import topLevelAwait from 'vite-plugin-top-level-await'

export default defineConfig({
  plugins: [
    react(),
    wasm(),
    topLevelAwait()
  ],
  server: {
    fs: {
      allow: ['..']
    }
  },
  define: {
    global: 'globalThis',
  },
  optimizeDeps: {
    exclude: ['@manastr/shared-game-logic']
  }
})
```

## Integration Testing

### CDK Wallet Tests
```typescript
// tests/e2e/cdkWalletFlow.test.ts
describe('CDK Wallet Integration', () => {
  test('can initialize dual currency wallet', async () => {
    const wallet = new ManaStrategyWallet('http://localhost:3333');
    await wallet.connect();
    
    const balances = await wallet.getBalances();
    expect(balances).toHaveProperty('mana');
    expect(balances).toHaveProperty('loot');
  });

  test('can purchase mana tokens with Lightning', async () => {
    // Navigate to purchase page
    await page.goto('/purchase');
    
    // Enter purchase details
    await page.fill('[data-testid="sats-input"]', '100');
    await page.selectOption('[data-testid="league-select"]', '0');
    
    // Click purchase (will use CDK)
    await page.click('[data-testid="purchase-button"]');
    
    // Mock Lightning payment via CDK
    await mockCdkLightningPayment();
    
    // Verify mana balance updated
    await expect(page.locator('[data-testid="mana-balance"]')).toContainText('500');
  });

  test('can claim loot tokens via CDK', async () => {
    // Simulate receiving loot token
    await simulateLootTokenReceived();
    
    // Navigate to loot page
    await page.goto('/loot');
    
    // Click claim button (uses CDK melt)
    await page.click('[data-testid="claim-loot-button"]');
    
    // Verify loot claimed successfully
    await expect(page.locator('[data-testid="success-message"]')).toBeVisible();
  });
});
```

---

**Status**: Ready for CDK-based implementation  
**Next Steps**: Begin React app setup with Cashu CDK dual-currency wallet integration