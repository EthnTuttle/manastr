import React from 'react';
import type { Unit } from '@/types/game';
import { getAbilityDisplayName, getAbilityDescription } from '@/stores/gameStore';

interface UnitCardProps {
  unit: Unit;
  isSelected?: boolean;
  onClick?: () => void;
  className?: string;
}

export const UnitCard: React.FC<UnitCardProps> = ({
  unit,
  isSelected = false,
  onClick,
  className = ''
}) => {
  const abilityClass = `ability-${unit.ability.toLowerCase()}`;
  const healthPercentage = (unit.health / unit.max_health) * 100;
  
  return (
    <div
      className={`unit-card cursor-pointer ${isSelected ? 'selected' : ''} ${className}`}
      onClick={onClick}
    >
      {/* Unit Stats */}
      <div className="flex justify-between items-start mb-3">
        <div className="text-sm font-semibold text-gray-700">
          Battle Unit
        </div>
        <div className={`text-xs px-2 py-1 rounded-full bg-gray-100 ${abilityClass}`}>
          {getAbilityDisplayName(unit.ability)}
        </div>
      </div>
      
      {/* Stats Grid */}
      <div className="grid grid-cols-3 gap-2 mb-3">
        <div className="text-center">
          <div className="text-xs text-gray-500">ATK</div>
          <div className="text-lg font-bold text-red-600">{unit.attack}</div>
        </div>
        <div className="text-center">
          <div className="text-xs text-gray-500">DEF</div>
          <div className="text-lg font-bold text-blue-600">{unit.defense}</div>
        </div>
        <div className="text-center">
          <div className="text-xs text-gray-500">HP</div>
          <div className="text-lg font-bold text-green-600">
            {unit.health}/{unit.max_health}
          </div>
        </div>
      </div>
      
      {/* Health Bar */}
      <div className="w-full bg-gray-200 rounded-full h-2 mb-2">
        <div
          className={`h-2 rounded-full transition-all duration-300 ${
            healthPercentage > 60 
              ? 'bg-green-500' 
              : healthPercentage > 30 
                ? 'bg-yellow-500' 
                : 'bg-red-500'
          }`}
          style={{ width: `${healthPercentage}%` }}
        />
      </div>
      
      {/* Ability Description */}
      {unit.ability !== 'None' && (
        <div className="text-xs text-gray-600 mt-2">
          {getAbilityDescription(unit.ability)}
        </div>
      )}
    </div>
  );
};