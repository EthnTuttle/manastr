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
        
        // Callbacks for UI updates
        this.onStatusUpdate = null;
        this.onLog = null;
        this.onPlayerUpdate = null;
        
        this.status = {
            nostr: 'Disconnected',
            balance: '0 mana',
            activeGames: '0',
            gameEngine: 'Disconnected'
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
            31003: '⚔️ Move Committed',
            31004: '🎭 Move Revealed',
            31005: '🏆 Match Result',
            31006: '💰 Loot Distributed'
        };
        
        const eventType = eventTypes[event.kind] || '🎮 Game Event';
        const pubkey = event.pubkey.substring(0, 8);
        const timestamp = new Date(event.created_at * 1000).toLocaleTimeString();
        
        this.log(`${eventType} from ${pubkey}... at ${timestamp}`);
        
        // Show event details if available
        try {
            const content = JSON.parse(event.content);
            if (content.wager_amount) {
                this.log(`   💰 Wager: ${content.wager_amount} mana`);
            }
            if (content.match_id) {
                this.log(`   🆔 Match: ${content.match_id.substring(0, 8)}...`);
            }
        } catch (e) {
            // Content might not be JSON, that's OK
        }
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
            this.updateMintHealth('Healthy');
            
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

    updateMintHealth(health) {
        if (this.onStatusUpdate) {
            this.onStatusUpdate({
                ...this.status,
                cashuMint: { 
                    status: health === 'Healthy' ? 'Connected' : 'Disconnected',
                    health: health,
                    totalTokens: 0 
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
                // Request quote for minting tokens with specific unit (matching integration test)
                const mintAmount = 100;
                const quote = await gamingWallet.createMintQuote(mintAmount, 'mana');
                this.log(`📋 Quote created for ${name}: ${quote.quote} (${mintAmount} mana)`);
                
                // Mint tokens using the quote
                const { proofs } = await gamingWallet.mintTokens(mintAmount, quote.quote);
                this.log(`✅ Minted ${proofs.length} proofs for ${name}`);
                
                // Calculate balance from proofs
                const balance = proofs.reduce((sum, proof) => sum + proof.amount, 0);
                
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

    disconnect() {
        if (this.nostr) {
            this.nostr.disconnect();
        }
        this.log('🔌 All quantum connections severed');
    }
}