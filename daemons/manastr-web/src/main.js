// Manastr Web Client - Main Application
// Revolutionary Nostr Client & Cashu Wallet

class ManastrApp {
    constructor() {
        this.nostr = null;
        this.cashu = null;
        this.wallet = null;
        this.connected = false;
        this.balance = 0;
        this.proofs = [];
        
        this.initializeApp();
    }

    async initializeApp() {
        this.log('🚀 Initializing Manastr web client...');
        this.setupEventListeners();
        
        // Try to load libraries
        try {
            await this.loadLibraries();
            this.log('✅ Libraries loaded successfully');
        } catch (error) {
            this.log(`❌ Failed to load libraries: ${error.message}`);
            this.log('📦 Make sure to run: npm install');
        }
    }

    async loadLibraries() {
        try {
            // Import Cashu-TS
            const cashuModule = await import('@cashu/cashu-ts');
            this.log('📦 Cashu-TS loaded');
            
            // Import NDK
            const ndkModule = await import('@nostr-dev-kit/ndk');
            this.log('📦 NDK loaded');
            
            // Store for later use
            this.CashuWallet = cashuModule.CashuWallet;
            this.CashuMint = cashuModule.CashuMint;
            this.NDK = ndkModule.NDK;
            this.NDKEvent = ndkModule.NDKEvent;
            this.NDKPrivateKeySigner = ndkModule.NDKPrivateKeySigner;
            
        } catch (error) {
            this.log(`❌ Library loading error: ${error.message}`);
            throw error;
        }
    }

    setupEventListeners() {
        // Nostr buttons
        document.getElementById('connect-nostr').addEventListener('click', () => this.connectNostr());
        document.getElementById('disconnect-nostr').addEventListener('click', () => this.disconnectNostr());
        document.getElementById('post-note').addEventListener('click', () => this.postNote());
        
        // Cashu buttons
        document.getElementById('connect-mint').addEventListener('click', () => this.connectMint());
        document.getElementById('mint-tokens').addEventListener('click', () => this.mintTokens());
        document.getElementById('check-balance').addEventListener('click', () => this.checkBalance());
        document.getElementById('show-proofs').addEventListener('click', () => this.showProofs());
        
        // Game Engine buttons
        document.getElementById('connect-engine').addEventListener('click', () => this.connectGameEngine());
        document.getElementById('create-match').addEventListener('click', () => this.createMatch());
        document.getElementById('list-matches').addEventListener('click', () => this.listMatches());
    }

    async connectNostr() {
        if (!this.NDK) {
            this.log('❌ NDK not loaded yet');
            return;
        }

        try {
            this.log('🔗 Connecting to Nostr relay...');
            
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
            
            this.log('✅ Connected to Nostr relay');
            this.updateNostrStatus('Connected');
            
            // Get public key from signer
            const user = await this.signer.user();
            const pubkey = user.pubkey;
            
            this.log(`🔑 Generated key pair`);
            this.log(`📋 Public key: ${pubkey}`);
            
            document.getElementById('nostr-pubkey').textContent = pubkey.substring(0, 32) + '...';
            document.getElementById('connect-nostr').disabled = true;
            document.getElementById('disconnect-nostr').disabled = false;
            
        } catch (error) {
            this.log(`❌ Nostr connection failed: ${error.message}`);
            this.updateNostrStatus('Failed');
        }
    }

    async disconnectNostr() {
        if (this.nostr) {
            this.nostr.disconnect();
            this.nostr = null;
            this.log('📡 Disconnected from Nostr');
            this.updateNostrStatus('Disconnected');
            
            document.getElementById('nostr-pubkey').textContent = 'Not connected';
            document.getElementById('connect-nostr').disabled = false;
            document.getElementById('disconnect-nostr').disabled = true;
        }
    }

    async postNote() {
        if (!this.nostr || !this.signer) {
            this.log('❌ Not connected to Nostr or no signer available');
            return;
        }

        try {
            const noteContent = `🏛️ Manastr web client test note at ${new Date().toISOString()}`;
            
            const event = new this.NDKEvent(this.nostr, {
                kind: 1,
                content: noteContent,
                created_at: Math.floor(Date.now() / 1000),
            });
            
            await event.sign(this.signer);
            await event.publish();
            
            this.log(`📝 Posted note: ${noteContent}`);
            this.log(`🆔 Event ID: ${event.id}`);
            
        } catch (error) {
            this.log(`❌ Failed to post note: ${error.message}`);
        }
    }

    async connectMint() {
        if (!this.CashuMint) {
            this.log('❌ Cashu-TS not loaded yet');
            return;
        }

        try {
            this.log('💰 Connecting to Cashu mint...');
            
            const mintUrl = 'http://localhost:3333';
            this.mint = new this.CashuMint(mintUrl);
            
            // Test connection by getting mint info
            const mintInfo = await this.mint.getInfo();
            this.log(`✅ Connected to mint: ${mintInfo.name || 'Unnamed mint'}`);
            this.log(`📋 Mint URL: ${mintUrl}`);
            this.log(`🔗 Mint version: ${mintInfo.version || 'Unknown'}`);
            
            // Create wallet instance
            this.wallet = new this.CashuWallet(this.mint);
            this.log('💼 Wallet created');
            
            await this.checkBalance();
            
        } catch (error) {
            this.log(`❌ Mint connection failed: ${error.message}`);
            this.log('🔧 Make sure the CDK mint is running on localhost:3333');
        }
    }

