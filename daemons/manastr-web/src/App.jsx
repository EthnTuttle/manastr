import React, { useState, useEffect } from 'react';
import styled, { createGlobalStyle, keyframes } from 'styled-components';
import ManastrClient from './ManastrClient';

// Global Arwes-style styling
const GlobalStyle = createGlobalStyle`
  body {
    background: linear-gradient(135deg, #000000 0%, #001122 50%, #000000 100%);
    color: #00ffff;
    font-family: 'Courier New', monospace;
    margin: 0;
    padding: 0;
    min-height: 100vh;
    overflow-x: hidden;
  }
  
  * {
    box-sizing: border-box;
  }
`;

const glowAnimation = keyframes`
  0%, 100% { box-shadow: 0 0 5px #00ffff, 0 0 10px #00ffff, 0 0 15px #00ffff; }
  50% { box-shadow: 0 0 10px #00ffff, 0 0 20px #00ffff, 0 0 30px #00ffff; }
`;

const scanlineAnimation = keyframes`
  0% { transform: translateY(-100vh); }
  100% { transform: translateY(100vh); }
`;

const pulseGlow = keyframes`
  0%, 100% { 
    text-shadow: 0 0 5px #00ffff, 0 0 10px #00ffff, 0 0 15px #00ffff, 0 0 20px #00ffff;
    filter: brightness(1);
  }
  50% { 
    text-shadow: 0 0 10px #00ffff, 0 0 20px #00ffff, 0 0 30px #00ffff, 0 0 40px #00ffff;
    filter: brightness(1.2);
  }
`;

const AppContainer = styled.div`
  min-height: 100vh;
  padding: 20px;
  position: relative;
  overflow: hidden;
  
  &::before {
    content: '';
    position: fixed;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: 
      radial-gradient(circle at 20% 20%, rgba(0, 255, 255, 0.1) 0%, transparent 50%),
      radial-gradient(circle at 80% 80%, rgba(139, 92, 246, 0.1) 0%, transparent 50%),
      linear-gradient(45deg, transparent 48%, rgba(0, 255, 255, 0.03) 49%, rgba(0, 255, 255, 0.03) 51%, transparent 52%),
      linear-gradient(-45deg, transparent 48%, rgba(139, 92, 246, 0.03) 49%, rgba(139, 92, 246, 0.03) 51%, transparent 52%);
    background-size: 100% 100%, 100% 100%, 50px 50px, 50px 50px;
    pointer-events: none;
    z-index: -1;
  }
  
  &::after {
    content: '';
    position: fixed;
    top: 0;
    left: 0;
    width: 100%;
    height: 2px;
    background: linear-gradient(90deg, transparent, #00ffff, transparent);
    animation: ${scanlineAnimation} 4s linear infinite;
    z-index: 1;
    pointer-events: none;
    opacity: 0.6;
  }
`;

const Header = styled.div`
  text-align: center;
  margin-bottom: 30px;
`;

const Title = styled.h1`
  font-size: 3rem;
  margin: 0;
  background: linear-gradient(45deg, #00ffff, #8b5cf6, #00ffff);
  background-size: 200% 200%;
  animation: ${pulseGlow} 3s ease-in-out infinite;
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
  
  @media (max-width: 768px) {
    font-size: 2rem;
  }
`;

const Subtitle = styled.div`
  font-size: 1.2rem;
  color: #7fffd4;
  margin: 5px 0;
  text-shadow: 0 0 10px rgba(127, 255, 212, 0.5);
  
  @media (max-width: 768px) {
    font-size: 1rem;
  }
`;

const DashboardLayout = styled.div`
  display: grid;
  grid-template-columns: 1fr;
  gap: 20px;
  max-width: 1400px;
  margin: 0 auto;
`;

const ServiceRow = styled.div`
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  gap: 20px;
`;

const PlayerRow = styled.div`
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 20px;
  
  @media (max-width: 768px) {
    grid-template-columns: 1fr;
  }
`;

const MatchControlsRow = styled.div`
  display: grid;
  grid-template-columns: 1fr;
  gap: 20px;
`;

const EntityBox = styled.div`
  background: rgba(0, 17, 34, 0.9);
  border: 1px solid #00ffff;
  padding: 20px;
  position: relative;
  transition: all 0.3s ease;
  
  &::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    border: 2px solid transparent;
    background: linear-gradient(45deg, #00ffff, transparent, #8b5cf6, transparent, #00ffff) border-box;
    -webkit-mask: linear-gradient(#fff 0 0) padding-box, linear-gradient(#fff 0 0);
    -webkit-mask-composite: subtract;
    mask-composite: exclude;
    opacity: 0.3;
    transition: opacity 0.3s ease;
  }
  
  &::after {
    content: '';
    position: absolute;
    top: 5px;
    left: 5px;
    right: 5px;
    bottom: 5px;
    border: 1px solid rgba(0, 255, 255, 0.2);
    pointer-events: none;
  }
  
  &:hover {
    background: rgba(0, 17, 34, 1);
    box-shadow: 0 0 20px rgba(0, 255, 255, 0.3);
    
    &::before {
      opacity: 0.6;
    }
  }
`;

