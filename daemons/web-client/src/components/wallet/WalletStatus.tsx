import React, { useState } from 'react';
import { useWasmStore } from '@/stores/wasmStore';
import { useGameStore } from '@/stores/gameStore';
import { LoadingSpinner } from '../common/LoadingSpinner';
import type { Unit } from '@/types/game';

export const WalletStatus: React.FC = () => {
  const [tokenSecret, setTokenSecret] = useState('demo_token_12345');
  const [generatedUnits, setGeneratedUnits] = useState<Unit[]>([]);
  const [isGenerating, setIsGenerating] = useState(false);
  
  const { 
    generateUnits, 
    testConnection, 
    isInitialized, 
    isLoading, 
    error 
  } = useWasmStore();
  
  const { selectedLeague, setPlayerUnits } = useGameStore();
  
  const handleGenerateUnits = async () => {
    if (!tokenSecret.trim() || isGenerating) return;
    
    setIsGenerating(true);
    try {
      const units = await generateUnits(tokenSecret.trim(), selectedLeague);
      setGeneratedUnits(units);
      setPlayerUnits(units);
      console.log('Generated units:', units);
    } catch (error) {
      console.error('Failed to generate units:', error);
    } finally {
      setIsGenerating(false);
    }
  };
  
  const handleTestConnection = async () => {
    try {
      const result = await testConnection();
      alert(`WASM Connection Test: ${result}`);
    } catch (error) {
      alert(`WASM Connection Failed: ${error}`);
    }
  };
  
  if (isLoading) {
    return (
      <div className="bg-white rounded-lg shadow-md p-6">
        <LoadingSpinner message="Initializing WASM wallet..." />
      </div>
    );
  }
  
  if (error) {
    return (
      <div className="bg-white rounded-lg shadow-md p-6">
        <div className="text-center">
          <div className="text-red-600 mb-4">
            <h3 className="font-semibold">Wallet Initialization Error</h3>
            <p className="text-sm mt-2">{error}</p>
          </div>
          <button
            onClick={handleTestConnection}
            className="px-4 py-2 bg-blue-600 text-white rounded hover:bg-blue-700"
          >
            Test WASM Connection
          </button>
        </div>
      </div>
    );
  }
  
  return (
    <div className="bg-white rounded-lg shadow-md p-6">
      <div className="flex justify-between items-center mb-6">
        <h2 className="text-xl font-bold text-gray-800">Mock Wallet</h2>
        <div className="text-xs bg-green-50 text-green-600 px-2 py-1 rounded">
          {isInitialized ? '✅ WASM Ready' : '❌ WASM Not Ready'}
        </div>
      </div>
      
      {/* Mock Balance Display */}
      <div className="grid grid-cols-2 gap-4 mb-6">
        <div className="bg-blue-50 rounded-lg p-4">
          <div className="text-sm text-blue-600 mb-1">Mana Balance</div>
          <div className="text-2xl font-bold text-blue-800">2,500</div>
          <div className="text-xs text-blue-500">Battle Currency</div>
        </div>
        
        <div className="bg-green-50 rounded-lg p-4">
          <div className="text-sm text-green-600 mb-1">Loot Balance</div>
          <div className="text-2xl font-bold text-green-800">750</div>
          <div className="text-xs text-green-500">Reward Currency</div>
        </div>
      </div>
      
      {/* Token Secret Input */}
      <div className="mb-4">
        <label className="block text-sm font-medium text-gray-700 mb-2">
          Mock Mana Token Secret
        </label>
        <input
          type="text"
          value={tokenSecret}
          onChange={(e) => setTokenSecret(e.target.value)}
          className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-blue-500 focus:border-transparent"
          placeholder="Enter token secret for unit generation..."
        />
        <p className="text-xs text-gray-500 mt-1">
          This simulates deriving battle units from a mana token secret using WASM
        </p>
      </div>
      
      {/* Generate Units Button */}
      <button
        onClick={handleGenerateUnits}
        disabled={!tokenSecret.trim() || isGenerating || !isInitialized}
        className="w-full mb-4 px-4 py-2 bg-blue-600 text-white font-semibold rounded-lg hover:bg-blue-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
      >
        {isGenerating ? (
          <div className="flex items-center justify-center">
            <div className="animate-spin rounded-full h-4 w-4 border-2 border-white border-t-transparent mr-2" />
            Generating Units with WASM...
          </div>
        ) : (
          '🎲 Generate Battle Units (WASM)'
        )}
      </button>
      
      {/* Generated Units Display */}
      {generatedUnits.length > 0 && (
        <div className="border-t pt-4">
          <h3 className="font-semibold text-gray-700 mb-3">
            Generated Units ({generatedUnits.length})
          </h3>
          <div className="grid grid-cols-2 md:grid-cols-4 gap-2 text-xs">
            {generatedUnits.map((unit, index) => (
              <div
                key={index}
                className="bg-gray-50 p-2 rounded border"
              >
                <div className="font-semibold mb-1">Unit {index + 1}</div>
                <div>ATK: {unit.attack}</div>
                <div>DEF: {unit.defense}</div>
                <div>HP: {unit.health}</div>
                <div className={`ability-${unit.ability.toLowerCase()} font-semibold`}>
                  {unit.ability}
                </div>
              </div>
            ))}
          </div>
          <div className="text-xs text-green-600 mt-2 text-center">
            ✅ Units generated using shared WASM logic (identical to server)
          </div>
        </div>
      )}
      
      {/* Test Connection Button */}
      <div className="border-t pt-4 mt-4">
        <button
          onClick={handleTestConnection}
          className="text-sm px-3 py-1 bg-gray-100 text-gray-700 rounded hover:bg-gray-200 transition-colors"
        >
          Test WASM Connection
        </button>
      </div>
    </div>
  );
};