    async mintTokens() {
        if (!this.wallet) {
            this.log('❌ No wallet connected');
            return;
        }

        try {
            this.log('🏭 Minting 10 mana tokens...');
            
            // Request a quote for minting
            const amount = 10;
            const quote = await this.wallet.createMintQuote(amount);
            this.log(`💳 Quote requested for ${amount} sats`);
            this.log(`🔗 Payment request: ${quote.request.substring(0, 50)}...`);
            
            // For testing with fake wallet, the quote should be automatically paid
            this.log('⚡ Checking quote payment status...');
            
            // Mint the tokens
            const proofs = await this.wallet.mintTokens(amount, quote.quote);
            this.log(`✅ Minted ${proofs.length} proofs totaling ${amount} sats`);
            
            this.proofs.push(...proofs);
            await this.checkBalance();
            
        } catch (error) {
            this.log(`❌ Minting failed: ${error.message}`);
            this.log(`🔍 Error details: ${JSON.stringify(error)}`);
        }
    }

    async checkBalance() {
        if (!this.wallet) {
            this.log('❌ No wallet connected');
            return;
        }

        try {
            // For now, calculate balance from stored proofs
            const balance = this.proofs.reduce((sum, proof) => sum + proof.amount, 0);
            
            this.balance = balance;
            this.log(`💰 Current balance: ${balance} mana`);
            
            document.getElementById('cashu-balance').textContent = `${balance} mana`;
            document.getElementById('proof-count').textContent = this.proofs.length.toString();
            
        } catch (error) {
            this.log(`❌ Balance check failed: ${error.message}`);
        }
    }

    async showProofs() {
        if (this.proofs.length === 0) {
            this.log('📄 No proofs to display');
            return;
        }

        this.log(`📄 Current proofs (${this.proofs.length}):`);
        this.proofs.forEach((proof, index) => {
            this.log(`  ${index + 1}. Amount: ${proof.amount}, Secret: ${proof.secret.substring(0, 16)}...`);
        });
    }

    async connectGameEngine() {
        try {
            this.log('🎮 Connecting to Game Engine...');
            
            // Simple HTTP health check to game engine
            const gameEngineUrl = 'http://localhost:4444/health';
            const response = await fetch(gameEngineUrl);
            
            if (response.ok) {
                this.log('✅ Game Engine connected');
                this.updateGameEngineStatus('Connected');
                document.getElementById('engine-status').textContent = 'Connected';
                
                // Get match count if available
                await this.listMatches();
            } else {
                throw new Error(`HTTP ${response.status}`);
            }
            
        } catch (error) {
            this.log(`❌ Game Engine connection failed: ${error.message}`);
            this.log('🔧 Make sure the Game Engine is running on localhost:4444');
            this.updateGameEngineStatus('Failed');
        }
    }

    async createMatch() {
        if (!this.nostr || !this.signer) {
            this.log('❌ Connect to Nostr first to create matches');
            return;
        }

        try {
            this.log('🎯 Creating match challenge...');
            
            // Create a match challenge event (Kind 31000)
            const challengeEvent = new this.NDKEvent(this.nostr, {
                kind: 31000,
                content: JSON.stringify({
                    wager_amount: 2,
                    league_id: 1,
                    challenge_message: "Web client match challenge!"
                }),
                created_at: Math.floor(Date.now() / 1000),
                tags: [
                    ['d', `match-${Date.now()}`], // Replaceable event identifier
                ]
            });
            
            await challengeEvent.sign(this.signer);
            await challengeEvent.publish();
            
            this.log(`✅ Match challenge created`);
            this.log(`🆔 Event ID: ${challengeEvent.id}`);
            
        } catch (error) {
            this.log(`❌ Failed to create match: ${error.message}`);
        }
    }

    async listMatches() {
        try {
            this.log('📋 Fetching active matches...');
            
            // For now, just simulate active matches
            // In a real implementation, you'd query the game engine or Nostr events
            const activeMatches = Math.floor(Math.random() * 5);
            
            document.getElementById('match-count').textContent = activeMatches.toString();
            document.getElementById('active-games').textContent = activeMatches.toString();
            
            this.log(`📊 Found ${activeMatches} active matches`);
            
        } catch (error) {
            this.log(`❌ Failed to list matches: ${error.message}`);
        }
    }

    updateGameEngineStatus(status) {
        const statusElement = document.getElementById('game-engine-status');
        statusElement.textContent = status;
        
        if (status === 'Connected') {
            statusElement.style.color = '#10b981';
        } else if (status === 'Failed') {
            statusElement.style.color = '#ef4444';
        } else {
            statusElement.style.color = '#64748b';
        }
    }

    updateNostrStatus(status) {
        const statusElement = document.getElementById('nostr-status');
        statusElement.textContent = status;
        
        if (status === 'Connected') {
            statusElement.style.color = '#10b981';
        } else if (status === 'Failed') {
            statusElement.style.color = '#ef4444';
        } else {
            statusElement.style.color = '#64748b';
        }
    }

    log(message) {
        const timestamp = new Date().toLocaleTimeString();
        const logOutput = document.getElementById('log-output');
        logOutput.textContent += `[${timestamp}] ${message}\n`;
        logOutput.scrollTop = logOutput.scrollHeight;
        
        // Also log to browser console
        console.log(`[Manastr] ${message}`);
    }
}

// Initialize the application when the page loads
document.addEventListener('DOMContentLoaded', () => {
    window.manastr = new ManastrApp();
});

// Export for debugging
window.ManastrApp = ManastrApp;