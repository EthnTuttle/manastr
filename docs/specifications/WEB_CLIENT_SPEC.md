# Web Client + Cashu-TS Wallet Integration Specification

## Overview
React-based web application at `:8080` with integrated Cashu wallet using `cashu-ts` library. Basic UI with no graphics, focusing on functionality for MVP.

## Technology Stack

### Core Dependencies
```json
{
  "dependencies": {
    "@cashu/cashu-ts": "^1.0.0",
    "nostr-tools": "^2.1.0",
    "react": "^18.2.0",
    "react-dom": "^18.2.0",
    "react-router-dom": "^6.8.0",
    "typescript": "^5.0.0",
    "@types/react": "^18.0.0",
    "@types/react-dom": "^18.0.0",
    "vite": "^5.0.0",
    "@vitejs/plugin-react": "^4.0.0"
  }
}
```

### Project Structure
```
daemons/web-client/
├── src/
│   ├── components/          # React components
│   │   ├── wallet/         # Wallet-related components
│   │   ├── game/           # Game UI components
│   │   └── common/         # Shared components
│   ├── services/           # API and service layers
│   │   ├── cashu.ts        # Cashu wallet integration
│   │   ├── nostr.ts        # Nostr client integration
│   │   └── game.ts         # Game state management
│   ├── types/              # TypeScript type definitions
│   ├── utils/              # Utility functions
│   ├── App.tsx             # Main app component
│   └── main.tsx            # Entry point
├── public/                 # Static assets
├── package.json
├── tsconfig.json
├── vite.config.ts
└── index.html
```

## Cashu Wallet Integration

### Wallet Service Layer
```typescript
// src/services/cashu.ts
import { CashuMint, CashuWallet, MintQuoteState } from '@cashu/cashu-ts';

export class ManaWalletService {
  private mint: CashuMint;
  private wallet: CashuWallet;
  private mintUrl = 'http://localhost:3333';

  constructor() {
    this.mint = new CashuMint(this.mintUrl);
    this.wallet = new CashuWallet(this.mint);
  }

  async initialize(): Promise<void> {
    await this.wallet.loadMint();
    // Load persisted tokens from localStorage
    this.loadStoredTokens();
  }

  // Mana token purchase (5 mana per sat, 5% fee)
  async purchaseMana(satAmount: number, leagueId: number): Promise<ManaTokenResult> {
    try {
      // Request quote from mint
      const mintQuote = await this.wallet.createMintQuote(satAmount);
      
      // For local development, payment is auto-approved
      // In production, this would show Lightning invoice
      console.log('Payment request:', mintQuote.request);
      
      // Check payment status (stubbed Lightning will auto-approve)
      const paidQuote = await this.checkPaymentStatus(mintQuote.quote);
      
      if (paidQuote.state === MintQuoteState.PAID) {
        // Calculate mana amount (5 mana per sat, 5% fee)
        const totalMana = satAmount * 5;
        const playerMana = Math.floor(totalMana * 0.95);
        
        // Mint mana tokens
        const proofs = await this.wallet.mintProofs(playerMana, mintQuote.quote);
        
        // Store tokens with league metadata
        const manaTokens = proofs.map(proof => ({
          ...proof,
          leagueId,
          unitSet: this.generateUnitsFromProof(proof, leagueId)
        }));
        
        this.storeTokens(manaTokens);
        
        return {
          success: true,
          tokens: manaTokens,
          playerManaReceived: playerMana,
          feeCollected: totalMana - playerMana
        };
      } else {
        throw new Error('Payment not confirmed');
      }
    } catch (error) {
      return {
        success: false,
        error: error.message
      };
    }
  }

  // Generate 8 units from mana token proof
  private generateUnitsFromProof(proof: any, leagueId: number): Unit[] {
    // Use proof secret as deterministic seed
    const secretBytes = this.hexToBytes(proof.secret);
    const units: Unit[] = [];
    
    for (let i = 0; i < 8; i++) {
      const offset = i * 4;
      if (offset + 3 < secretBytes.length) {
        units.push({
          attack: secretBytes[offset],
          defense: secretBytes[offset + 1], 
          health: secretBytes[offset + 2],
          maxHealth: secretBytes[offset + 2],
          ability: this.mapToAbility(secretBytes[offset + 3] % 4),
          leagueModifiers: this.getLeagueModifiers(leagueId)
        });
      }
    }
    
    return units;
  }

  // Loot token claiming
  async claimLoot(lootToken: LootToken): Promise<ClaimResult> {
    try {
      // Verify token is locked to our npub
      if (lootToken.lockedToNpub !== this.getCurrentPlayerNpub()) {
        throw new Error('Loot token not locked to this player');
      }
      
      // Process claim via mint API
      const result = await this.mint.meltTokens([lootToken.proof], lootToken.amount);
      
      if (result.success) {
        // Remove from stored loot tokens
        this.removeLootToken(lootToken);
        
        return {
          success: true,
          satsClaimed: lootToken.amount
        };
      } else {
        throw new Error('Failed to claim loot token');
      }
    } catch (error) {
      return {
        success: false,
        error: error.message
      };
    }
  }

  // Get current balances
  getBalances(): WalletBalance {
    const storedTokens = this.getStoredTokens();
    
    return {
      manaTokens: storedTokens.mana.length,
      lootTokens: storedTokens.loot.length,
      totalLootValue: storedTokens.loot.reduce((sum, token) => sum + token.amount, 0)
    };
  }

  private loadStoredTokens(): void {
    const stored = localStorage.getItem('mana-wallet-tokens');
    if (stored) {
      this.storedTokens = JSON.parse(stored);
    }
  }

  private storeTokens(tokens: ManaToken[]): void {
    const current = this.getStoredTokens();
    current.mana.push(...tokens);
    localStorage.setItem('mana-wallet-tokens', JSON.stringify(current));
  }
}
```

