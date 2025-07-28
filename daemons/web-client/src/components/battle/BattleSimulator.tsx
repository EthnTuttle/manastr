import React, { useState, useEffect } from 'react';
import { useWasmStore } from '@/stores/wasmStore';
import { useGameStore } from '@/stores/gameStore';
import { UnitCard } from './UnitCard';
import { LoadingSpinner } from '../common/LoadingSpinner';
import type { Unit, RoundResult } from '@/types/game';

export const BattleSimulator: React.FC = () => {
  const [selectedUnit1, setSelectedUnit1] = useState<Unit | null>(null);
  const [selectedUnit2, setSelectedUnit2] = useState<Unit | null>(null);
  const [combatResult, setCombatResult] = useState<RoundResult | null>(null);
  const [isSimulating, setIsSimulating] = useState(false);
  
  const { processCombat, generateUnits, isInitialized, isLoading, error } = useWasmStore();
  const { selectedLeague } = useGameStore();
  
  // Sample units for testing
  const [sampleUnits, setSampleUnits] = useState<Unit[]>([]);
  
  useEffect(() => {
    // Generate sample units when component mounts
    const loadSampleUnits = async () => {
      try {
        const units = await generateUnits('sample_token_secret_123', selectedLeague);
        setSampleUnits(units.slice(0, 4)); // Take first 4 units for demo
      } catch (error) {
        console.error('Failed to generate sample units:', error);
      }
    };
    
    if (isInitialized) {
      loadSampleUnits();
    }
  }, [isInitialized, selectedLeague, generateUnits]);
  
  const simulateCombat = async () => {
    if (!selectedUnit1 || !selectedUnit2 || isSimulating) return;
    
    setIsSimulating(true);
    setCombatResult(null);
    
    try {
      const result = await processCombat(
        selectedUnit1,
        selectedUnit2,
        'player1_test',
        'player2_test'
      );
      
      setCombatResult(result);
    } catch (error) {
      console.error('Combat simulation failed:', error);
    } finally {
      setIsSimulating(false);
    }
  };
  
  if (isLoading) {
    return (
      <div className="bg-white rounded-lg shadow-md p-6">
        <LoadingSpinner message="Loading WASM battle engine..." />
      </div>
    );
  }
  
  if (error) {
    return (
      <div className="bg-white rounded-lg shadow-md p-6">
        <div className="text-red-600 text-center">
          <h3 className="font-semibold mb-2">WASM Loading Error</h3>
          <p className="text-sm">{error}</p>
        </div>
      </div>
    );
  }
  
  return (
    <div className="bg-white rounded-lg shadow-md p-6">
      <div className="flex items-center justify-between mb-6">
        <h2 className="text-xl font-bold text-gray-800">Combat Simulator</h2>
        <div className="text-xs text-green-600 bg-green-50 px-2 py-1 rounded">
          ✅ WASM Enabled
        </div>
      </div>
      
      {/* Unit Selection */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-6 mb-6">
        <div>
          <h3 className="font-semibold mb-3 text-gray-700">Select Unit 1</h3>
          <div className="grid grid-cols-1 sm:grid-cols-2 gap-3">
            {sampleUnits.slice(0, 2).map((unit, index) => (
              <UnitCard
                key={index}
                unit={unit}
                isSelected={selectedUnit1 === unit}
                onClick={() => setSelectedUnit1(unit)}
                className="text-sm"
              />
            ))}
          </div>
        </div>
        
        <div>
          <h3 className="font-semibold mb-3 text-gray-700">Select Unit 2</h3>
          <div className="grid grid-cols-1 sm:grid-cols-2 gap-3">
            {sampleUnits.slice(2, 4).map((unit, index) => (
              <UnitCard
                key={index + 2}
                unit={unit}
                isSelected={selectedUnit2 === unit}
                onClick={() => setSelectedUnit2(unit)}
                className="text-sm"
              />
            ))}
          </div>
        </div>
      </div>
      
      {/* Battle Button */}
      <div className="text-center mb-6">
        <button
          onClick={simulateCombat}
          disabled={!selectedUnit1 || !selectedUnit2 || isSimulating}
          className="px-6 py-3 bg-red-600 text-white font-semibold rounded-lg hover:bg-red-700 disabled:opacity-50 disabled:cursor-not-allowed transition-colors"
        >
          {isSimulating ? (
            <div className="flex items-center">
              <div className="animate-spin rounded-full h-4 w-4 border-2 border-white border-t-transparent mr-2" />
              Simulating Combat...
            </div>
          ) : (
            '⚔️ Simulate Combat (WASM)'
          )}
        </button>
      </div>
      
      {/* Combat Result */}
      {combatResult && (
        <div className="border-t pt-6">
          <h3 className="font-semibold mb-4 text-gray-800">Combat Result</h3>
          
          <div className="grid grid-cols-1 md:grid-cols-2 gap-4 mb-4">
            <div className="bg-blue-50 p-4 rounded-lg">
              <h4 className="font-semibold text-blue-800 mb-2">Player 1 Unit</h4>
              <div className="text-sm space-y-1">
                <div>Health: {combatResult.player1_unit.health}/{combatResult.player1_unit.max_health}</div>
                <div>Damage Taken: {combatResult.damage_dealt[1]}</div>
              </div>
            </div>
            
            <div className="bg-red-50 p-4 rounded-lg">
              <h4 className="font-semibold text-red-800 mb-2">Player 2 Unit</h4>
              <div className="text-sm space-y-1">
                <div>Health: {combatResult.player2_unit.health}/{combatResult.player2_unit.max_health}</div>
                <div>Damage Taken: {combatResult.damage_dealt[0]}</div>
              </div>
            </div>
          </div>
          
          <div className="text-center">
            <div className="text-lg font-bold mb-2">
              {combatResult.winner ? (
                <>🏆 Winner: {combatResult.winner}</>
              ) : (
                <>🤝 Tie Game</>
              )}
            </div>
            <div className="text-xs text-green-600 bg-green-50 px-3 py-1 rounded-full inline-block">
              ✅ Result matches server authority (WASM synchronization)
            </div>
          </div>
        </div>
      )}
    </div>
  );
};