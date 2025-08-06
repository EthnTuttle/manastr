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

const matrixRain = keyframes`
  0% { transform: translateY(-100vh); opacity: 1; }
  100% { transform: translateY(100vh); opacity: 0; }
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
  margin-bottom: 40px;
  position: relative;
`;

const Title = styled.h1`
  font-size: 4rem;
  margin: 0;
  background: linear-gradient(45deg, #00ffff, #8b5cf6, #00ffff);
  background-size: 200% 200%;
  animation: ${pulseGlow} 3s ease-in-out infinite;
  -webkit-background-clip: text;
  -webkit-text-fill-color: transparent;
  background-clip: text;
  position: relative;
  z-index: 2;
  
  @media (max-width: 768px) {
    font-size: 2.5rem;
  }
  
  &::before {
    content: '🏛️ MANASTR';
    position: absolute;
    top: 0;
    left: 0;
    width: 100%;
    height: 100%;
    background: transparent;
    -webkit-text-fill-color: rgba(0, 255, 255, 0.2);
    background-clip: text;
    -webkit-background-clip: text;
    z-index: -1;
    animation: ${matrixRain} 8s linear infinite;
  }
`;

const Subtitle = styled.div`
  font-size: 1.4rem;
  color: #7fffd4;
  margin: 10px 0;
  text-shadow: 0 0 10px rgba(127, 255, 212, 0.5);
  
  @media (max-width: 768px) {
    font-size: 1rem;
  }
`;

const StatusGrid = styled.div`
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(250px, 1fr));
  gap: 20px;
  margin-bottom: 40px;
`;

const StatusCard = styled.div`
  background: rgba(0, 0, 0, 0.8);
  border: 1px solid #00ffff;
  padding: 20px;
  text-align: center;
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
    opacity: 0.6;
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
    background: rgba(0, 17, 34, 0.9);
    box-shadow: 0 0 20px rgba(0, 255, 255, 0.3);
    transform: translateY(-2px);
    
    &::before {
      opacity: 1;
    }
  }
`;

const StatusTitle = styled.div`
  font-size: 1.1rem;
  color: #7fffd4;
  margin-bottom: 10px;
  text-transform: uppercase;
  letter-spacing: 2px;
`;

const StatusValue = styled.div`
  font-size: 1.8rem;
  font-weight: bold;
  color: #00ffff;
  text-shadow: 0 0 10px rgba(0, 255, 255, 0.8);
`;

const ActionsGrid = styled.div`
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(350px, 1fr));
  gap: 30px;
  margin-bottom: 40px;
`;

const ActionSection = styled.div`
  background: rgba(0, 17, 34, 0.8);
  border: 1px solid #00ffff;
  padding: 30px;
  position: relative;
  
  &::after {
    content: '';
    position: absolute;
    top: 10px;
    left: 10px;
    right: 10px;
    bottom: 10px;
    border: 1px solid rgba(0, 255, 255, 0.3);
    pointer-events: none;
  }
`;

const ActionTitle = styled.h2`
  font-size: 1.6rem;
  margin-bottom: 20px;
  color: #00ffff;
  text-transform: uppercase;
  letter-spacing: 3px;
  text-align: center;
  text-shadow: 0 0 15px rgba(0, 255, 255, 0.6);
`;

const InfoGrid = styled.div`
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 15px;
  margin-bottom: 20px;
  
  @media (max-width: 480px) {
    grid-template-columns: 1fr;
  }
`;

const InfoItem = styled.div`
  background: rgba(0, 255, 255, 0.1);
  padding: 15px;
  border: 1px solid rgba(0, 255, 255, 0.3);
`;

const InfoLabel = styled.div`
  font-size: 0.9rem;
  color: #7fffd4;
  margin-bottom: 5px;
  text-transform: uppercase;
  letter-spacing: 1px;
`;

const InfoValue = styled.div`
  font-family: 'Courier New', monospace;
  font-size: 0.9rem;
  color: #00ffff;
  word-break: break-all;
  text-shadow: 0 0 5px rgba(0, 255, 255, 0.5);
`;

const ActionButton = styled.button`
  background: linear-gradient(45deg, rgba(0, 255, 255, 0.2), rgba(139, 92, 246, 0.2));
  border: 1px solid #00ffff;
  color: #00ffff;
  padding: 12px 24px;
  margin: 8px;
  font-family: 'Courier New', monospace;
  font-size: 1rem;
  text-transform: uppercase;
  letter-spacing: 1px;
  cursor: pointer;
  transition: all 0.3s ease;
  position: relative;
  overflow: hidden;
  
  &:hover {
    background: linear-gradient(45deg, rgba(0, 255, 255, 0.4), rgba(139, 92, 246, 0.4));
    box-shadow: 0 0 20px rgba(0, 255, 255, 0.5);
    transform: translateY(-2px);
  }
  
  &:active {
    transform: translateY(0);
  }
  
  &:disabled {
    opacity: 0.5;
    cursor: not-allowed;
    transform: none;
    box-shadow: none;
  }
  
  &::before {
    content: '';
    position: absolute;
    top: 0;
    left: -100%;
    width: 100%;
    height: 100%;
    background: linear-gradient(90deg, transparent, rgba(255, 255, 255, 0.2), transparent);
    transition: left 0.5s;
  }
  
  &:hover::before {
    left: 100%;
  }
`;

const LogsSection = styled.div`
  background: rgba(0, 0, 0, 0.9);
  border: 1px solid #00ffff;
  padding: 20px;
  margin-top: 20px;
`;

const LogsTitle = styled.h3`
  margin-bottom: 15px;
  color: #7fffd4;
  text-transform: uppercase;
  letter-spacing: 2px;
`;