### Wallet React Hook
```typescript
// src/hooks/useWallet.ts
import { useState, useEffect } from 'react';
import { ManaWalletService } from '../services/cashu';

export function useWallet() {
  const [wallet] = useState(() => new ManaWalletService());
  const [balance, setBalance] = useState<WalletBalance>({ manaTokens: 0, lootTokens: 0, totalLootValue: 0 });
  const [isLoading, setIsLoading] = useState(true);

  useEffect(() => {
    const initWallet = async () => {
      try {
        await wallet.initialize();
        setBalance(wallet.getBalances());
      } catch (error) {
        console.error('Failed to initialize wallet:', error);
      } finally {
        setIsLoading(false);
      }
    };

    initWallet();
  }, [wallet]);

  const purchaseMana = async (satAmount: number, leagueId: number) => {
    setIsLoading(true);
    try {
      const result = await wallet.purchaseMana(satAmount, leagueId);
      if (result.success) {
        setBalance(wallet.getBalances());
      }
      return result;
    } finally {
      setIsLoading(false);
    }
  };

  const claimLoot = async (lootToken: LootToken) => {
    setIsLoading(true);
    try {
      const result = await wallet.claimLoot(lootToken);
      if (result.success) {
        setBalance(wallet.getBalances());
      }
      return result;
    } finally {
      setIsLoading(false);
    }
  };

  return {
    balance,
    isLoading,
    purchaseMana,
    claimLoot,
    getAvailableTokens: () => wallet.getStoredTokens()
  };
}
```

## Nostr Integration

