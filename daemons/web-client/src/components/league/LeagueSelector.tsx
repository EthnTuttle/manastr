import React from 'react';
import { useGameStore, getLeagueDisplayName } from '@/stores/gameStore';

export const LeagueSelector: React.FC = () => {
  const { 
    selectedLeague, 
    availableLeagues, 
    selectLeague,
    loadLeagueInfo,
    isLoading 
  } = useGameStore();
  
  React.useEffect(() => {
    loadLeagueInfo();
  }, [loadLeagueInfo]);
  
  const getLeagueColorClass = (leagueId: number) => {
    switch (leagueId % 4) {
      case 0: return 'league-fire border-red-300 bg-red-50 hover:bg-red-100';
      case 1: return 'league-ice border-blue-300 bg-blue-50 hover:bg-blue-100';
      case 2: return 'league-shadow border-purple-300 bg-purple-50 hover:bg-purple-100';
      case 3: return 'league-nature border-green-300 bg-green-50 hover:bg-green-100';
      default: return 'border-gray-300 bg-gray-50 hover:bg-gray-100';
    }
  };
  
  if (isLoading) {
    return (
      <div className="bg-white rounded-lg shadow-md p-6">
        <div className="animate-pulse">
          <div className="h-6 bg-gray-200 rounded w-1/3 mb-4"></div>
          <div className="space-y-3">
            {[1, 2, 3, 4].map(i => (
              <div key={i} className="h-16 bg-gray-200 rounded"></div>
            ))}
          </div>
        </div>
      </div>
    );
  }
  
  return (
    <div className="bg-white rounded-lg shadow-md p-6">
      <h2 className="text-xl font-bold text-gray-800 mb-4">Select Battle League</h2>
      
      <div className="space-y-3">
        {availableLeagues.map((league) => (
          <div
            key={league.id}
            onClick={() => selectLeague(league.id)}
            className={`
              p-4 rounded-lg border-2 cursor-pointer transition-all duration-200
              ${selectedLeague === league.id 
                ? 'ring-2 ring-blue-500 ring-offset-2' 
                : ''
              }
              ${getLeagueColorClass(league.id)}
            `}
          >
            <div className="flex justify-between items-start">
              <div>
                <h3 className="font-semibold text-gray-800">{league.name}</h3>
                <p className="text-sm text-gray-600 mt-1">
                  {getLeagueDisplayName(league.id)}
                </p>
              </div>
              
              <div className="flex space-x-3 text-sm">
                {league.attack_bonus > 0 && (
                  <div className="text-red-600 font-semibold">
                    +{league.attack_bonus} ATK
                  </div>
                )}
                {league.defense_bonus > 0 && (
                  <div className="text-blue-600 font-semibold">
                    +{league.defense_bonus} DEF
                  </div>
                )}
                {league.health_bonus > 0 && (
                  <div className="text-green-600 font-semibold">
                    +{league.health_bonus} HP
                  </div>
                )}
              </div>
            </div>
            
            {selectedLeague === league.id && (
              <div className="mt-2 text-xs text-blue-600 font-semibold">
                ✓ Currently Selected
              </div>
            )}
          </div>
        ))}
      </div>
      
      <div className="mt-4 text-xs text-gray-500">
        League bonuses are applied to all generated battle units
      </div>
    </div>
  );
};