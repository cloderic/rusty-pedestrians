import React, { useCallback, useState } from 'react';
import { Canvas } from 'react-three-fiber';
import Pedestrian from './Pedestrian';
import PedestrianDebugInfo from './PedestrianDebugInfo';
import Environment from './Environment';
import Stylesheet, { DARK_GREY, GREY, SELECT_THEME } from './Stylesheet';
import styled from '@emotion/styled';
import { MapControls, softShadows } from '@react-three/drei';
import AccessibleEmoji from './AccessibleEmoji';
import Select from 'react-select';
import Navmesh from './Navmesh';
import useSimulation from './hooks/useSimulation';
import SCENARII from './scenarii';

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
  display: flex;
  justify-content: flex-start;
  align-items: center;
  padding: 0.5rem;
  .buttons {
    flex 1 1;
    display: flex;
    justify-content: center;
    button {
      height: 2rem;
      line-height: 2rem;
      font-size: 2rem;
    }
  }
`;

const useToggle = (initialValue = false) => {
  const [value, setValue] = useState(initialValue);
  const toggle = useCallback(() => {
    setValue((v) => !v);
  }, []);
  return [value, toggle];
};

const SELECT_SCENARIO_OPTIONS = Object.keys(SCENARII).map((scenarioName) => ({
  label: scenarioName,
  value: SCENARII[scenarioName],
}));

const SELECT_SCENARIO_STYLES = {
  container: (provided) => ({
    ...provided,
    width: '250px',
    height: '2rem',
  }),
};

const App = ({ universe }) => {
  const [paused, togglePaused] = useToggle(true);

  const [selectedAgentIdx, setSelectedAgentIdx] = useState(null);
  const handleClearSelection = useCallback(() => {
    setSelectedAgentIdx(null);
  }, [setSelectedAgentIdx]);

  const [selectedScenario, setSelectedScenario] = useState(
    SELECT_SCENARIO_OPTIONS[0].value
  );
  const handleSelectedScenarioChange = useCallback(
    ({ value }) => {
      if (!paused) {
        togglePaused();
      }
      setSelectedScenario(value);
      return 'set-value';
    },
    [setSelectedScenario, paused, togglePaused]
  );

  const {
    agents,
    navmeshObj,
    computeSimulationStep,
    selectedAgentDebugInfo,
    started,
    restart,
  } = useSimulation({
    universe,
    scenario: selectedScenario,
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
          <div className="buttons">
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
            <button onClick={restart} name="restart" disabled={!started}>
              <AccessibleEmoji emoji="↩️" label="Restart" />
            </button>
          </div>
          <Select
            options={SELECT_SCENARIO_OPTIONS}
            defaultValue={SELECT_SCENARIO_OPTIONS[0]}
            onChange={handleSelectedScenarioChange}
            isClearable={false}
            menuPlacement="top"
            styles={SELECT_SCENARIO_STYLES}
            theme={SELECT_THEME}
          />
        </ControlBar>
      </Container>
    </>
  );
};

export default App;
