import React, { useMemo, useState, useEffect } from 'react';
import styled, { keyframes } from 'styled-components';

const dataStreamAnimation = keyframes`
  0% { opacity: 0; transform: translateY(10px); }
  100% { opacity: 1; transform: translateY(0); }
`;

const quantumGlow = keyframes`
  0%, 100% { box-shadow: 0 0 5px #00ffff; }
  50% { box-shadow: 0 0 15px #00ffff, 0 0 25px rgba(0, 255, 255, 0.5); }
`;

const StreamContainer = styled.div`
  background: linear-gradient(135deg, rgba(0, 0, 0, 0.95), rgba(0, 20, 30, 0.9));
  border: 1px solid #00ffff;
  padding: 20px;
  max-height: 500px;
  overflow-y: auto;
  position: relative;
  
  &::before {
    content: '';
    position: absolute;
    top: 0;
    left: 0;
    right: 0;
    bottom: 0;
    background: 
      linear-gradient(90deg, transparent 48%, rgba(0, 255, 255, 0.05) 49%, rgba(0, 255, 255, 0.05) 51%, transparent 52%),
      linear-gradient(0deg, transparent 48%, rgba(0, 255, 255, 0.03) 49%, rgba(0, 255, 255, 0.03) 51%, transparent 52%);
    background-size: 30px 30px, 20px 20px;
    pointer-events: none;
    opacity: 0.3;
  }
  
  &::-webkit-scrollbar {
    width: 8px;
  }
  
  &::-webkit-scrollbar-track {
    background: rgba(0, 0, 0, 0.3);
  }
  
  &::-webkit-scrollbar-thumb {
    background: linear-gradient(to bottom, #00ffff, #8b5cf6);
    border-radius: 4px;
  }
`;

const EventGrid = styled.div`
  display: grid;
  gap: 12px;
  grid-template-columns: 1fr;
`;

const EventItem = styled.div`
  animation: ${dataStreamAnimation} 0.4s ease-out;
  transform-origin: top;
`;

const Card = styled.div`
  border: 1px solid #00ffff;
  padding: 16px;
  background: linear-gradient(135deg, rgba(0, 30, 50, 0.85), rgba(0, 15, 25, 0.85));
  position: relative;
  transition: all 0.3s ease;
  
  &::before {
    content: '';
    position: absolute;
    left: 0;
    top: 0;
    bottom: 0;
    width: 3px;
    background: linear-gradient(to bottom, #00ffff, #8b5cf6);
    opacity: 0.8;
  }
  
  &:hover {
    background: linear-gradient(135deg, rgba(0, 40, 60, 0.9), rgba(0, 20, 35, 0.9));
    animation: ${quantumGlow} 2s ease-in-out infinite;
  }
`;

const Title = styled.div`
  font-size: 1.1rem;
  color: #00ffff;
  text-transform: uppercase;
  letter-spacing: 1.5px;
  margin-bottom: 10px;
  text-shadow: 0 0 8px rgba(0, 255, 255, 0.4);
  font-weight: bold;
`;

const Meta = styled.div`
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(150px, 1fr));
  gap: 6px;
  font-family: 'Courier New', monospace;
  color: #7fffd4;
  margin-bottom: 10px;
  font-size: 0.85rem;
`;

const KV = styled.div`
  display: flex;
  justify-content: space-between;
`;

const Details = styled.div`
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
  gap: 8px;
`;

const PlayerEventContainer = styled.div`
  display: grid;
  grid-template-columns: 1fr 1fr;
  gap: 20px;
  margin-bottom: 15px;
  
  @media (max-width: 768px) {
    grid-template-columns: 1fr;
    gap: 10px;
  }
`;

const PlayerColumn = styled.div`
  background: ${({ player }) => player === 'alexi' ? 'rgba(0, 255, 255, 0.05)' : 'rgba(139, 92, 246, 0.05)'};
  border: 1px solid ${({ player }) => player === 'alexi' ? '#00ffff' : '#8b5cf6'};
  border-radius: 4px;
  padding: 12px;
`;

const PlayerHeader = styled.div`
  color: ${({ player }) => player === 'alexi' ? '#00ffff' : '#8b5cf6'};
  font-family: 'Courier New', monospace;
  font-size: 0.9rem;
  font-weight: bold;
  margin-bottom: 8px;
  text-transform: uppercase;
  text-align: center;
  letter-spacing: 1px;
`;