const LogOutput = styled.div`
  background: #000;
  color: #0f0;
  padding: 15px;
  border: 1px solid #0f0;
  font-family: 'Courier New', monospace;
  font-size: 0.9rem;
  max-height: 300px;
  overflow-y: auto;
  white-space: pre-wrap;
  line-height: 1.4;
  
  /* Custom scrollbar */
  &::-webkit-scrollbar {
    width: 8px;
  }
  
  &::-webkit-scrollbar-track {
    background: #001100;
  }
  
  &::-webkit-scrollbar-thumb {
    background: #0f0;
    border-radius: 4px;
  }
  
  &::-webkit-scrollbar-thumb:hover {
    background: #0a0;
  }
`;

export default function App() {
  const [manastrClient] = useState(new ManastrClient());
  const [status, setStatus] = useState({
    nostr: 'Disconnected',
    balance: '0 mana',
    activeGames: '0',
    gameEngine: 'Disconnected'
  });
  const [logs, setLogs] = useState('MANASTR system initialized...\nQuantum gaming protocols loaded.\nReady for revolutionary gaming.\n');

  useEffect(() => {
    // Set up client callbacks
    manastrClient.onStatusUpdate = setStatus;
    manastrClient.onLog = (message) => {
      setLogs(prev => prev + `[${new Date().toLocaleTimeString()}] ${message}\n`);
    };

    // Initialize the client
    manastrClient.initialize();

    return () => {
      manastrClient.disconnect();
    };
  }, [manastrClient]);

  return (
    <>
      <GlobalStyle />
      <AppContainer>
        <Header>
          <Title>🏛️ MANASTR</Title>
          <Subtitle>Revolutionary Zero-Coordination Gaming</Subtitle>
          <Subtitle>Quantum Nostr Client & Cashu Wallet</Subtitle>
        </Header>

          <StatusGrid>
            <StatusCard>
              <StatusTitle>NOSTR CONNECTION</StatusTitle>
              <StatusValue style={{ color: status.nostr === 'Connected' ? '#10b981' : '#64748b' }}>
                {status.nostr}
              </StatusValue>
            </StatusCard>
            <StatusCard>
              <StatusTitle>CASHU BALANCE</StatusTitle>
              <StatusValue>{status.balance}</StatusValue>
            </StatusCard>
            <StatusCard>
              <StatusTitle>ACTIVE GAMES</StatusTitle>
              <StatusValue>{status.activeGames}</StatusValue>
            </StatusCard>
            <StatusCard>
              <StatusTitle>GAME ENGINE</StatusTitle>
              <StatusValue style={{ color: status.gameEngine === 'Connected' ? '#10b981' : '#64748b' }}>
                {status.gameEngine}
              </StatusValue>
            </StatusCard>
          </StatusGrid>

          <ActionsGrid>
            <ActionSection>
              <ActionTitle>🔗 NOSTR CLIENT</ActionTitle>
              <InfoGrid>
                <InfoItem>
                  <InfoLabel>Public Key</InfoLabel>
                  <InfoValue id="nostr-pubkey">Not connected</InfoValue>
                </InfoItem>
                <InfoItem>
                  <InfoLabel>Relay</InfoLabel>
                  <InfoValue>ws://localhost:7777</InfoValue>
                </InfoItem>
              </InfoGrid>
              <ActionButton onClick={() => manastrClient.connectNostr()}>
                Connect to Nostr
              </ActionButton>
              <ActionButton onClick={() => manastrClient.disconnectNostr()}>
                Disconnect
              </ActionButton>
              <ActionButton onClick={() => manastrClient.postNote()}>
                Post Note
              </ActionButton>
            </ActionSection>

            <ActionSection>
              <ActionTitle>💰 CASHU WALLET</ActionTitle>
              <InfoGrid>
                <InfoItem>
                  <InfoLabel>Mint URL</InfoLabel>
                  <InfoValue>http://localhost:3333</InfoValue>
                </InfoItem>
                <InfoItem>
                  <InfoLabel>Total Proofs</InfoLabel>
                  <InfoValue id="proof-count">0</InfoValue>
                </InfoItem>
              </InfoGrid>
              <ActionButton onClick={() => manastrClient.connectMint()}>
                Connect to Mint
              </ActionButton>
              <ActionButton onClick={() => manastrClient.mintTokens()}>
                Mint 10 mana
              </ActionButton>
              <ActionButton onClick={() => manastrClient.checkBalance()}>
                Check Balance
              </ActionButton>
              <ActionButton onClick={() => manastrClient.showProofs()}>
                Show Proofs
              </ActionButton>
            </ActionSection>

            <ActionSection>
              <ActionTitle>🎮 GAME ENGINE</ActionTitle>
              <InfoGrid>
                <InfoItem>
                  <InfoLabel>Engine Status</InfoLabel>
                  <InfoValue id="engine-status">Not connected</InfoValue>
                </InfoItem>
                <InfoItem>
                  <InfoLabel>Match Count</InfoLabel>
                  <InfoValue id="match-count">0</InfoValue>
                </InfoItem>
              </InfoGrid>
              <ActionButton onClick={() => manastrClient.connectGameEngine()}>
                Connect Engine
              </ActionButton>
              <ActionButton onClick={() => manastrClient.createMatch()}>
                Create Match
              </ActionButton>
              <ActionButton onClick={() => manastrClient.listMatches()}>
                List Matches
              </ActionButton>
            </ActionSection>
          </ActionsGrid>

          <LogsSection>
            <LogsTitle>📜 QUANTUM ACTIVITY LOG</LogsTitle>
            <LogOutput>{logs}</LogOutput>
          </LogsSection>
      </AppContainer>
    </>
  );
}