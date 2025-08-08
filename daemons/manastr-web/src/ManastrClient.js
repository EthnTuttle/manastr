// Manastr Client - Handles Nostr, Cashu, and Game Engine interactions
export default class ManastrClient {
    constructor() {
        this.nostr = null;
        this.cashu = null;
        this.wallet = null;
        this.signer = null;
        this.connected = false;
        this.balance = 0;
        this.proofs = [];
        
        // Players (like integration test)
        this.players = {
            alexi: null,
            boberto: null
        };
        
        // Match state tracking
        this.currentMatch = {
            id: null,
            challengeEvent: null,
            acceptanceEvent: null,
            challenger: null,
            acceptor: null,
            wagerAmount: 2,
            leagueId: 1
        };
        
        // Callbacks for UI updates
        this.onStatusUpdate = null;
        this.onLog = null;
        this.onPlayerUpdate = null;
        this.onMatchUpdate = null;
        this.onGameEvent = null;
        
        this.status = {
            nostr: 'Disconnected',
            balance: '0 mana',
            activeGames: '0',
            gameEngine: 'Disconnected'
        };
        
        // Runtime stats for UI counters
        this.stats = {
            nostrEvents: 0,
            matches: 0,
            validations: 0,
        };

        // Mint runtime stats
        this.mintStats = {
            totalTokensMinted: 0,
            lastQuoteState: 'UNPAID',
        };
    }

    async initialize() {
        this.log('🚀 Initializing Manastr quantum client...');
        
        try {
            await this.loadLibraries();
            this.log('✅ Quantum libraries loaded successfully');
            
            // Auto-connect to services
            await this.connectNostr();
            await this.connectMint();
            
            this.log('🔮 Ready for revolutionary gaming operations');
        } catch (error) {
            this.log(`❌ Failed to load quantum libraries: ${error.message}`);
            this.log('📦 Execute: npm install to acquire required dependencies');
        }
    }

    async loadLibraries() {
        try {
            // Import Cashu-TS
            const cashuModule = await import('@cashu/cashu-ts');
            this.log('📦 Cashu-TS quantum protocols loaded');
            
            // Import NDK
            const ndkModule = await import('@nostr-dev-kit/ndk');
            this.log('📦 NDK neural-network initialized');
            console.log('NDK module:', ndkModule); // Debug log
            
            // Store for later use
            this.CashuWallet = cashuModule.CashuWallet;
            this.CashuMint = cashuModule.CashuMint;
            this.NDK = ndkModule.default || ndkModule.NDK;
            this.NDKEvent = ndkModule.NDKEvent;
            this.NDKPrivateKeySigner = ndkModule.NDKPrivateKeySigner;
            
        } catch (error) {
            this.log(`❌ Quantum library loading error: ${error.message}`);
            throw error;
        }
    }

    async connectNostr() {
        if (!this.NDK) {
            this.log('❌ NDK neural-network not loaded');
            return;
        }

        try {
            this.log('🔗 Establishing Nostr quantum entanglement...');
            
            // Generate ephemeral signer for demo
            this.signer = this.NDKPrivateKeySigner.generate();
            
            // Create NDK instance with signer
            this.nostr = new this.NDK({
                explicitRelayUrls: ['ws://localhost:7777'],
                signer: this.signer
            });
            
            // Connect
            await this.nostr.connect();
            
            this.log('✅ Nostr quantum tunnel established');
            this.updateStatus('nostr', 'Connected');
            
            // Get public key from signer
            const user = await this.signer.user();
            const pubkey = user.pubkey;
            
            this.log(`🔑 Quantum keypair generated`);
            this.log(`📋 Public quantum signature: ${pubkey}`);
            
            // Update UI
            const pubkeyElement = document.getElementById('nostr-pubkey');
            if (pubkeyElement) {
                pubkeyElement.textContent = pubkey.substring(0, 32) + '...';
            }
            
            // Subscribe to game events for live feed
            await this.subscribeToGameEvents();
            
            // Update game engine status since it operates via Nostr
            this.log('🎮 Game engine accessible via Nostr protocol');
            this.updateStatus('gameEngine', 'Nostr Ready');
            
            const engineStatusElement = document.getElementById('engine-status');
            if (engineStatusElement) {
                engineStatusElement.textContent = 'Nostr Ready';
            }
            
        } catch (error) {
            this.log(`❌ Nostr quantum entanglement failed: ${error.message}`);
            this.updateStatus('nostr', 'Failed');
        }
    }

