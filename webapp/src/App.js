import React, { useCallback, useEffect, useState } from 'react';
import { Canvas } from 'react-three-fiber';
import chunk from 'lodash.chunk';
import Pedestrian from './Pedestrian';
import PedestrianDebugInfo from './PedestrianDebugInfo';
import Environment from './Environment';
import Stylesheet from './Stylesheet';
import styled from '@emotion/styled';
import { MapControls, softShadows } from '@react-three/drei';
import { GREY } from './Stylesheet';
import AccessibleEmoji from './AccessibleEmoji';

const UPDATE_FREQUENCY = 60;

// Inject soft shadow shader
softShadows();

const Container = styled.div`
  height: 100vh;
  background: ${GREY};
  display: flex;
  flex-direction: column;
`;

const ControlBar = styled.div`
  font-size: 2rem;
  display: flex;
  justify-content: center;
`;

const useToggle = (initialValue = false) => {
  const [value, setValue] = useState(initialValue);
  const toggle = useCallback(() => {
    setValue((v) => !v);
  }, []);
  return [value, toggle];
};

const App = ({ universe }) => {
  const [selectedAgentIdx, setSelectedAgentIdx] = useState(null);
  const handleClearSelection = useCallback(() => {
    setSelectedAgentIdx(null);
  }, [setSelectedAgentIdx]);
  const [{ agents, agentDebugInfo }, setRenderResult] = useState({
    agents: [],
  });

  const render = useCallback(() => {
    const agents = chunk(universe.render(), 7).map(
      ([posX, posY, dirX, dirY, velX, velY, r], index) => ({
        index,
        position: { x: posX, y: posY },
        direction: { x: dirX, y: dirY },
        radius: r,
        handleClick: () => {
          setSelectedAgentIdx(index);
        },
      })
    );
    if (selectedAgentIdx != null) {
      setRenderResult({
        agents,
        agentDebugInfo: JSON.parse(
          universe.render_debug_info(selectedAgentIdx)
        ),
      });
    } else {
      setRenderResult({ agents });
    }
  }, [setRenderResult, universe, selectedAgentIdx, setSelectedAgentIdx]);

  const [universeResetted, setUniverseResetted] = useState(false);
  const resetUniverse = useCallback(() => {
    universe.reset();
    render();
    setUniverseResetted(true);
  }, [universe, render]);
  useEffect(() => {
    universe.reset();
  }, [universe]);

  const [paused, togglePaused] = useToggle(true);
  const computeSimulationStep = useCallback(() => {
    universe.update(1 / UPDATE_FREQUENCY);
    setUniverseResetted(false);
    render();
  }, [universe, render]);

  useEffect(() => {
    if (paused) {
      render();
    } else {
      const interval = setInterval(
        computeSimulationStep,
        1000 / UPDATE_FREQUENCY
      );
      return () => clearInterval(interval);
    }
  }, [universe, paused, computeSimulationStep, render]);

  return (
    <>
      <Stylesheet />
      <Container>
        <Canvas
          camera={{ position: [0, 80, 0], fov: 10 }}
          shadowMap={true}
          onPointerMissed={handleClearSelection}
        >
          <Environment color={GREY} />
          {agents.map(({ index, position, direction, radius, handleClick }) => (
            <Pedestrian
              key={index}
              position={position}
              direction={direction}
              radius={radius}
              onClick={handleClick}
              selected={index === selectedAgentIdx}
            />
          ))}
          {agentDebugInfo ? <PedestrianDebugInfo {...agentDebugInfo} /> : null}
          <mesh
            position={[0, 0, 0]}
            rotation={[-Math.PI / 2, 0, 0]}
            receiveShadow
          >
            <planeBufferGeometry args={[100, 100, 1000, 1000]} />
            <shadowMaterial transparent opacity={0.4} />
          </mesh>
          <MapControls />
        </Canvas>
        <ControlBar>
          <button onClick={togglePaused} name="toggle-play-pause">
            {paused ? (
              <AccessibleEmoji emoji="▶️" label="Play" />
            ) : (
              <AccessibleEmoji emoji="⏸" label="Pause" />
            )}
          </button>
          <button
            onClick={computeSimulationStep}
            name="single-step"
            disabled={!paused}
          >
            <AccessibleEmoji emoji="⏭" label="Single Step" />
          </button>
          <button
            onClick={resetUniverse}
            name="reset"
            disabled={universeResetted}
          >
            <AccessibleEmoji emoji="↩️" label="Restart" />
          </button>
        </ControlBar>
      </Container>
    </>
  );
};

export default App;