### Nostr Service Layer
```typescript
// src/services/nostr.ts
import { SimplePool, Event, getEventHash, getSignature, getPublicKey } from 'nostr-tools';

export class NostrGameService {
  private pool: SimplePool;
  private relay = 'ws://localhost:7777';
  private privateKey: string;
  private publicKey: string;

  constructor() {
    this.pool = new SimplePool();
    this.initializeKeys();
  }

  private initializeKeys(): void {
    // Get or generate Nostr key pair
    let storedKey = localStorage.getItem('nostr-private-key');
    if (!storedKey) {
      storedKey = this.generatePrivateKey();
      localStorage.setItem('nostr-private-key', storedKey);
    }
    
    this.privateKey = storedKey;
    this.publicKey = getPublicKey(storedKey);
  }

  async publishChallenge(challengedNpub: string, matchId: string): Promise<string> {
    const event = {
      kind: 1,
      pubkey: this.publicKey,
      created_at: Math.floor(Date.now() / 1000),
      tags: [
        ['match_id', matchId],
        ['challenged', challengedNpub],
        ['game', 'mana-strategy'],
        ['challenge']
      ],
      content: 'Challenge issued',
    };

    event.id = getEventHash(event);
    event.sig = getSignature(event, this.privateKey);

    const pub = this.pool.publish([this.relay], event);
    await pub;
    
    return event.id;
  }

  async publishCommitment(matchId: string, round: number, commitmentHash: string): Promise<string> {
    const event = {
      kind: 1,
      pubkey: this.publicKey,
      created_at: Math.floor(Date.now() / 1000),
      tags: [
        ['match_id', matchId],
        ['round', round.toString()],
        ['game', 'mana-strategy'],
        ['commitment']
      ],
      content: commitmentHash,
    };

    event.id = getEventHash(event);
    event.sig = getSignature(event, this.privateKey);

    const pub = this.pool.publish([this.relay], event);
    await pub;
    
    return event.id;
  }

  async publishReveal(
    matchId: string, 
    round: number, 
    manaToken: ManaToken, 
    unitIndex: number,
    commitmentEventId: string
  ): Promise<string> {
    const event = {
      kind: 1,
      pubkey: this.publicKey,
      created_at: Math.floor(Date.now() / 1000),
      tags: [
        ['match_id', matchId],
        ['round', round.toString()],
        ['game', 'mana-strategy'],
        ['reveal'],
        ['e', commitmentEventId]
      ],
      content: JSON.stringify({
        mana_token_secret: manaToken.proof.secret,
        mana_token_signature: manaToken.proof.C,
        unit_index: unitIndex,
        round: round,
        league_id: manaToken.leagueId
      }),
    };

    event.id = getEventHash(event);
    event.sig = getSignature(event, this.privateKey);

    const pub = this.pool.publish([this.relay], event);
    await pub;
    
    return event.id;
  }

  subscribeToMatch(matchId: string, onEvent: (event: Event) => void): () => void {
    const sub = this.pool.sub([this.relay], [{
      kinds: [1],
      '#match_id': [matchId],
      '#game': ['mana-strategy']
    }]);

    sub.on('event', onEvent);
    
    return () => sub.unsub();
  }

  getCurrentPlayerNpub(): string {
    return this.publicKey;
  }
}
```

## React Component Architecture

### Main App Component
```typescript
// src/App.tsx
import React from 'react';
import { BrowserRouter as Router, Routes, Route } from 'react-router-dom';
import { WalletProvider } from './contexts/WalletContext';
import { NostrProvider } from './contexts/NostrContext';
import { Header } from './components/common/Header';
import { HomePage } from './pages/HomePage';
import { WalletPage } from './pages/WalletPage';
import { GameLobby } from './pages/GameLobby';
import { MatchPage } from './pages/MatchPage';

function App() {
  return (
    <WalletProvider>
      <NostrProvider>
        <Router>
          <div className="app">
            <Header />
            <main>
              <Routes>
                <Route path="/" element={<HomePage />} />
                <Route path="/wallet" element={<WalletPage />} />
                <Route path="/lobby" element={<GameLobby />} />
                <Route path="/match/:matchId" element={<MatchPage />} />
              </Routes>
            </main>
            <footer>
              <p>Mana Strategy Game - Local Development</p>
            </footer>
          </div>
        </Router>
      </NostrProvider>
    </WalletProvider>
  );
}

export default App;
```