    async disconnectNostr() {
        if (this.nostr) {
            this.nostr.disconnect();
            this.nostr = null;
            this.signer = null;
            this.log('📡 Nostr quantum tunnel severed');
            this.updateStatus('nostr', 'Disconnected');
            
            const pubkeyElement = document.getElementById('nostr-pubkey');
            if (pubkeyElement) {
                pubkeyElement.textContent = 'Not connected';
            }
        }
    }

    async subscribeToGameEvents() {
        if (!this.nostr) {
            this.log('❌ Nostr connection required for game events');
            return;
        }

        try {
            this.log('🎮 Subscribing to live game events...');
            
            // Subscribe to game-related events (kinds 31000-31006)
            const gameEventFilter = {
                kinds: [31000, 31001, 31002, 31003, 31004, 31005, 31006],
                limit: 50
            };
            
            const subscription = this.nostr.subscribe(gameEventFilter);
            
            subscription.on('event', (event) => {
                this.handleGameEvent(event);
                if (this.onGameEvent) this.onGameEvent(event);
            });
            
            this.log('📡 Live game event feed activated');
            
        } catch (error) {
            this.log(`❌ Game event subscription failed: ${error.message}`);
        }
    }

    handleGameEvent(event) {
        const eventTypes = {
            31000: '🎯 Match Challenge',
            31001: '🎲 Match Accepted',
            31002: '🔮 Token Reveal',
            31003: '⚔️ Combat Move',
            31004: '🎭 Move Revealed',
            31005: '🏆 Match Result',
            31006: '💰 Loot Distributed'
        };
        
        const eventType = eventTypes[event.kind] || '🎮 Game Event';
        const pubkey = event.pubkey.substring(0, 8);
        const timestamp = new Date(event.created_at * 1000).toLocaleTimeString();
        
        this.log(`📡 LIVE: ${eventType} from ${pubkey}... at ${timestamp}`);
        
        // Show event details if available
        try {
            const content = JSON.parse(event.content);
            if (content.wager_amount) {
                this.log(`   💰 Wager: ${content.wager_amount} mana`);
            }
            if (content.match_event_id || content.match_id) {
                const matchId = content.match_event_id || content.match_id;
                this.log(`   🆔 Match: ${matchId.substring(0, 8)}...`);
            }
            if (content.round_number) {
                this.log(`   🗡️ Round: ${content.round_number}`);
            }
            if (content.calculated_winner) {
                this.log(`   🏆 Winner: ${content.calculated_winner.substring(0, 8)}...`);
            }
            if (content.loot_cashu_token && event.kind === 31006) {
                this.log(`   💎 Loot distributed by Game Engine`);
            }
        } catch (e) {
            // Content might not be JSON, that's OK
        }
        
        // Update counters and service statistics for real-time monitoring
        this.stats.nostrEvents += 1;
        if (event.kind === 31000) {
            this.stats.matches += 1;
        }
        if (event.kind === 31005 || event.kind === 31006) {
            this.stats.validations += 1;
        }
        this.updateServiceStats();
    }

    updateServiceStats() {
        // Update Nostr relay stats and counters
        if (this.onStatusUpdate) {
            this.onStatusUpdate({
                ...this.status,
                nostr: this.nostr ? 'Connected' : 'Disconnected',
                nostrEvents: this.stats.nostrEvents,
                matches: this.stats.matches,
                validations: this.stats.validations,
            });
        }
        const currentTime = new Date().toLocaleTimeString();
        this.log(`📊 Service stats updated at ${currentTime}`);
    }

    async postNote() {
        if (!this.nostr || !this.signer) {
            this.log('❌ Nostr quantum tunnel not established');
            return;
        }

        try {
            const noteContent = `🏛️ MANASTR quantum transmission at ${new Date().toISOString()}\nRevolutionary gaming protocols active.`;
            
            const event = new this.NDKEvent(this.nostr, {
                kind: 1,
                content: noteContent,
                created_at: Math.floor(Date.now() / 1000),
            });
            
            await event.sign(this.signer);
            await event.publish();
            
            this.log(`📝 Quantum note transmitted to Nostr network`);
            this.log(`🆔 Event quantum signature: ${event.id}`);
            
        } catch (error) {
            this.log(`❌ Quantum transmission failed: ${error.message}`);
        }
    }

