import { useCallback, useEffect, useState } from 'react';
import chunk from 'lodash.chunk';

const useSimulation = ({
  universe,
  scenario,
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
    if (selectedAgentIdx != null && selectedAgentIdx < agents.length) {
      setSelectedAgentDebugInfo(
        JSON.parse(universe.render_debug_info(selectedAgentIdx))
      );
    } else {
      setSelectedAgentDebugInfo(null);
    }
  }, [setAgents, setSelectedAgentDebugInfo, universe, selectedAgentIdx]);

  // Simulation restart
  const [started, setStarted] = useState(false);
  const [navmeshObj, setNavmeshObj] = useState(false);
  const restart = useCallback(() => {
    const scenario_str = JSON.stringify(scenario);
    universe.load_scenario(scenario_str);
    renderAgents();
    setNavmeshObj(universe.render_navmesh());
    setStarted(false);
  }, [universe, renderAgents, scenario]);

  // Scenario changes effect, basically restart.
  useEffect(restart, [restart]);

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
    restart,
    computeSimulationStep,
  };
};

export default useSimulation;