const EntityTitle = styled.h3`
  margin: 0 0 15px 0;
  color: #00ffff;
  font-size: 1.2rem;
  text-transform: uppercase;
  letter-spacing: 2px;
  text-align: center;
  text-shadow: 0 0 10px rgba(0, 255, 255, 0.5);
`;

const EntityStatus = styled.div`
  display: flex;
  justify-content: space-between;
  margin: 10px 0;
  font-size: 0.9rem;
`;

const StatusLabel = styled.span`
  color: #7fffd4;
`;

const StatusValue = styled.span`
  color: #00ffff;
  font-family: 'Courier New', monospace;
  text-shadow: 0 0 5px rgba(0, 255, 255, 0.3);
`;

const MatchFlowSection = styled(EntityBox)`
  text-align: center;
`;

const PhaseGrid = styled.div`
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
  gap: 15px;
  margin: 20px 0;
`;

const PhaseButton = styled.button`
  background: linear-gradient(45deg, rgba(0, 255, 255, 0.2), rgba(139, 92, 246, 0.2));
  border: 1px solid #00ffff;
  color: #00ffff;
  padding: 15px 10px;
  font-family: 'Courier New', monospace;
  font-size: 0.9rem;
  text-transform: uppercase;
  cursor: pointer;
  transition: all 0.3s ease;
  position: relative;
  
  &:hover {
    background: linear-gradient(45deg, rgba(0, 255, 255, 0.4), rgba(139, 92, 246, 0.4));
    box-shadow: 0 0 15px rgba(0, 255, 255, 0.5);
    transform: translateY(-2px);
  }
  
  &:disabled {
    opacity: 0.5;
    cursor: not-allowed;
    transform: none;
  }
  
  &.completed {
    background: linear-gradient(45deg, rgba(34, 197, 94, 0.3), rgba(21, 128, 61, 0.3));
    border-color: #22c55e;
    color: #22c55e;
  }
  
  &.active {
    animation: ${glowAnimation} 2s ease-in-out infinite;
  }
`;

const ActivityFeed = styled.div`
  background: rgba(0, 0, 0, 0.9);
  border: 1px solid #00ffff;
  padding: 20px;
  max-height: 400px;
  overflow-y: auto;
  
  &::-webkit-scrollbar {
    width: 8px;
  }
  
  &::-webkit-scrollbar-track {
    background: #001100;
  }
  
  &::-webkit-scrollbar-thumb {
    background: #00ffff;
    border-radius: 4px;
  }
`;

const ActivityTitle = styled.h3`
  margin: 0 0 15px 0;
  color: #7fffd4;
  text-transform: uppercase;
  letter-spacing: 2px;
`;

const ActivityLog = styled.div`
  font-family: 'Courier New', monospace;
  font-size: 0.9rem;
  color: #0f0;
  line-height: 1.4;
  white-space: pre-wrap;
`;