    async connectMint() {
        if (!this.CashuMint) {
            this.log('❌ Cashu quantum protocols not loaded');
            return;
        }

        try {
            this.log('💰 Establishing connection to quantum mint...');
            
            const mintUrl = 'http://localhost:3333';
            this.mint = new this.CashuMint(mintUrl);
            
            // Test connection by getting mint info
            const mintInfo = await this.mint.getInfo();
            this.log(`✅ Quantum mint connected: ${mintInfo.name || 'Unnamed quantum mint'}`);
            this.log(`📋 Mint quantum portal: ${mintUrl}`);
            this.log(`🔗 Mint quantum version: ${mintInfo.version || 'Unknown'}`);
            
            // Update mint health status
            this.updateMintHealth('Healthy', mintInfo);
            
            // Create wallet instance
            this.wallet = new this.CashuWallet(this.mint);
            this.log('💼 Quantum wallet materialized');
            
            await this.checkBalance();
            
        } catch (error) {
            this.log(`❌ Quantum mint connection failed: ${error.message}`);
            this.log('🔧 Ensure CDK quantum mint is operational on localhost:3333');
            this.updateMintHealth('Failed');
        }
    }

    updateMintHealth(health, info) {
        if (this.onStatusUpdate) {
            this.onStatusUpdate({
                ...this.status,
                cashuMint: { 
                    status: health === 'Healthy' ? 'Connected' : 'Disconnected',
                    health: health,
                    totalTokens: this.mintStats.totalTokensMinted,
                    version: info?.version,
                    name: info?.name,
                }
            });
        }
    }

    async mintTokens() {
        if (!this.wallet) {
            this.log('❌ No quantum wallet connected');
            return;
        }

        try {
            this.log('🏭 Materializing 10 quantum mana tokens...');
            
            // Request a quote for minting
            const amount = 10;
            const quote = await this.wallet.createMintQuote(amount);
            this.log(`💳 Quantum quote requested for ${amount} sats`);
            this.log(`🔗 Lightning quantum invoice: ${quote.request.substring(0, 50)}...`);
            
            // For testing with fake wallet, the quote should be automatically paid
            this.log('⚡ Processing lightning quantum payment...');
            
            // Mint the tokens
            const proofs = await this.wallet.mintTokens(amount, quote.quote);
            this.log(`✅ Materialized ${proofs.length} quantum proofs totaling ${amount} sats`);
            
            this.proofs.push(...proofs);
            await this.checkBalance();
            
        } catch (error) {
            this.log(`❌ Quantum materialization failed: ${error.message}`);
            this.log(`🔍 Quantum error analysis: ${JSON.stringify(error)}`);
        }
    }

    async checkBalance() {
        if (!this.wallet) {
            this.log('❌ No quantum wallet connected');
            return;
        }

        try {
            // Calculate balance from stored proofs
            const balance = this.proofs.reduce((sum, proof) => sum + proof.amount, 0);
            
            this.balance = balance;
            this.log(`💰 Current quantum balance: ${balance} mana`);
            
            this.updateStatus('balance', `${balance} mana`);
            
            const proofCountElement = document.getElementById('proof-count');
            if (proofCountElement) {
                proofCountElement.textContent = this.proofs.length.toString();
            }
            
        } catch (error) {
            this.log(`❌ Quantum balance check failed: ${error.message}`);
        }
    }

    async showProofs() {
        if (this.proofs.length === 0) {
            this.log('📄 No quantum proofs to display');
            return;
        }

        this.log(`📄 Current quantum proofs (${this.proofs.length}):`);
        this.proofs.forEach((proof, index) => {
            this.log(`  ${index + 1}. Amount: ${proof.amount}, Secret: ${proof.secret.substring(0, 16)}...`);
        });
    }

    async connectGameEngine() {
        try {
            this.log('🎮 Quantum game engine operates via Nostr protocol...');
            
            // Game engine communicates purely via Nostr - no HTTP endpoints
            if (this.connected && this.nostr) {
                this.log('✅ Game engine accessible via Nostr relay');
                this.log('🤖 Pure state machine architecture - no HTTP endpoints required');
                this.updateStatus('gameEngine', 'Nostr Ready');
                
                const engineStatusElement = document.getElementById('engine-status');
                if (engineStatusElement) {
                    engineStatusElement.textContent = 'Nostr Ready';
                }
                
                // List matches via Nostr events instead of HTTP
                await this.listMatches();
            } else {
                this.log('⚠️ Connect to Nostr relay first for game engine communication');
                this.updateStatus('gameEngine', 'Nostr Required');
                
                const engineStatusElement = document.getElementById('engine-status');
                if (engineStatusElement) {
                    engineStatusElement.textContent = 'Nostr Required';
                }
            }
            
        } catch (error) {
            this.log(`❌ Game engine setup failed: ${error.message}`);
            this.updateStatus('gameEngine', 'Error');
            
            const engineStatusElement = document.getElementById('engine-status');
            if (engineStatusElement) {
                engineStatusElement.textContent = 'Error';
            }
        }
    }