### Wallet Components
```typescript
// src/components/wallet/WalletBalance.tsx
import React from 'react';
import { useWallet } from '../../hooks/useWallet';

export function WalletBalance() {
  const { balance, isLoading } = useWallet();

  if (isLoading) {
    return <div>Loading wallet...</div>;
  }

  return (
    <div className="wallet-balance">
      <h3>Wallet Balance</h3>
      <div className="balance-item">
        <span>Mana Tokens:</span>
        <span>{balance.manaTokens}</span>
      </div>
      <div className="balance-item">
        <span>Loot Tokens:</span>
        <span>{balance.lootTokens}</span>
      </div>
      <div className="balance-item">
        <span>Total Loot Value:</span>
        <span>{balance.totalLootValue} sats</span>
      </div>
    </div>
  );
}

// src/components/wallet/ManaPurchase.tsx
import React, { useState } from 'react';
import { useWallet } from '../../hooks/useWallet';

export function ManaGift() {
  const { purchaseMana } = useWallet();
  const [satAmount, setSatAmount] = useState(1000);
  const [leagueId, setLeagueId] = useState(0);
  const [isPurchasing, setIsPurchasing] = useState(false);

  const handlePurchase = async () => {
    setIsPurchasing(true);
    try {
      const result = await purchaseMana(satAmount, leagueId);
      if (result.success) {
        alert(`Successfully purchased ${result.playerManaReceived} mana tokens!`);
      } else {
        alert(`Purchase failed: ${result.error}`);
      }
    } finally {
      setIsPurchasing(false);
    }
  };

  const expectedMana = Math.floor(satAmount * 5 * 0.95);
  const fee = satAmount * 5 - expectedMana;

  return (
    <div className="mana-purchase">
      <h3>Purchase Mana</h3>
      <div className="purchase-form">
        <div className="form-group">
          <label>Amount (sats):</label>
          <input
            type="number"
            value={satAmount}
            onChange={(e) => setSatAmount(parseInt(e.target.value))}
            min="100"
            step="100"
          />
        </div>
        <div className="form-group">
          <label>League:</label>
          <select value={leagueId} onChange={(e) => setLeagueId(parseInt(e.target.value))}>
            <option value={0}>Fire League (+10 attack)</option>
            <option value={1}>Ice League (+20 health)</option>
            <option value={2}>Shadow League (+5 attack, +5 defense)</option>
            <option value={3}>Nature League (+15 health, +5 defense)</option>
            {/* Add more leagues */}
          </select>
        </div>
        <div className="purchase-preview">
          <p>You will receive: {expectedMana} mana tokens</p>
          <p>Fee (5%): {fee} mana</p>
        </div>
        <button 
          onClick={handlePurchase} 
          disabled={isPurchasing}
          className="purchase-button"
        >
          {isPurchasing ? 'Processing...' : 'Purchase Mana'}
        </button>
      </div>
    </div>
  );
}
```