const EventPhase = styled.div`
  background: rgba(0, 0, 0, 0.3);
  border: 1px solid rgba(0, 255, 255, 0.2);
  padding: 8px;
  margin: 4px 0;
  font-size: 0.8rem;
  color: #7fffd4;
  font-family: 'Courier New', monospace;
  
  &.completed {
    border-color: #22c55e;
    color: #22c55e;
    background: rgba(34, 197, 94, 0.1);
  }
  
  &.active {
    border-color: #fbbf24;
    color: #fbbf24;
    background: rgba(251, 191, 36, 0.1);
    animation: ${quantumGlow} 2s ease-in-out infinite;
  }
`;

export default function MatchFlowEventStream({ events, matchStatus, playerStates }) {
  const eventsByPlayer = useMemo(() => {
    const alexi = events?.filter(ev => 
      playerStates.alexi.npub && ev.pubkeyShort.includes(playerStates.alexi.npub.slice(0, 8))
    ) || [];
    const boberto = events?.filter(ev => 
      playerStates.boberto.npub && ev.pubkeyShort.includes(playerStates.boberto.npub.slice(0, 8))
    ) || [];
    return { alexi: alexi.slice(0, 5), boberto: boberto.slice(0, 5) };
  }, [events, playerStates]);

  const phases = [
    { name: 'Challenge Created', kind: 31000 },
    { name: 'Challenge Accepted', kind: 31001 },
    { name: 'Tokens Revealed', kind: 31002 },
    { name: 'Moves Committed', kind: 31003 },
    { name: 'Moves Revealed', kind: 31004 },
    { name: 'Results Submitted', kind: 31005 },
    { name: 'Loot Distributed', kind: 31006 }
  ];

  const getPhaseStatus = (phase, playerEvents) => {
    const hasEvent = playerEvents.some(ev => ev.kind === phase.kind);
    const isActive = matchStatus.phase === 'in_combat' && phase.kind >= 31003;
    return hasEvent ? 'completed' : isActive ? 'active' : '';
  };

  if (!events?.length) {
    return (
      <StreamContainer>
        <div style={{
          textAlign: 'center', 
          color: '#7fffd4', 
          padding: '30px 20px',
          fontFamily: 'Courier New',
          fontSize: '0.9rem',
          opacity: 0.7
        }}>
          🛰️ EVENT STREAM INITIALIZED<br/>
          <span style={{fontSize: '0.75rem', opacity: 0.6}}>Awaiting match flow events...</span>
        </div>
      </StreamContainer>
    );
  }

  return (
    <StreamContainer>
      <PlayerEventContainer>
        <PlayerColumn player="alexi">
          <PlayerHeader player="alexi">🏛️ Alexi Events</PlayerHeader>
          {phases.map((phase, idx) => {
            const status = getPhaseStatus(phase, eventsByPlayer.alexi);
            return (
              <EventPhase key={idx} className={status}>
                {phase.name} {status === 'completed' ? '✓' : status === 'active' ? '...' : ''}
              </EventPhase>
            );
          })}
        </PlayerColumn>
        
        <PlayerColumn player="boberto">
          <PlayerHeader player="boberto">🏛️ Boberto Events</PlayerHeader>
          {phases.map((phase, idx) => {
            const status = getPhaseStatus(phase, eventsByPlayer.boberto);
            return (
              <EventPhase key={idx} className={status}>
                {phase.name} {status === 'completed' ? '✓' : status === 'active' ? '...' : ''}
              </EventPhase>
            );
          })}
        </PlayerColumn>
      </PlayerEventContainer>
      
      {events.slice(0, 3).length > 0 && (
        <>
          <div style={{color: '#7fffd4', fontSize: '0.8rem', marginBottom: '8px', textAlign: 'center', opacity: 0.8}}>
            Recent Events
          </div>
          <EventGrid>
            {events.slice(0, 3).map((ev, i) => (
              <EventItem key={ev.id || i}>
                <Card>
                  <Title>{ev.title}</Title>
                  <Meta>
                    <KV><span>KIND</span><span>{ev.kind}</span></KV>
                    <KV><span>TIME</span><span>{ev.time}</span></KV>
                    <KV><span>FROM</span><span>{ev.pubkeyShort}</span></KV>
                  </Meta>
                </Card>
              </EventItem>
            ))}
          </EventGrid>
        </>
      )}
    </StreamContainer>
  );
}