    async createMatch() {
        if (!this.nostr || !this.signer) {
            this.log('❌ Establish Nostr quantum tunnel first to create matches');
            return;
        }

        try {
            this.log('🎯 Creating quantum match challenge...');
            
            // Create a match challenge event (Kind 31000)
            const challengeEvent = new this.NDKEvent(this.nostr, {
                kind: 31000,
                content: JSON.stringify({
                    wager_amount: 2,
                    league_id: 1,
                    challenge_message: "Quantum web client match challenge initiated!"
                }),
                created_at: Math.floor(Date.now() / 1000),
                tags: [
                    ['d', `match-${Date.now()}`], // Replaceable event identifier
                ]
            });
            
            await challengeEvent.sign(this.signer);
            await challengeEvent.publish();
            
            this.log(`✅ Quantum match challenge transmitted to network`);
            this.log(`🆔 Event quantum signature: ${challengeEvent.id}`);
            
        } catch (error) {
            this.log(`❌ Quantum match creation failed: ${error.message}`);
        }
    }

    async listMatches() {
        try {
            this.log('📋 Scanning for active quantum matches...');
            
            // For now, simulate active matches
            // In a real implementation, you'd query the game engine or Nostr events
            const activeMatches = Math.floor(Math.random() * 5);
            
            const matchCountElement = document.getElementById('match-count');
            if (matchCountElement) {
                matchCountElement.textContent = activeMatches.toString();
            }
            
            this.updateStatus('activeGames', activeMatches.toString());
            
            this.log(`📊 Detected ${activeMatches} active quantum matches`);
            
        } catch (error) {
            this.log(`❌ Quantum match scan failed: ${error.message}`);
        }
    }

    updateStatus(key, value) {
        this.status[key] = value;
        if (this.onStatusUpdate) {
            this.onStatusUpdate(this.status);
        }
    }

    log(message) {
        const timestamp = new Date().toLocaleTimeString();
        const logMessage = `[${timestamp}] ${message}`;
        
        if (this.onLog) {
            this.onLog(logMessage);
        }
        
        // Also log to browser console
        console.log(`[MANASTR-QUANTUM] ${message}`);
    }

    // Deterministic player creation (matching integration test)
    createDeterministicKeys(seed) {
        // Use same hash-based key generation as integration test
        const encoder = new TextEncoder();
        const data = encoder.encode(seed);
        
        return crypto.subtle.digest('SHA-256', data).then(hashBuffer => {
            const hashArray = new Uint8Array(hashBuffer);
            const privateKeyHex = Array.from(hashArray, b => b.toString(16).padStart(2, '0')).join('');
            return privateKeyHex;
        });
    }