### Game Components
```typescript
// src/components/game/PlayerLobby.tsx
import React, { useState, useEffect } from 'react';
import { useNostr } from '../../hooks/useNostr';

export function PlayerLobby() {
  const { publishChallenge, subscribeToLobby } = useNostr();
  const [availablePlayers, setAvailablePlayers] = useState<Player[]>([]);
  const [challengeTarget, setChallengeTarget] = useState('');

  const handleChallenge = async () => {
    if (!challengeTarget) return;
    
    const matchId = crypto.randomUUID();
    try {
      await publishChallenge(challengeTarget, matchId);
      alert('Challenge sent!');
      // Redirect to match page
      window.location.href = `/match/${matchId}`;
    } catch (error) {
      alert('Failed to send challenge');
    }
  };

  return (
    <div className="player-lobby">
      <h2>Game Lobby</h2>
      
      <div className="challenge-section">
        <h3>Challenge a Player</h3>
        <div className="challenge-form">
          <input
            type="text"
            placeholder="Enter player npub..."
            value={challengeTarget}
            onChange={(e) => setChallengeTarget(e.target.value)}
          />
          <button onClick={handleChallenge}>Send Challenge</button>
        </div>
      </div>

      <div className="self-play-section">
        <h3>Self-Play Testing</h3>
        <p>Open another browser tab to challenge yourself for testing.</p>
        <button onClick={() => window.open('/', '_blank')}>
          Open Second Player Tab
        </button>
      </div>
    </div>
  );
}

// src/components/game/MatchView.tsx
import React, { useState, useEffect } from 'react';
import { useParams } from 'react-router-dom';
import { useNostr } from '../../hooks/useNostr';
import { useWallet } from '../../hooks/useWallet';

export function MatchView() {
  const { matchId } = useParams<{ matchId: string }>();
  const { subscribeToMatch, publishCommitment, publishReveal } = useNostr();
  const { getAvailableTokens } = useWallet();
  
  const [matchState, setMatchState] = useState<MatchState | null>(null);
  const [selectedToken, setSelectedToken] = useState<ManaToken | null>(null);
  const [selectedUnitIndex, setSelectedUnitIndex] = useState<number | null>(null);
  const [commitmentMade, setCommitmentMade] = useState(false);

  useEffect(() => {
    if (!matchId) return;

    const unsubscribe = subscribeToMatch(matchId, (event) => {
      // Process match events and update state
      updateMatchState(event);
    });

    return unsubscribe;
  }, [matchId]);

  const handleCommitment = async () => {
    if (!selectedToken || selectedUnitIndex === null) return;

    // Calculate commitment hash
    const commitmentHash = await calculateCommitmentHash(
      selectedToken.proof.secret,
      selectedToken.proof.C,
      selectedUnitIndex,
      matchState.currentRound,
      matchId
    );

    try {
      await publishCommitment(matchId, matchState.currentRound, commitmentHash);
      setCommitmentMade(true);
    } catch (error) {
      alert('Failed to submit commitment');
    }
  };

  const handleReveal = async () => {
    if (!selectedToken || selectedUnitIndex === null || !matchState?.commitmentEventId) return;

    try {
      await publishReveal(
        matchId,
        matchState.currentRound,
        selectedToken,
        selectedUnitIndex,
        matchState.commitmentEventId
      );
    } catch (error) {
      alert('Failed to submit reveal');
    }
  };

  if (!matchState) {
    return <div>Loading match...</div>;
  }

  return (
    <div className="match-view">
      <h2>Match: {matchId}</h2>
      <div className="match-info">
        <p>Round: {matchState.currentRound}/5</p>
        <p>Phase: {matchState.phase}</p>
      </div>

      {matchState.phase === 'commitment' && !commitmentMade && (
        <div className="commitment-phase">
          <h3>Select Your Unit</h3>
          <TokenSelector
            tokens={getAvailableTokens().mana}
            onTokenSelect={setSelectedToken}
            onUnitSelect={setSelectedUnitIndex}
          />
          <button onClick={handleCommitment} disabled={!selectedToken || selectedUnitIndex === null}>
            Commit Unit
          </button>
        </div>
      )}

      {matchState.phase === 'reveal' && commitmentMade && (
        <div className="reveal-phase">
          <h3>Waiting for all commitments...</h3>
          <button onClick={handleReveal}>
            Reveal Unit
          </button>
        </div>
      )}

      {matchState.phase === 'combat' && (
        <div className="combat-phase">
          <h3>Combat in Progress...</h3>
          {matchState.lastRoundResult && (
            <RoundResult result={matchState.lastRoundResult} />
          )}
        </div>
      )}
    </div>
  );
}
```

## Basic Styling (CSS)

