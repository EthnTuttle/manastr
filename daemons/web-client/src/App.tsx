import { useEffect } from 'react';
import { BattleSimulator } from '@/components/battle/BattleSimulator';
import { LeagueSelector } from '@/components/league/LeagueSelector';
import { WalletStatus } from '@/components/wallet/WalletStatus';
import { LoadingSpinner } from '@/components/common/LoadingSpinner';
import { useWasmStore, initializeWasmStore } from '@/stores/wasmStore';
import { useGameStore } from '@/stores/gameStore';

function App() {
  const { isInitialized, isLoading, error } = useWasmStore();
  const { config } = useGameStore();
  
  useEffect(() => {
    // Initialize WASM store when app loads
    initializeWasmStore();
  }, []);
  
  if (isLoading) {
    return (
      <div className="min-h-screen bg-gray-100 flex items-center justify-center">
        <div className="bg-white rounded-lg shadow-lg p-8">
          <LoadingSpinner size="lg" message="Loading Mana Strategy Game..." />
          <div className="mt-4 text-center text-sm text-gray-600">
            Initializing WASM game engine...
          </div>
        </div>
      </div>
    );
  }
  
  if (error) {
    return (
      <div className="min-h-screen bg-gray-100 flex items-center justify-center">
        <div className="bg-white rounded-lg shadow-lg p-8 max-w-md">
          <div className="text-center">
            <div className="text-red-600 text-6xl mb-4">⚠️</div>
            <h1 className="text-xl font-bold text-gray-800 mb-2">
              Failed to Load Game Engine
            </h1>
            <p className="text-gray-600 mb-4">{error}</p>
            <button
              onClick={() => window.location.reload()}
              className="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700"
            >
              Retry
            </button>
          </div>
        </div>
      </div>
    );
  }
  
  return (
    <div className="min-h-screen bg-gray-100">
      {/* Header */}
      <header className="bg-white shadow-sm border-b">
        <div className="container mx-auto px-4 py-4">
          <div className="flex justify-between items-center">
            <div>
              <h1 className="text-2xl font-bold text-gray-800">
                🎮 Mana Strategy Game
              </h1>
              <p className="text-sm text-gray-600">
                Cashu CDK + WASM Game Logic Demo
              </p>
            </div>
            
            <div className="flex items-center space-x-4 text-sm">
              <div className="bg-green-50 text-green-600 px-3 py-1 rounded-full">
                ✅ WASM Engine: {isInitialized ? 'Ready' : 'Loading...'}
              </div>
              <div className="text-gray-500">
                Mint: {config.mintUrl}
              </div>
              <div className="text-gray-500">
                Engine: {config.gameEngineUrl}
              </div>
            </div>
          </div>
        </div>
      </header>
      
      {/* Main Content */}
      <main className="container mx-auto px-4 py-8">
        <div className="grid grid-cols-1 lg:grid-cols-3 gap-8">
          {/* Left Column - Wallet and League */}
          <div className="space-y-6">
            <WalletStatus />
            <LeagueSelector />
          </div>
          
          {/* Right Column - Battle Simulator */}
          <div className="lg:col-span-2">
            <BattleSimulator />
          </div>
        </div>
        
        {/* Footer Info */}
        <div className="mt-12 bg-white rounded-lg shadow-md p-6">
          <h2 className="text-lg font-bold text-gray-800 mb-4">
            🚀 WASM Shared Logic Demo
          </h2>
          <div className="grid grid-cols-1 md:grid-cols-2 gap-6 text-sm">
            <div>
              <h3 className="font-semibold text-gray-700 mb-2">Architecture Highlights</h3>
              <ul className="space-y-1 text-gray-600">
                <li>• Identical Rust logic compiled for both server and client</li>
                <li>• Client predictions match server authority perfectly</li>
                <li>• Deterministic unit generation from token secrets</li>
                <li>• Real-time combat simulation with WASM performance</li>
              </ul>
            </div>
            <div>
              <h3 className="font-semibold text-gray-700 mb-2">Technology Stack</h3>
              <ul className="space-y-1 text-gray-600">
                <li>• <strong>Backend:</strong> Rust + Axum (Game Engine Bot)</li>
                <li>• <strong>Frontend:</strong> React + TypeScript + WASM</li>
                <li>• <strong>Shared Logic:</strong> Rust → WebAssembly</li>
                <li>• <strong>Wallet:</strong> Cashu CDK (dual currencies)</li>
              </ul>
            </div>
          </div>
          
          <div className="mt-4 p-3 bg-blue-50 rounded-lg">
            <div className="text-sm text-blue-800">
              <strong>🎯 Perfect Synchronization:</strong> This client uses the exact same combat logic as the 
              authoritative game server via shared WASM modules, eliminating desynchronization issues common 
              in real-time multiplayer games.
            </div>
          </div>
        </div>
      </main>
    </div>
  );
}

export default App;