    async createPlayer(name, seed) {
        try {
            this.log(`👥 Creating player ${name} with deterministic seed...`);
            
            // Create deterministic keys (same method as integration test)
            const privateKeyHex = await this.createDeterministicKeys(seed);
            
            // Create signer from deterministic private key
            const signer = new this.NDKPrivateKeySigner(privateKeyHex);
            const user = await signer.user();
            const npub = user.npub;
            
            // Create separate NDK instance for this player
            const playerNostr = new this.NDK({
                explicitRelayUrls: ['ws://localhost:7777'],
                signer: signer
            });
            
            await playerNostr.connect();
            
            // Create gaming wallet for this player (like integration test)
            const mintUrl = 'http://localhost:3333';
            const playerMint = new this.CashuMint(mintUrl);
            
            // Test mint connection first
            try {
                await playerMint.getInfo();
                this.log(`🔗 Mint connection verified for ${name}`);
            } catch (mintError) {
                throw new Error(`Mint connection failed for ${name}: ${mintError.message}`);
            }
            
            const gamingWallet = new this.CashuWallet(playerMint);
            
            // Mint initial gaming tokens (like integration test: 100 mana)
            this.log(`💰 Minting 100 mana tokens for ${name}...`);
            
            try {
                // Request quote for minting tokens (try without currency unit first)
                const mintAmount = 100;
                this.log(`📋 Requesting mint quote for ${mintAmount} tokens...`);
                
                // Real CDK mint integration - fix API format issue
                let quote;
                try {
                    // Test different API formats to match what mint expects
                    this.log(`📋 Requesting mint quote for ${mintAmount} mana tokens...`);
                    
                    // Try different parameter formats for Cashu-TS library
                    try {
                        // Format 1: String parameter  
                        quote = await gamingWallet.createMintQuote(mintAmount, 'mana');
                        this.log(`📋 String format worked: ${quote.quote}`);
                    } catch (stringError) {
                        this.log(`⚠️ String format failed: ${stringError.message}`);
                        try {
                            // Format 2: Object parameter
                            quote = await gamingWallet.createMintQuote(mintAmount, { unit: 'mana' });
                            this.log(`📋 Object format worked: ${quote.quote}`);
                        } catch (objectError) {
                            this.log(`⚠️ Object format failed: ${objectError.message}`);
                            // Format 3: No unit parameter (defaults to sat)
                            quote = await gamingWallet.createMintQuote(mintAmount);
                            this.log(`📋 Default format worked (sat): ${quote.quote}`);
                        }
                    }
                    
                } catch (quoteError) {
                    this.log(`❌ All quote formats failed: ${quoteError.message}`);
                    this.log(`🔍 Final error details: ${JSON.stringify(quoteError, null, 2)}`);
                    throw quoteError;
                }
                
                // Wait until mint marks quote as PAID (fake wallet auto-pays in integration)
                await this.waitForMintQuotePaid(gamingWallet, quote.quote);

                // Use correct Cashu-TS API: mintProofs method with 100x1 outputs
                const oneUnitOutputs = { keepAmounts: Array.from({ length: mintAmount }, () => 1) };
                this.log(`🔨 Minting ${mintAmount} proofs of 1 mana each with quote ${quote.quote}...`);
                const proofs = await gamingWallet.mintProofs(mintAmount, quote.quote, { outputAmounts: oneUnitOutputs });
                this.log(`✅ Mint result: ${JSON.stringify(proofs, null, 2)}`);
                this.log(`✅ Minted ${proofs.length} real CDK proofs for ${name}`);
                
                
                // Calculate balance from proofs
                const balance = proofs.reduce((sum, proof) => sum + (proof.amount || 0), 0);
                this.mintStats.totalTokensMinted += proofs.length;
                this.updateMintHealth('Healthy');
                
                const player = {
                    name,
                    npub,
                    signer,
                    nostrClient: playerNostr,
                    wallet: gamingWallet,
                    tokens: proofs,
                    balance: balance,
                    eventsPublished: 0,
                    connected: true
                };
                
                this.players[name.toLowerCase()] = player;
                
                this.log(`✅ Player ${name} created successfully`);
                this.log(`🔑 ${name} npub: ${npub.substring(0, 20)}...`);
                this.log(`💰 ${name} balance: ${player.balance} mana`);
                
                // Update UI via callback
                if (this.onPlayerUpdate) {
                    this.onPlayerUpdate(name.toLowerCase(), {
                        balance: player.balance,
                        npub: npub,
                        eventsPublished: player.eventsPublished,
                        connected: true
                    });
                }
                
                return player;
                
            } catch (mintError) {
                this.log(`❌ Token minting failed for ${name}: ${mintError.message}`);
                this.log(`🔍 Mint error details: ${JSON.stringify(mintError, null, 2)}`);
                throw new Error(`Token minting failed: ${mintError.message}`);
            }
            
        } catch (error) {
            this.log(`❌ Failed to create player ${name}: ${error.message}`);
            
            // Update UI to show player creation failed
            if (this.onPlayerUpdate) {
                this.onPlayerUpdate(name.toLowerCase(), {
                    balance: 0,
                    npub: 'Creation failed',
                    eventsPublished: 0,
                    connected: false
                });
            }
            
            throw error;
        }
    }

    async createPlayers() {
        try {
            this.log('🎭 Creating deterministic test players...');
            
            // Create Alexi and Boberto with same seeds as integration test
            const alexi = await this.createPlayer('Alexi', 'Alexi');
            const boberto = await this.createPlayer('Boberto', 'Boberto'); 
            
            this.log('✅ Both players created successfully');
            this.log(`🏛️ Alexi balance: ${alexi.balance} mana`);
            this.log(`🏛️ Boberto balance: ${boberto.balance} mana`);
            
            return { alexi, boberto };
            
        } catch (error) {
            this.log(`❌ Player creation failed: ${error.message}`);
            throw error;
        }
    }

    // ============= 7-PHASE MATCH FLOW IMPLEMENTATION =============

