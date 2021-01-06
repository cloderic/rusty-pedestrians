import React, { useCallback, useState } from 'react';
import { Canvas } from 'react-three-fiber';
import Pedestrian from './Pedestrian';
import PedestrianDebugInfo from './PedestrianDebugInfo';
import Environment from './Environment';
import Stylesheet from './Stylesheet';
import styled from '@emotion/styled';
import { MapControls, softShadows } from '@react-three/drei';
import { DARK_GREY, GREY } from './Stylesheet';
import AccessibleEmoji from './AccessibleEmoji';
import Navmesh from './Navmesh';
import useSimulation from './useSimulation';

const SIMULATION_FREQUENCY = 60;

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

  const [paused, togglePaused] = useToggle(true);

  const {
    agents,
    navmeshObj,
    computeSimulationStep,
    selectedAgentDebugInfo,
    started,
    reset,
  } = useSimulation({
    universe,
    selectedAgentIdx,
    paused,
    simulationFrequency: SIMULATION_FREQUENCY,
  });

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
          {agents.map(({ index, position, direction, radius }) => (
            <Pedestrian
              key={index}
              position={position}
              direction={direction}
              radius={radius}
              onClick={setSelectedAgentIdx.bind(null, index)}
              selected={index === selectedAgentIdx}
            />
          ))}
          {selectedAgentDebugInfo ? (
            <PedestrianDebugInfo {...selectedAgentDebugInfo} />
          ) : null}
          <Navmesh color={DARK_GREY} navmeshObj={navmeshObj} />
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
          <button onClick={reset} name="reset" disabled={!started}>
            <AccessibleEmoji emoji="↩️" label="Reset" />
          </button>
        </ControlBar>
      </Container>
    </>
  );
};

export default App;