export default function App() {
  const [manastrClient] = useState(new ManastrClient());
  
  // Service states
  const [serviceStates, setServiceStates] = useState({
    nostrRelay: { status: 'Disconnected', events: 0, connections: 0 },
    cashuMint: { status: 'Disconnected', health: 'Unknown', totalTokens: 0 },
    gameEngine: { status: 'Disconnected', matches: 0, validations: 0 }
  });
  
  // Player states  
  const [playerStates, setPlayerStates] = useState({
    alexi: { balance: 0, npub: '', eventsPublished: 0, connected: false },
    boberto: { balance: 0, npub: '', eventsPublished: 0, connected: false }
  });
  
  // Match flow state
  const [matchFlow, setMatchFlow] = useState({
    currentPhase: 0,
    phases: [
      { name: '👥 Create Players', completed: false },
      { name: '🎯 Challenge', completed: false },
      { name: '🎲 Accept', completed: false },
      { name: '🔮 Reveal Tokens', completed: false },
      { name: '⚔️ Combat', completed: false },
      { name: '🏆 Results', completed: false },
      { name: '💰 Loot Distribution', completed: false }
    ]
  });
  
  const [logs, setLogs] = useState('MANASTR Integration Test Dashboard\nReady for revolutionary gaming operations.\n');

  useEffect(() => {
    // Set up client callbacks
    manastrClient.onStatusUpdate = (status) => {
      setServiceStates(prev => ({
        ...prev,
        nostrRelay: { 
          ...prev.nostrRelay, 
          status: status.nostr === 'Connected' ? 'Connected' : 'Disconnected',
          events: prev.nostrRelay.events,
          connections: status.nostr === 'Connected' ? 1 : 0
        },
        cashuMint: status.cashuMint || { 
          ...prev.cashuMint, 
          status: 'Unknown', 
          health: 'Unknown',
          totalTokens: prev.cashuMint.totalTokens
        },
        gameEngine: { 
          ...prev.gameEngine, 
          status: status.gameEngine === 'Nostr Ready' ? 'Nostr Ready' : 'Disconnected',
          matches: prev.gameEngine.matches,
          validations: prev.gameEngine.validations
        }
      }));
    };
    
    manastrClient.onPlayerUpdate = (playerName, playerData) => {
      setPlayerStates(prev => ({
        ...prev,
        [playerName]: { ...prev[playerName], ...playerData }
      }));
    };
    
    manastrClient.onLog = (message) => {
      setLogs(prev => prev + `[${new Date().toLocaleTimeString()}] ${message}\n`);
    };

    // Initialize the client
    manastrClient.initialize();

    return () => {
      if (manastrClient.nostr) {
        manastrClient.disconnectNostr();
      }
    };
  }, [manastrClient]);

  const executePhase = async (phaseIndex) => {
    const phaseName = matchFlow.phases[phaseIndex].name;
    manastrClient.log(`🎮 Executing ${phaseName}...`);
    
    try {
      switch (phaseIndex) {
        case 0: // Create Players
          await manastrClient.createPlayers();
          break;
        case 1: // Challenge
          manastrClient.log('🎯 Phase 1: Creating match challenge...');
          // TODO: Implement challenge creation
          break;
        case 2: // Accept
          manastrClient.log('🎲 Phase 2: Accepting challenge...');
          // TODO: Implement challenge acceptance
          break;
        case 3: // Reveal Tokens
          manastrClient.log('🔮 Phase 3: Revealing tokens...');
          // TODO: Implement token reveal
          break;
        case 4: // Combat
          manastrClient.log('⚔️ Phase 4: Executing combat rounds...');
          // TODO: Implement combat
          break;
        case 5: // Results
          manastrClient.log('🏆 Phase 5: Submitting results...');
          // TODO: Implement result submission
          break;
        case 6: // Loot Distribution
          manastrClient.log('💰 Phase 6: Distributing loot...');
          // TODO: Implement loot distribution
          break;
        default:
          manastrClient.log(`❌ Unknown phase: ${phaseIndex}`);
          return;
      }
      
      // Mark phase as completed
      setMatchFlow(prev => ({
        ...prev,
        currentPhase: Math.max(prev.currentPhase, phaseIndex + 1),
        phases: prev.phases.map((phase, idx) => 
          idx === phaseIndex ? { ...phase, completed: true } : phase
        )
      }));
      
      manastrClient.log(`✅ ${phaseName} completed successfully`);
      
    } catch (error) {
      manastrClient.log(`❌ ${phaseName} failed: ${error.message}`);
    }
  };

  return (
    <>
      <GlobalStyle />
      <AppContainer>
        <Header>
          <Title>🏛️ MANASTR Integration Dashboard</Title>
          <Subtitle>Revolutionary Zero-Coordination Gaming System</Subtitle>
          <Subtitle>Live Entity Monitoring & Match Flow Control</Subtitle>
        </Header>

        <DashboardLayout>
          {/* Service Entity Boxes */}
          <ServiceRow>
            <EntityBox>
              <EntityTitle>📡 Nostr Relay</EntityTitle>
              <EntityStatus>
                <StatusLabel>Status:</StatusLabel>
                <StatusValue style={{color: serviceStates.nostrRelay.status === 'Connected' ? '#22c55e' : '#ef4444'}}>
                  {serviceStates.nostrRelay.status}
                </StatusValue>
              </EntityStatus>
              <EntityStatus>
                <StatusLabel>Events Processed:</StatusLabel>
                <StatusValue>{serviceStates.nostrRelay.events}</StatusValue>
              </EntityStatus>
              <EntityStatus>
                <StatusLabel>Active Connections:</StatusLabel>
                <StatusValue>{serviceStates.nostrRelay.connections}</StatusValue>
              </EntityStatus>
            </EntityBox>

            <EntityBox>
              <EntityTitle>💰 Cashu Mint</EntityTitle>
              <EntityStatus>
                <StatusLabel>Health:</StatusLabel>
                <StatusValue style={{color: serviceStates.cashuMint.status === 'Connected' ? '#22c55e' : '#ef4444'}}>
                  {serviceStates.cashuMint.health || serviceStates.cashuMint.status}
                </StatusValue>
              </EntityStatus>
              <EntityStatus>
                <StatusLabel>Total Tokens:</StatusLabel>
                <StatusValue>{serviceStates.cashuMint.totalTokens}</StatusValue>
              </EntityStatus>
              <EntityStatus>
                <StatusLabel>Endpoint:</StatusLabel>
                <StatusValue>:3333</StatusValue>
              </EntityStatus>
            </EntityBox>

            <EntityBox>
              <EntityTitle>🎮 Game Engine Bot</EntityTitle>
              <EntityStatus>
                <StatusLabel>Status:</StatusLabel>
                <StatusValue style={{color: serviceStates.gameEngine.status === 'Nostr Ready' ? '#22c55e' : '#ef4444'}}>
                  {serviceStates.gameEngine.status}
                </StatusValue>
              </EntityStatus>
              <EntityStatus>
                <StatusLabel>Active Matches:</StatusLabel>
                <StatusValue>{serviceStates.gameEngine.matches}</StatusValue>
              </EntityStatus>
              <EntityStatus>
                <StatusLabel>Validations:</StatusLabel>
                <StatusValue>{serviceStates.gameEngine.validations}</StatusValue>
              </EntityStatus>
            </EntityBox>
          </ServiceRow>

          {/* Player Entity Boxes */}
          <PlayerRow>
            <EntityBox>
              <EntityTitle>🏛️ Alexi</EntityTitle>
              <EntityStatus>
                <StatusLabel>Balance:</StatusLabel>
                <StatusValue>{playerStates.alexi.balance} mana</StatusValue>
              </EntityStatus>
              <EntityStatus>
                <StatusLabel>Npub:</StatusLabel>
                <StatusValue>{playerStates.alexi.npub ? playerStates.alexi.npub.substring(0, 16) + '...' : 'Not created'}</StatusValue>
              </EntityStatus>
              <EntityStatus>
                <StatusLabel>Events Published:</StatusLabel>
                <StatusValue>{playerStates.alexi.eventsPublished}</StatusValue>
              </EntityStatus>
              <EntityStatus>
                <StatusLabel>Status:</StatusLabel>
                <StatusValue style={{color: playerStates.alexi.connected ? '#22c55e' : '#ef4444'}}>
                  {playerStates.alexi.connected ? 'Connected' : 'Not Created'}
                </StatusValue>
              </EntityStatus>
            </EntityBox>

            <EntityBox>
              <EntityTitle>🏛️ Boberto</EntityTitle>
              <EntityStatus>
                <StatusLabel>Balance:</StatusLabel>
                <StatusValue>{playerStates.boberto.balance} mana</StatusValue>
              </EntityStatus>
              <EntityStatus>
                <StatusLabel>Npub:</StatusLabel>
                <StatusValue>{playerStates.boberto.npub ? playerStates.boberto.npub.substring(0, 16) + '...' : 'Not created'}</StatusValue>
              </EntityStatus>
              <EntityStatus>
                <StatusLabel>Events Published:</StatusLabel>
                <StatusValue>{playerStates.boberto.eventsPublished}</StatusValue>
              </EntityStatus>
              <EntityStatus>
                <StatusLabel>Status:</StatusLabel>
                <StatusValue style={{color: playerStates.boberto.connected ? '#22c55e' : '#ef4444'}}>
                  {playerStates.boberto.connected ? 'Connected' : 'Not Created'}
                </StatusValue>
              </EntityStatus>
            </EntityBox>
          </PlayerRow>

          {/* Match Flow Controls */}
          <MatchControlsRow>
            <MatchFlowSection>
              <EntityTitle>🎯 7-Phase Match Flow Control</EntityTitle>
              <PhaseGrid>
                {matchFlow.phases.map((phase, idx) => (
                  <PhaseButton
                    key={idx}
                    onClick={() => executePhase(idx)}
                    className={`
                      ${phase.completed ? 'completed' : ''} 
                      ${idx === matchFlow.currentPhase ? 'active' : ''}
                    `}
                    disabled={idx > matchFlow.currentPhase + 1}
                  >
                    {phase.name}
                  </PhaseButton>
                ))}
              </PhaseGrid>
            </MatchFlowSection>
          </MatchControlsRow>

          {/* Live Activity Feed */}
          <ActivityFeed>
            <ActivityTitle>📡 Live Activity Stream</ActivityTitle>
            <ActivityLog>{logs}</ActivityLog>
          </ActivityFeed>
        </DashboardLayout>
      </AppContainer>
    </>
  );
}