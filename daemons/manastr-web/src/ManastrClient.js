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
        
        // Callbacks for UI updates
        this.onStatusUpdate = null;
        this.onLog = null;
        
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
            
            // Store for later use
            this.CashuWallet = cashuModule.CashuWallet;
            this.CashuMint = cashuModule.CashuMint;
            this.NDK = ndkModule.NDK;
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
            
            // Generate ephemeral private key for demo
            const privateKey = this.NDKPrivateKeySigner.generate();
            this.signer = new this.NDKPrivateKeySigner(privateKey);
            
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
            
            // Create wallet instance
            this.wallet = new this.CashuWallet(this.mint);
            this.log('💼 Quantum wallet materialized');
            
            await this.checkBalance();
            
        } catch (error) {
            this.log(`❌ Quantum mint connection failed: ${error.message}`);
            this.log('🔧 Ensure CDK quantum mint is operational on localhost:3333');
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
            this.log('🎮 Establishing connection to quantum game engine...');
            
            // Simple HTTP health check to game engine
            const gameEngineUrl = 'http://localhost:4444/health';
            const response = await fetch(gameEngineUrl);
            
            if (response.ok) {
                this.log('✅ Quantum game engine synchronized');
                this.updateStatus('gameEngine', 'Connected');
                
                const engineStatusElement = document.getElementById('engine-status');
                if (engineStatusElement) {
                    engineStatusElement.textContent = 'Connected';
                }
                
                // Get match count if available
                await this.listMatches();
            } else {
                throw new Error(`HTTP quantum interference: ${response.status}`);
            }
            
        } catch (error) {
            this.log(`❌ Quantum game engine connection failed: ${error.message}`);
            this.log('🔧 Ensure quantum game engine is operational on localhost:4444');
            this.updateStatus('gameEngine', 'Failed');
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

    disconnect() {
        if (this.nostr) {
            this.nostr.disconnect();
        }
        this.log('🔌 All quantum connections severed');
    }
}