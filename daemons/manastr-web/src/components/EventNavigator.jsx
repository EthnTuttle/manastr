import React, { useMemo, useState, useEffect } from 'react';
import styled from 'styled-components';

const Container = styled.div`
  position: relative;
  background: rgba(0, 10, 20, 0.9);
  border: 1px solid #00ffff;
  padding: 16px 48px;
  overflow: hidden;
  width: 100%;
  max-width: 960px;
  margin: 0 auto;
  @media (max-width: 1024px) { max-width: 840px; }
  @media (max-width: 900px) { max-width: 720px; }
  @media (max-width: 768px) { max-width: 100%; padding: 12px 40px; }
`;

const Slides = styled.div`
  display: flex;
  transition: transform 300ms ease-out;
  will-change: transform;
`;

const Slide = styled.div`
  min-width: 100%;
  padding: 8px 12px;
`;

const Arrow = styled.button`
  position: absolute;
  top: 50%;
  transform: translateY(-50%);
  background: rgba(0, 255, 255, 0.12);
  border: 1px solid #00ffff;
  color: #00ffff;
  width: 36px;
  height: 36px;
  border-radius: 18px;
  cursor: pointer;
  &:hover { background: rgba(0, 255, 255, 0.2); }
`;

const ArrowLeft = styled(Arrow)` left: 8px; `;
const ArrowRight = styled(Arrow)` right: 8px; `;

const DotBar = styled.div`
  display: flex;
  gap: 6px;
  justify-content: center;
  margin-top: 10px;
`;
const Dot = styled.button`
  width: 8px; height: 8px; border-radius: 4px; border: 1px solid #00ffff;
  background: ${({ active }) => (active ? '#00ffff' : 'transparent')};
  opacity: ${({ active }) => (active ? 1 : 0.5)};
`;

const Card = styled.div`
  border: 1px solid #00ffff;
  padding: 14px;
  background: linear-gradient(135deg, rgba(0, 20, 30, 0.8), rgba(0, 8, 16, 0.8));
  width: 100%;
  box-sizing: border-box;
`;

const Title = styled.div`
  font-size: 1rem;
  color: #00ffff;
  text-transform: uppercase;
  letter-spacing: 1px;
  margin-bottom: 6px;
`;

const Meta = styled.div`
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(180px, 1fr));
  gap: 8px;
  font-family: 'Courier New', monospace;
  color: #7fffd4;
  margin-bottom: 8px;
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

export default function EventNavigator({ events }) {
  const [index, setIndex] = useState(0);
  const clampedEvents = useMemo(() => events?.slice(0, 20) || [], [events]);

  useEffect(() => {
    if (index >= clampedEvents.length) setIndex(0);
  }, [clampedEvents.length, index]);

  const go = (dir) => {
    if (!clampedEvents.length) return;
    setIndex((i) => (i + dir + clampedEvents.length) % clampedEvents.length);
  };

  return (
    <Container>
      <Slides style={{ transform: `translateX(-${index * 100}%)` }}>
        {clampedEvents.map((ev, i) => (
          <Slide key={ev.id || i}>
            <Card>
              <Title>{ev.title}</Title>
              <Meta>
                <KV><span>Kind</span><span>{ev.kind}</span></KV>
                <KV><span>Time</span><span>{ev.time}</span></KV>
                <KV><span>Event</span><span>{ev.idShort}</span></KV>
                <KV><span>From</span><span>{ev.pubkeyShort}</span></KV>
              </Meta>
              <Details>
                {ev.details?.map((d, idx) => (
                  <KV key={idx}><span>{d.label}</span><span>{d.value}</span></KV>
                ))}
              </Details>
            </Card>
          </Slide>
        ))}
      </Slides>
      <ArrowLeft onClick={() => go(-1)}>{'<'}</ArrowLeft>
      <ArrowRight onClick={() => go(1)}>{'>'}</ArrowRight>
      <DotBar>
        {clampedEvents.map((_, i) => (
          <Dot key={i} active={i === index} onClick={() => setIndex(i)} />
        ))}
      </DotBar>
    </Container>
  );
}