    async createMatchChallenge() {
        this.log('🎯 Phase 1: Creating match challenge...');
        
        if (!this.players.alexi || !this.players.boberto) {
            throw new Error('Players must be created first');
        }
        
        try {
            const challenger = this.players.alexi;
            const wagerAmount = this.currentMatch.wagerAmount;
            
            this.log(`🏛️ ${challenger.name} creating challenge with ${wagerAmount} mana wager`);
            
            // Create commitment for tokens (simplified for web demo)
            const tokenSecrets = challenger.tokens.slice(0, wagerAmount).map(token => token.secret);
            const tokenNonce = this.generateNonce();
            const tokenCommitment = await this.createCommitment(tokenSecrets.join(''), tokenNonce);
            
            // Create match challenge event (Kind 31000)
            const challengeData = {
                challenger_npub: challenger.npub,
                wager_amount: wagerAmount,
                league_id: this.currentMatch.leagueId,
                cashu_token_commitment: tokenCommitment,
                expires_at: Math.floor(Date.now() / 1000) + 3600,
                created_at: Math.floor(Date.now() / 1000),
                match_event_id: '' // Will be set after event creation
            };
            
            const event = new this.NDKEvent(challenger.nostrClient, {
                kind: 31000,
                content: JSON.stringify(challengeData),
                created_at: Math.floor(Date.now() / 1000),
                tags: [
                    ['d', `match-${Date.now()}`] // Replaceable event identifier
                ]
            });
            
            await event.sign(challenger.signer);
            await event.publish();
            
            // Update match state
            this.currentMatch.id = event.id;
            this.currentMatch.challengeEvent = event;
            this.currentMatch.challenger = challenger;
            challengeData.match_event_id = event.id;
            
            this.log(`✅ Challenge created with event ID: ${event.id.substring(0, 16)}...`);
            this.log(`💰 Wager: ${wagerAmount} mana from ${challenger.name}`);
            
            this.emitMatchUpdate('challenged');
            
        } catch (error) {
            this.log(`❌ Challenge creation failed: ${error.message}`);
            throw error;
        }
    }

    async acceptMatchChallenge() {
        this.log('🎲 Phase 2: Accepting challenge...');
        
        if (!this.currentMatch.challengeEvent) {
            throw new Error('No challenge to accept - create challenge first');
        }
        
        try {
            const acceptor = this.players.boberto;
            const challengeId = this.currentMatch.id;
            
            this.log(`🏛️ ${acceptor.name} accepting challenge ${challengeId.substring(0, 16)}...`);
            
            // Create commitment for acceptor's tokens
            const tokenSecrets = acceptor.tokens.slice(0, this.currentMatch.wagerAmount).map(token => token.secret);
            const tokenNonce = this.generateNonce();
            const tokenCommitment = await this.createCommitment(tokenSecrets.join(''), tokenNonce);
            
            // Create match acceptance event (Kind 31001)
            const acceptanceData = {
                acceptor_npub: acceptor.npub,
                match_event_id: challengeId,
                cashu_token_commitment: tokenCommitment,
                accepted_at: Math.floor(Date.now() / 1000)
            };
            
            const event = new this.NDKEvent(acceptor.nostrClient, {
                kind: 31001,
                content: JSON.stringify(acceptanceData),
                created_at: Math.floor(Date.now() / 1000)
            });
            
            await event.sign(acceptor.signer);
            await event.publish();
            
            // Update match state
            this.currentMatch.acceptanceEvent = event;
            this.currentMatch.acceptor = acceptor;
            
            this.log(`✅ Challenge accepted by ${acceptor.name}`);
            this.log(`🎮 Match is now active between ${this.currentMatch.challenger.name} vs ${acceptor.name}`);
            
            this.emitMatchUpdate('accepted');
            
        } catch (error) {
            this.log(`❌ Challenge acceptance failed: ${error.message}`);
            throw error;
        }
    }

    async revealTokens() {
        this.log('🔮 Phase 3: Revealing tokens for army verification...');
        
        if (!this.currentMatch.acceptanceEvent) {
            throw new Error('Match must be accepted before revealing tokens');
        }
        
        try {
            const matchId = this.currentMatch.id;
            
            // Both players reveal their tokens
            await this.publishTokenReveal(this.currentMatch.challenger, matchId);
            await new Promise(resolve => setTimeout(resolve, 100)); // Brief delay
            await this.publishTokenReveal(this.currentMatch.acceptor, matchId);
            
            this.log('✅ Token revelation complete - armies can now be generated from C values');
            
            this.emitMatchUpdate('tokens_revealed');
            
        } catch (error) {
            this.log(`❌ Token revelation failed: ${error.message}`);
            throw error;
        }
    }