### Main Stylesheet
```css
/* src/styles/main.css */
* {
  margin: 0;
  padding: 0;
  box-sizing: border-box;
}

body {
  font-family: 'Arial', sans-serif;
  background-color: #f5f5f5;
  color: #333;
}

.app {
  min-height: 100vh;
  display: flex;
  flex-direction: column;
}

header {
  background: #2c3e50;
  color: white;
  padding: 1rem;
}

nav ul {
  list-style: none;
  display: flex;
  gap: 1rem;
}

nav a {
  color: white;
  text-decoration: none;
}

nav a:hover {
  text-decoration: underline;
}

main {
  flex: 1;
  padding: 2rem;
  max-width: 1200px;
  margin: 0 auto;
}

.wallet-balance {
  background: white;
  padding: 1.5rem;
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0,0,0,0.1);
  margin-bottom: 2rem;
}

.balance-item {
  display: flex;
  justify-content: space-between;
  margin-bottom: 0.5rem;
  padding: 0.5rem 0;
  border-bottom: 1px solid #eee;
}

.mana-purchase {
  background: white;
  padding: 1.5rem;
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

.form-group {
  margin-bottom: 1rem;
}

.form-group label {
  display: block;
  margin-bottom: 0.5rem;
  font-weight: bold;
}

.form-group input,
.form-group select {
  width: 100%;
  padding: 0.5rem;
  border: 1px solid #ddd;
  border-radius: 4px;
}

.purchase-preview {
  background: #f8f9fa;
  padding: 1rem;
  border-radius: 4px;
  margin: 1rem 0;
}

.purchase-button {
  background: #27ae60;
  color: white;
  padding: 0.75rem 1.5rem;
  border: none;
  border-radius: 4px;
  cursor: pointer;
  font-size: 1rem;
}

.purchase-button:hover {
  background: #2ecc71;
}

.purchase-button:disabled {
  background: #95a5a6;
  cursor: not-allowed;
}

.match-view {
  background: white;
  padding: 2rem;
  border-radius: 8px;
  box-shadow: 0 2px 4px rgba(0,0,0,0.1);
}

.match-info {
  background: #ecf0f1;
  padding: 1rem;
  border-radius: 4px;
  margin-bottom: 2rem;
}

.unit-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 1rem;
  margin: 1rem 0;
}

.unit-card {
  border: 2px solid #ddd;
  padding: 1rem;
  border-radius: 8px;
  cursor: pointer;
  transition: border-color 0.2s;
}

.unit-card:hover {
  border-color: #3498db;
}

.unit-card.selected {
  border-color: #e74c3c;
  background: #fdf2f2;
}

footer {
  background: #34495e;
  color: white;
  text-align: center;
  padding: 1rem;
}
```

## Configuration Files

### Vite Configuration
```typescript
// vite.config.ts
import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

export default defineConfig({
  plugins: [react()],
  server: {
    port: 8080,
    host: '127.0.0.1'
  },
  define: {
    // For development environment
    'process.env.NODE_ENV': JSON.stringify(process.env.NODE_ENV || 'development'),
    'process.env.MINT_URL': JSON.stringify('http://localhost:3333'),
    'process.env.NOSTR_RELAY': JSON.stringify('ws://localhost:7777')
  }
});
```

### Package.json Scripts
```json
{
  "scripts": {
    "dev": "vite",
    "build": "tsc && vite build",
    "preview": "vite preview",
    "test": "vitest",
    "lint": "eslint src --ext ts,tsx"
  }
}
```

## Development Workflow

### Local Development
```bash
cd daemons/web-client
npm install
npm run dev
# App available at http://localhost:8080
```

### Self-Play Testing
1. Open http://localhost:8080 in first browser tab
2. Generate/load Nostr key pair
3. Purchase mana tokens for testing
4. Open second browser tab (different profile/incognito)
5. Generate different Nostr key pair
6. Challenge first player
7. Complete full match workflow

This specification provides everything needed to implement D4 (Web Client daemon) with integrated Cashu wallet and basic UI for MVP testing.