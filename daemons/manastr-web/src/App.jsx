import React, { useState, useEffect } from 'react';
import styled, { createGlobalStyle, keyframes } from 'styled-components';
import ManastrClient from './ManastrClient';
import MatchFlowEventStream from './components/EventCarousel.jsx';

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
    animation: ${scanlineAnimation} 6s linear infinite;
    z-index: 1;
    pointer-events: none;
    opacity: 0.4;
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
  grid-template-columns: 280px 1fr;
  gap: 20px;
  max-width: 1400px;
  margin: 0 auto;
  min-height: calc(100vh - 200px);
  
  @media (max-width: 1024px) {
    grid-template-columns: 1fr;
    max-width: 100%;
  }
`;

const Sidebar = styled.div`
  display: flex;
  flex-direction: column;
  gap: 15px;
  
  @media (max-width: 1024px) {
    display: grid;
    grid-template-columns: repeat(auto-fit, minmax(280px, 1fr));
    gap: 15px;
  }
`;

const MainContent = styled.div`
  display: flex;
  flex-direction: column;
  gap: 20px;
  min-height: 100%;
`;

const FeatureShowcase = styled.div`
  background: linear-gradient(135deg, rgba(0, 255, 255, 0.05), rgba(139, 92, 246, 0.05));
  border: 2px solid #00ffff;
  padding: 25px;
  position: relative;
  text-align: center;
  overflow: hidden;
  
  &::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: linear-gradient(45deg, transparent 48%, rgba(0, 255, 255, 0.1) 49%, rgba(0, 255, 255, 0.1) 51%, transparent 52%);
    background-size: 20px 20px;
    opacity: 0.3;
    pointer-events: none;
  }
  
  &::after {
    content: '';
    position: absolute;
    top: -2px;
    left: -2px;
    right: -2px;
    bottom: -2px;
    background: linear-gradient(45deg, #00ffff, transparent, #8b5cf6, transparent, #00ffff);
    background-size: 400% 400%;
    animation: ${pulseGlow} 4s ease-in-out infinite;
    z-index: -1;
    opacity: 0.5;
  }
`;

const EntityBox = styled.div`
  background: rgba(0, 17, 34, 0.95);
  border: 1px solid #00ffff;
  padding: 18px;
  position: relative;
  transition: all 0.3s ease;
  backdrop-filter: blur(5px);
  
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
  margin: 0 0 12px 0;
  color: #00ffff;
  font-size: 1rem;
  text-transform: uppercase;
  letter-spacing: 1.5px;
  text-align: center;
  text-shadow: 0 0 8px rgba(0, 255, 255, 0.5);
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

const ShowcaseTitle = styled.h2`
  font-size: 2.5rem;
  margin: 0 0 10px 0;
  background: linear-gradient(45deg, #00ffff, #8b5cf6, #00ffff);
  background-size: 200% 200%;
  animation: ${pulseGlow} 3s ease-in-out infinite;
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
  text-transform: uppercase;
  letter-spacing: 3px;
  
  @media (max-width: 768px) {
    font-size: 1.8rem;
    letter-spacing: 2px;
  }
`;

const ShowcaseSubtitle = styled.div`
  font-size: 1.1rem;
  color: #7fffd4;
  margin: 0 0 25px 0;
  text-shadow: 0 0 10px rgba(127, 255, 212, 0.5);
  letter-spacing: 1px;
`;

const PlayerStatsGrid = styled.div`
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 15px;
  margin: 20px 0;
  
  @media (max-width: 768px) {
    grid-template-columns: 1fr;
  }
`;

const ActivitySection = styled.div`
  display: flex;
  flex-direction: column;
  gap: 20px;
`;

const InfoIcon = styled.button`
  position: absolute;
  left: 0;
  top: 50%;
  transform: translateY(-50%);
  background: rgba(0, 255, 255, 0.1);
  border: 1px solid #00ffff;
  color: #00ffff;
  width: 32px;
  height: 32px;
  border-radius: 50%;
  cursor: pointer;
  font-size: 1rem;
  display: flex;
  align-items: center;
  justify-content: center;
  transition: all 0.2s ease;
  
  &:hover {
    background: rgba(0, 255, 255, 0.2);
    box-shadow: 0 0 10px rgba(0, 255, 255, 0.3);
  }
`;

const Modal = styled.div`
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.8);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  backdrop-filter: blur(5px);
`;

const ModalContent = styled.div`
  background: linear-gradient(135deg, rgba(0, 17, 34, 0.95), rgba(0, 30, 50, 0.95));
  border: 2px solid #00ffff;
  padding: 30px;
  max-width: 600px;
  max-height: 80vh;
  overflow-y: auto;
  position: relative;
  
  &::before {
    content: '';
    position: absolute;
    top: -2px;
    left: -2px;
    right: -2px;
    bottom: -2px;
    background: linear-gradient(45deg, #00ffff, transparent, #8b5cf6, transparent, #00ffff);
    background-size: 400% 400%;
    animation: ${pulseGlow} 4s ease-in-out infinite;
    z-index: -1;
    opacity: 0.3;
  }
`;

const ModalClose = styled.button`
  position: absolute;
  top: 15px;
  right: 15px;
  background: none;
  border: none;
  color: #00ffff;
  font-size: 1.5rem;
  cursor: pointer;
  width: 30px;
  height: 30px;
  display: flex;
  align-items: center;
  justify-content: center;
  
  &:hover {
    color: #ff4444;
  }
`;

const CompactPlayerCard = styled.div`
  background: rgba(0, 17, 34, 0.95);
  border: 1px solid #00ffff;
  padding: 12px;
  position: relative;
  transition: all 0.3s ease;
  backdrop-filter: blur(5px);
`;

const PlayerCardTitle = styled.div`
  color: #00ffff;
  font-size: 0.9rem;
  text-transform: uppercase;
  letter-spacing: 1px;
  text-align: center;
  margin-bottom: 8px;
  font-weight: bold;
`;

const PlayerCardStats = styled.div`
  display: flex;
  justify-content: space-between;
  font-family: 'Courier New', monospace;
  font-size: 0.75rem;
  color: #7fffd4;
  
  span {
    text-align: center;
    flex: 1;
  }
`;

const ControlBar = styled.div`
  display: flex;
  justify-content: center;
  gap: 8px;
  margin-top: 8px;
`;

const SmallButton = styled.button`
  background: rgba(0, 255, 255, 0.1);
  border: 1px solid #00ffff;
  color: #00ffff;
  padding: 8px 12px;
  font-family: 'Courier New', monospace;
  font-size: 0.8rem;
  text-transform: uppercase;
  cursor: pointer;
  transition: all 0.2s ease;
  line-height: 1;
  &:hover { background: rgba(0, 255, 255, 0.2); }
`;

const PhaseGrid = styled.div`
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(140px, 1fr));
  gap: 8px;
  margin: 15px 0;
`;

const PhaseButton = styled.button`
  background: linear-gradient(45deg, rgba(0, 255, 255, 0.15), rgba(139, 92, 246, 0.15));
  border: 1px solid #00ffff;
  color: #00ffff;
  padding: 10px 8px;
  font-family: 'Courier New', monospace;
  font-size: 0.75rem;
  text-transform: uppercase;
  cursor: pointer;
  transition: all 0.2s ease;
  position: relative;
  line-height: 1.2;
  
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
  const [showAbout, setShowAbout] = useState(false);
  
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

  const [matchStatus, setMatchStatus] = useState({
    id: '',
    phase: 'idle',
    challenger: '',
    acceptor: '',
    winner: '',
    wagerAmount: 0,
  });
  
  const [logs, setLogs] = useState('MANASTR Integration Test Dashboard\nReady for revolutionary gaming operations.\n');
  const [timeline, setTimeline] = useState([]);
  const [eventCards, setEventCards] = useState([]);

  useEffect(() => {
    // Set up client callbacks
    manastrClient.onStatusUpdate = (status) => {
      setServiceStates(prev => ({
        ...prev,
        nostrRelay: { 
          ...prev.nostrRelay, 
          status: status.nostr === 'Connected' ? 'Connected' : 'Disconnected',
          events: status.nostrEvents ?? prev.nostrRelay.events,
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
          matches: status.matches ?? prev.gameEngine.matches,
          validations: status.validations ?? prev.gameEngine.validations
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

    manastrClient.onMatchUpdate = (info) => {
      setMatchStatus({
        id: info.id || '',
        phase: info.phase,
        challenger: info.challenger || '',
        acceptor: info.acceptor || '',
        winner: info.winner || '',
        wagerAmount: info.wagerAmount || 0,
      });
      setTimeline(prev => [{ t: Date.now(), type: 'phase', data: info }, ...prev].slice(0, 200));
    };

    manastrClient.onGameEvent = (event) => {
      setTimeline(prev => [{ t: Date.now(), type: 'event', data: event }, ...prev].slice(0, 200));
      try {
        const content = typeof event.content === 'string' ? JSON.parse(event.content || '{}') : {};
        const idShort = (event.id || '').slice(0, 8) + '...';
        const pubkeyShort = (event.pubkey || '').slice(0, 8) + '...';
        const time = new Date((event.created_at || Math.floor(Date.now()/1000)) * 1000).toLocaleTimeString();
        const titles = {
          31000: '🎯 Match Challenge',
          31001: '🎲 Match Accepted',
          31002: '🔮 Token Reveal',
          31003: '⚔️ Combat Move',
          31004: '🎭 Move Revealed',
          31005: '🏆 Match Result',
          31006: '💰 Loot Distributed'
        };
        const title = titles[event.kind] || '🎮 Game Event';
        const details = [];
        if (content.wager_amount) details.push({ label: 'Wager', value: `${content.wager_amount} mana` });
        if (content.match_event_id || content.match_id) details.push({ label: 'Match', value: (content.match_event_id || content.match_id).slice(0,8)+'...' });
        if (content.round_number) details.push({ label: 'Round', value: content.round_number });
        if (content.calculated_winner) details.push({ label: 'Winner', value: content.calculated_winner.slice(0,8)+'...' });
        if (content.loot_cashu_token && event.kind === 31006) details.push({ label: 'Loot', value: 'Issued' });
        const card = { id: event.id, kind: event.kind, time, idShort, pubkeyShort, title, details };
        setEventCards((cards) => [card, ...cards].slice(0, 20));
      } catch (_) {}
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
          await manastrClient.createMatchChallenge();
          break;
        case 2: // Accept
          await manastrClient.acceptMatchChallenge();
          break;
        case 3: // Reveal Tokens
          await manastrClient.revealTokens();
          break;
        case 4: // Combat
          await manastrClient.executeCombat();
          break;
        case 5: // Results
          await manastrClient.submitResults();
          break;
        case 6: // Loot Distribution
          await manastrClient.distributeLoot();
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

  const runAll = async () => {
    try {
      await manastrClient.runFullFlow();
      setMatchFlow(prev => ({
        ...prev,
        currentPhase: prev.phases.length,
        phases: prev.phases.map(p => ({ ...p, completed: true }))
      }));
    } catch (e) {}
  };

  const reset = () => {
    manastrClient.resetMatch();
    setMatchFlow({
      currentPhase: 0,
      phases: matchFlow.phases.map(p => ({ ...p, completed: false }))
    });
    setLogs('MANASTR Integration Test Dashboard\nReady for revolutionary gaming operations.\n');
    setMatchStatus({ id: '', phase: 'idle', challenger: '', acceptor: '', winner: '', wagerAmount: 0 });
  };

  return (
    <>
      <GlobalStyle />
      <AppContainer>
        <Header>
          <div style={{position: 'relative', display: 'flex', alignItems: 'center', justifyContent: 'center', marginBottom: '20px'}}>
            <InfoIcon onClick={() => setShowAbout(true)}>
              ℹ️
            </InfoIcon>
            <ControlBar>
              <SmallButton onClick={runAll}>▶ Run All Phases</SmallButton>
              <SmallButton onClick={reset}>↻ Reset</SmallButton>
            </ControlBar>
          </div>
        </Header>

        <DashboardLayout>
          {/* Left Sidebar - System Daemons */}
          <Sidebar>
            <EntityBox>
              <EntityTitle>📡 Nostr Relay</EntityTitle>
              <EntityStatus>
                <StatusLabel>Status:</StatusLabel>
                <StatusValue style={{color: serviceStates.nostrRelay.status === 'Connected' ? '#22c55e' : '#ef4444'}}>
                  {serviceStates.nostrRelay.status}
                </StatusValue>
              </EntityStatus>
              <EntityStatus>
                <StatusLabel>Events:</StatusLabel>
                <StatusValue>{serviceStates.nostrRelay.events}</StatusValue>
              </EntityStatus>
              <EntityStatus>
                <StatusLabel>Connections:</StatusLabel>
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
                <StatusLabel>Tokens:</StatusLabel>
                <StatusValue>{serviceStates.cashuMint.totalTokens}</StatusValue>
              </EntityStatus>
              <EntityStatus>
                <StatusLabel>Endpoint:</StatusLabel>
                <StatusValue>:3333</StatusValue>
              </EntityStatus>
            </EntityBox>

            <EntityBox>
              <EntityTitle>🎮 Game Engine</EntityTitle>
              <EntityStatus>
                <StatusLabel>Status:</StatusLabel>
                <StatusValue style={{color: serviceStates.gameEngine.status === 'Nostr Ready' ? '#22c55e' : '#ef4444'}}>
                  {serviceStates.gameEngine.status}
                </StatusValue>
              </EntityStatus>
              <EntityStatus>
                <StatusLabel>Matches:</StatusLabel>
                <StatusValue>{serviceStates.gameEngine.matches}</StatusValue>
              </EntityStatus>
              <EntityStatus>
                <StatusLabel>Validations:</StatusLabel>
                <StatusValue>{serviceStates.gameEngine.validations}</StatusValue>
              </EntityStatus>
            </EntityBox>
          </Sidebar>

          {/* Main Content Area */}
          <MainContent>
            {/* Feature Showcase - Battle Engine */}
            <FeatureShowcase>
              
              {(playerStates.alexi.connected || playerStates.boberto.connected) && (
                <PlayerStatsGrid>
                  {playerStates.alexi.connected && (
                    <CompactPlayerCard style={{background: 'rgba(0, 255, 255, 0.08)', border: '1px solid #00ffff'}}>
                      <PlayerCardTitle>🏛️ Alexi</PlayerCardTitle>
                      <PlayerCardStats>
                        <span>{playerStates.alexi.balance} mana</span>
                        <span style={{color: '#22c55e'}}>Ready</span>
                        <span>{playerStates.alexi.eventsPublished} events</span>
                      </PlayerCardStats>
                    </CompactPlayerCard>
                  )}

                  {playerStates.boberto.connected && (
                    <CompactPlayerCard style={{background: 'rgba(139, 92, 246, 0.08)', border: '1px solid #8b5cf6'}}>
                      <PlayerCardTitle>🏛️ Boberto</PlayerCardTitle>
                      <PlayerCardStats>
                        <span>{playerStates.boberto.balance} mana</span>
                        <span style={{color: '#22c55e'}}>Ready</span>
                        <span>{playerStates.boberto.eventsPublished} events</span>
                      </PlayerCardStats>
                    </CompactPlayerCard>
                  )}
                </PlayerStatsGrid>
              )}
              
              <PhaseGrid>
                {matchFlow.phases.map((phase, idx) => (
                  <PhaseButton
                    key={idx}
                    onClick={() => executePhase(idx)}
                    className={`
                      ${phase.completed ? 'completed' : ''} 
                      ${idx === matchFlow.currentPhase ? 'active' : ''}
                    `}
                    disabled={idx > matchFlow.currentPhase}
                  >
                    {phase.name}
                  </PhaseButton>
                ))}
              </PhaseGrid>
              
              
              <div style={{marginTop: 20, padding: '15px', background: 'rgba(0, 0, 0, 0.4)', border: '1px solid rgba(0, 255, 255, 0.3)', fontSize: '0.85rem', color: '#7fffd4'}}>
                <div style={{display: 'grid', gridTemplateColumns: 'repeat(auto-fit, minmax(200px, 1fr))', gap: '8px'}}>
                  <div>Match ID: <span style={{color:'#00ffff'}}>{matchStatus.id ? matchStatus.id.substring(0,12)+'...' : 'No Active Match'}</span></div>
                  <div>Phase: <span style={{color:'#00ffff'}}>{matchStatus.phase}</span></div>
                  <div>Challenger: <span style={{color:'#00ffff'}}>{matchStatus.challenger ? matchStatus.challenger.substring(0,12)+'...' : '—'}</span></div>
                  <div>Acceptor: <span style={{color:'#00ffff'}}>{matchStatus.acceptor ? matchStatus.acceptor.substring(0,12)+'...' : '—'}</span></div>
                  <div>Winner: <span style={{color:'#22c55e'}}>{matchStatus.winner ? matchStatus.winner.substring(0,12)+'...' : '—'}</span></div>
                  <div>Wager: <span style={{color:'#00ffff'}}>{matchStatus.wagerAmount} mana</span></div>
                </div>
              </div>
            </FeatureShowcase>

            {/* Activity Stream & Events */}
            <ActivitySection>
              {/* Real-time Event Feed */}
              <EntityBox>
                <EntityTitle>🛰️ Event Stream</EntityTitle>
                <MatchFlowEventStream events={eventCards} matchStatus={matchStatus} playerStates={playerStates} />
              </EntityBox>

              {/* Visual Timeline */}
              <EntityBox>
                <EntityTitle>📡 System Timeline</EntityTitle>
                <div style={{display:'grid', gridTemplateColumns:'1fr', gap: '6px', maxHeight: 300, overflowY:'auto'}}>
                  {timeline.slice(0, 15).map((item, idx) => (
                    <div key={idx} style={{display:'flex', justifyContent:'space-between', fontFamily:'Courier New', fontSize: '0.85rem', padding: '4px 8px', background: idx % 2 === 0 ? 'rgba(0, 255, 255, 0.02)' : 'transparent'}}>
                      <span style={{color:'#7fffd4', minWidth: '70px'}}>{new Date(item.t).toLocaleTimeString()}</span>
                      <span style={{color: item.type === 'phase' ? '#22c55e' : '#00ffff', flex: 1, textAlign: 'center'}}>
                        {item.type === 'phase' ? `Phase → ${item.data.phase}` : `Event ${item.data.kind}`}
                      </span>
                      <span style={{color:'#8b5cf6', minWidth: '80px', textAlign: 'right'}}>{item.type === 'phase' ? (item.data.id?.slice(0,8) || '') : (item.data.id?.slice(0,8) || '')}</span>
                    </div>
                  ))}
                </div>
              </EntityBox>
            </ActivitySection>

            {/* Terminal Logs at Bottom */}
            <ActivityFeed>
              <ActivityTitle>💻 System Terminal Log</ActivityTitle>
              <ActivityLog>{logs}</ActivityLog>
            </ActivityFeed>
          </MainContent>
        </DashboardLayout>
        
        {showAbout && (
          <Modal onClick={() => setShowAbout(false)}>
            <ModalContent onClick={(e) => e.stopPropagation()}>
              <ModalClose onClick={() => setShowAbout(false)}>×</ModalClose>
              <div style={{color: '#00ffff', fontSize: '1.2rem', fontWeight: 'bold', marginBottom: '20px', textAlign: 'center'}}>
                🏛️ MANASTR - Revolutionary Gaming System
              </div>
              <div style={{color: '#7fffd4', lineHeight: 1.6, fontSize: '0.9rem'}}>
                <p><strong>Zero-Coordination Multiplayer Gaming</strong></p>
                <p>Manastr represents a revolutionary breakthrough in decentralized gaming where players control the entire match flow through cryptographically-secured Nostr events.</p>
                
                <p><strong>Key Innovations:</strong></p>
                <ul style={{paddingLeft: '20px', margin: '10px 0'}}>
                  <li>🎯 <strong>Player-Driven Matches:</strong> No central coordination required</li>
                  <li>🔒 <strong>Cryptographic Anti-Cheat:</strong> Commitment/reveal prevents all cheating</li>
                  <li>💰 <strong>Fair Economics:</strong> 95% of rewards go to players</li>
                  <li>🎮 <strong>Pure Validator Engine:</strong> Game engine cannot manipulate outcomes</li>
                  <li>🛰️ <strong>7 Nostr Event Types:</strong> Complete decentralized match lifecycle</li>
                  <li>🎲 <strong>Cashu Token Armies:</strong> True randomness from mint signatures</li>
                </ul>
                
                <p><strong>Architecture:</strong></p>
                <p>Built on Bitcoin principles with Cashu tokens for economics and Nostr events for communication. The game engine serves only as a validator - it cannot cheat, coordinate matches, or manipulate outcomes.</p>
                
                <p style={{textAlign: 'center', marginTop: '20px', fontSize: '0.8rem', opacity: 0.8}}>
                  🚀 The future of trustless multiplayer gaming
                </p>
              </div>
            </ModalContent>
          </Modal>
        )}
      </AppContainer>
    </>
  );
}