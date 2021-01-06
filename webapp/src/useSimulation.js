import { useCallback, useEffect, useState } from 'react';
import chunk from 'lodash.chunk';

const useSimulation = ({
  universe,
  selectedAgentIdx,
  paused,
  simulationFrequency,
}) => {
  // Agents rendering (including debug info for the selected agent)
  const [agents, setAgents] = useState([]);
  const [selectedAgentDebugInfo, setSelectedAgentDebugInfo] = useState(null);

  const renderAgents = useCallback(() => {
    const agents = chunk(universe.render_agents(), 7).map(
      ([posX, posY, dirX, dirY, velX, velY, r], index) => ({
        index,
        position: { x: posX, y: posY },
        direction: { x: dirX, y: dirY },
        radius: r,
      })
    );
    setAgents(agents);
    if (selectedAgentIdx != null) {
      setSelectedAgentDebugInfo(
        JSON.parse(universe.render_debug_info(selectedAgentIdx))
      );
    }
  }, [setAgents, setSelectedAgentDebugInfo, universe, selectedAgentIdx]);

  // Rendered navmesh
  const [navmeshObj, setNavmeshObj] = useState('');

  // Simulation restart
  const [started, setStarted] = useState(false);
  const reset = useCallback(() => {
    universe.reset();
    renderAgents();
    setStarted(false);
  }, [universe, renderAgents]);

  // Simulation load effect
  useEffect(() => {
    universe.reset();
    setNavmeshObj(universe.render_navmesh());
  }, [universe, setNavmeshObj]);

  // Simulation step effect
  const computeSimulationStep = useCallback(() => {
    universe.update(1 / simulationFrequency);
    setStarted(true);
    renderAgents();
  }, [universe, renderAgents, setStarted, simulationFrequency]);

  useEffect(() => {
    if (paused) {
      renderAgents();
    } else {
      const interval = setInterval(
        computeSimulationStep,
        1000 / simulationFrequency
      );
      return () => clearInterval(interval);
    }
  }, [
    universe,
    paused,
    renderAgents,
    computeSimulationStep,
    simulationFrequency,
  ]);

  return {
    agents,
    selectedAgentDebugInfo,
    navmeshObj,
    started,
    paused,
    reset,
    computeSimulationStep,
  };
};

export default useSimulation;