    async publishTokenReveal(player, matchId) {
        this.log(`🔮 ${player.name} revealing Cashu tokens for army verification`);
        
        // Reveal token secrets for army generation (simplified for web demo)
        const tokenSecrets = player.tokens.slice(0, this.currentMatch.wagerAmount).map(token => token.secret);
        
        const revealData = {
            player_npub: player.npub,
            match_event_id: matchId,
            cashu_tokens: tokenSecrets,
            token_secrets_nonce: this.generateNonce(),
            revealed_at: Math.floor(Date.now() / 1000)
        };
        
        const event = new this.NDKEvent(player.nostrClient, {
            kind: 31002,
            content: JSON.stringify(revealData),
            created_at: Math.floor(Date.now() / 1000)
        });
        
        await event.sign(player.signer);
        await event.publish();
        
        this.log(`📡 ${player.name} published token reveal event`);
    }

    async executeCombat() {
        this.log('⚔️ Phase 4: Executing combat rounds...');
        
        try {
            const matchId = this.currentMatch.id;
            const rounds = 3; // 3 combat rounds like integration test
            
            this.log(`⚔️ Beginning ${rounds} rounds of turn-based combat`);
            
            // Execute turn-based combat rounds
            for (let round = 1; round <= rounds; round++) {
                this.log(`🗡️ Round ${round}/${rounds}`);
                
                // Challenger moves first
                await this.publishCombatMove(this.currentMatch.challenger, matchId, round, null);
                await new Promise(resolve => setTimeout(resolve, 100));
                
                // Acceptor responds
                await this.publishCombatMove(this.currentMatch.acceptor, matchId, round, null);
                await new Promise(resolve => setTimeout(resolve, 100));
                
                this.log(`✅ Round ${round} completed`);
            }
            
            this.log('🏆 Combat phase completed - all rounds executed');
            
            this.emitMatchUpdate('combat_complete');
            
        } catch (error) {
            this.log(`❌ Combat execution failed: ${error.message}`);
            throw error;
        }
    }

    async publishCombatMove(player, matchId, round, previousEventHash) {
        const moveData = {
            player_npub: player.npub,
            match_event_id: matchId,
            previous_event_hash: previousEventHash,
            round_number: round,
            unit_positions: [1, 2, 3, 4], // Army unit positions
            unit_abilities: ["boost", "shield"], // Unit abilities used
            move_timestamp: Math.floor(Date.now() / 1000)
        };
        
        const event = new this.NDKEvent(player.nostrClient, {
            kind: 31003,
            content: JSON.stringify(moveData),
            created_at: Math.floor(Date.now() / 1000)
        });
        
        await event.sign(player.signer);
        await event.publish();
        
        this.log(`⚔️ ${player.name} executed combat move for round ${round}`);
        return event.id;
    }

    async submitResults() {
        this.log('🏆 Phase 5: Submitting match results...');
        
        try {
            const matchId = this.currentMatch.id;
            
            // Simulate match outcome - challenger wins for demo
            const winner = this.currentMatch.challenger;
            
            this.log(`🎯 Calculating match outcome...`);
            this.log(`🏆 Winner: ${winner.name}`);
            
            // Both players submit their calculated results
            await this.publishMatchResult(this.currentMatch.challenger, matchId, winner.npub);
            await new Promise(resolve => setTimeout(resolve, 100));
            await this.publishMatchResult(this.currentMatch.acceptor, matchId, winner.npub);
            
            this.log('✅ Match results submitted by both players');
            
            this.currentMatch.winner = winner.npub;
            this.emitMatchUpdate('results_submitted');
            
        } catch (error) {
            this.log(`❌ Result submission failed: ${error.message}`);
            throw error;
        }
    }

    async publishMatchResult(player, matchId, winnerNpub) {
        const resultData = {
            player_npub: player.npub,
            match_event_id: matchId,
            final_army_state: { units: "final_state_demo" },
            all_round_results: [
                { round: 1, damage: 15 },
                { round: 2, damage: 12 },
                { round: 3, damage: 8 }
            ],
            calculated_winner: winnerNpub,
            match_completed_at: Math.floor(Date.now() / 1000)
        };
        
        const event = new this.NDKEvent(player.nostrClient, {
            kind: 31005,
            content: JSON.stringify(resultData),
            created_at: Math.floor(Date.now() / 1000)
        });
        
        await event.sign(player.signer);
        await event.publish();
        
        this.log(`📊 ${player.name} submitted match result`);
    }

    async distributeLoot() {
        this.log('💰 Phase 6: Game Engine distributing loot...');
        
        try {
            const matchId = this.currentMatch.id;
            const winner = this.currentMatch.challenger; // Demo: challenger wins
            const totalWager = this.currentMatch.wagerAmount * 2; // 2 players
            const lootAmount = Math.floor(totalWager * 0.95); // 95% to winner
            const systemFee = totalWager - lootAmount; // 5% system fee
            
            this.log(`💰 Economic model: ${totalWager} total mana → ${lootAmount} loot tokens (95%), ${systemFee} system fee`);
            this.log(`🏆 Winner: ${winner.name} receives ${lootAmount} loot tokens`);
            
            // In real implementation, Game Engine would mint loot tokens
            // For demo, we simulate the loot distribution event
            const lootData = {
                game_engine_npub: "game_engine_demo_npub",
                match_event_id: matchId,
                winner_npub: winner.npub,
                loot_cashu_token: "demo_loot_token_c_value",
                match_fee: systemFee,
                loot_issued_at: Math.floor(Date.now() / 1000),
                validation_summary: {
                    status: "success",
                    integrity_score: 1.0,
                    validation_notes: "All events verified successfully"
                }
            };
            
            // Game Engine publishes loot distribution (Kind 31006)
            // For demo, we use the main client's signer
            if (this.nostr && this.signer) {
                const event = new this.NDKEvent(this.nostr, {
                    kind: 31006,
                    content: JSON.stringify(lootData),
                    created_at: Math.floor(Date.now() / 1000)
                });
                
                await event.sign(this.signer);
                await event.publish();
                
                this.log(`📡 Game Engine published KIND 31006 Loot Distribution event`);
            }
            
            this.log('✅ Loot distribution complete - zero-coordination gaming cycle finished!');
            this.log('🎮 Revolutionary player-driven match completed successfully');
            
            this.emitMatchUpdate('loot_distributed');
            
        } catch (error) {
            this.log(`❌ Loot distribution failed: ${error.message}`);
            throw error;
        }
    }

    // Helper methods for match flow
    generateNonce() {
        return Math.random().toString(36).substring(2, 15) + Math.random().toString(36).substring(2, 15);
    }

    async createCommitment(data, nonce) {
        // Simple commitment scheme for demo (in real implementation, use SHA256)
        const encoder = new TextEncoder();
        const combinedData = encoder.encode(data + nonce);
        const hashBuffer = await crypto.subtle.digest('SHA-256', combinedData);
        const hashArray = new Uint8Array(hashBuffer);
        return Array.from(hashArray, b => b.toString(16).padStart(2, '0')).join('');
    }

    emitMatchUpdate(phase) {
        if (!this.onMatchUpdate) return;
        const info = {
            id: this.currentMatch.id,
            phase,
            wagerAmount: this.currentMatch.wagerAmount,
            leagueId: this.currentMatch.leagueId,
            challenger: this.currentMatch.challenger ? this.currentMatch.challenger.npub : null,
            acceptor: this.currentMatch.acceptor ? this.currentMatch.acceptor.npub : null,
            winner: this.currentMatch.winner || null,
        };
        this.onMatchUpdate(info);
    }

    async runFullFlow() {
        // Convenience method for UI: run phases sequentially
        await this.createPlayers();
        await this.createMatchChallenge();
        await this.acceptMatchChallenge();
        await this.revealTokens();
        await this.executeCombat();
        await this.submitResults();
        await this.distributeLoot();
    }

    async waitForMintQuotePaid(wallet, quoteId) {
        try {
            this.log(`⏳ Waiting for mint quote ${quoteId} to be paid...`);
            const maxAttempts = 60; // ~6 seconds
            const delayMs = 100;
            for (let i = 0; i < maxAttempts; i++) {
                const state = await wallet.checkMintQuote(quoteId);
                if (state && (state.state === 'PAID' || state.state === 'ISSUED')) {
                    this.log(`✅ Quote is ${state.state}`);
                    return;
                }
                await new Promise(r => setTimeout(r, delayMs));
            }
            throw new Error('Timeout waiting for mint quote to be paid');
        } catch (e) {
            this.log(`⚠️ Quote check failed or unpaid: ${e.message}`);
            throw e;
        }
    }

    resetMatch() {
        this.currentMatch = {
            id: null,
            challengeEvent: null,
            acceptanceEvent: null,
            challenger: null,
            acceptor: null,
            wagerAmount: 2,
            leagueId: 1,
            winner: null,
        };
        this.log('🧹 Match state reset');
        this.emitMatchUpdate('reset');
    }
    disconnect() {
        if (this.nostr) {
            this.nostr.disconnect();
        }
        this.log('🔌 All quantum connections severed');
    }
}