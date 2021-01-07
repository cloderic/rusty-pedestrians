mod agent;
mod agent_debug_info;
mod agents;
mod apply_velocity_navigator;
mod look_where_you_go_navigator;
mod navmesh;
mod neighborhood;
mod orca;
mod reach_target_navigator;
mod scenarii;
mod utils;
mod vec2;

use agent_debug_info::AgentDebugInfo;
use agents::Agents;
use navmesh::Navmesh;
use scenarii::{load_scenario, EmptyScenario, Scenario};
use vec2::Vec2;

use wasm_bindgen::prelude::*;

use itertools::izip;

use std::io::Cursor;

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen]
extern "C" {
  fn alert(s: &str);
}

#[wasm_bindgen]
#[derive(Clone, Debug, PartialEq)]
pub struct RenderedAgent {
  position: Vec2,
}

#[wasm_bindgen]
pub struct Universe {
  agents: Agents,
  navmesh: Navmesh,
  scenario: Box<dyn Scenario>,
  last_dt: f64,
}

// Public methods w/ js bindings.
#[wasm_bindgen]
impl Universe {
  pub fn new() -> Universe {
    utils::set_panic_hook();

    let empty_scenario = EmptyScenario::new();
    let (agents, navmesh) = empty_scenario.generate();

    Universe {
      agents,
      navmesh,
      scenario: Box::new(empty_scenario),
      last_dt: 0.,
    }
  }
  pub fn load_scenario(&mut self, scenario_data: &str) {
    self.scenario = Box::new(load_scenario(scenario_data));
    let (agents, navmesh) = self.scenario.generate();
    self.agents = agents;
    self.navmesh = navmesh;
  }
  pub fn update(&mut self, dt: f64) {
    let neighborhoods = neighborhood::AgentNeighborhood::compute_agents_neighborhood(
      self.agents.get_positions(),
      self.agents.get_velocities(),
      self.agents.get_radii(),
    );
    self.agents.set_velocities(&orca::orca_navigator(
      self.agents.get_positions(),
      self.agents.get_directions(),
      &reach_target_navigator::reach_target_navigator(
        self.agents.get_positions(),
        self.agents.get_velocities(),
        self.agents.get_targets(),
        self.agents.get_desired_speeds(),
        self.agents.get_maximum_accelerations(),
        dt,
      ),
      self.agents.get_radii(),
      self.agents.get_maximum_speeds(),
      &neighborhoods,
      5.0,
      dt,
    ));

    self
      .agents
      .set_positions(&apply_velocity_navigator::apply_velocity_navigator(
        self.agents.get_positions(),
        self.agents.get_velocities(),
        dt,
      ));

    self
      .agents
      .set_directions(&look_where_you_go_navigator::look_where_you_go_navigator(
        self.agents.get_directions(),
        self.agents.get_velocities(),
      ));

    self.last_dt = dt;
  }
  pub fn render_agents(&self) -> Box<[f64]> {
    izip!(
      self.agents.get_positions().iter(),
      self.agents.get_directions().iter(),
      self.agents.get_velocities().iter(),
      self.agents.get_radii().iter()
    )
    .flat_map(|(p, d, v, &r)| vec![p.x(), p.y(), d.x(), d.y(), v.x(), v.y(), r])
    .collect::<Vec<f64>>()
    .into_boxed_slice()
  }
  pub fn render_navmesh(&self) -> String {
    let mut output = Vec::new();
    self
      .navmesh
      .render_to_obj(&mut Cursor::new(&mut output))
      .unwrap();
    String::from_utf8(output).unwrap()
  }
  pub fn render_debug_info(&self, idx_agent: usize) -> String {
    let mut debug_info = AgentDebugInfo::new().agent(self.agents.retrieve_agent(idx_agent));
    if self.last_dt > 0. {
      let neighborhood_slice = &neighborhood::AgentNeighborhood::compute_agents_neighborhood(
        &self.agents.get_positions(),
        &self.agents.get_velocities(),
        &self.agents.get_radii(),
      )[idx_agent..idx_agent + 1];
      let position_slice = &self.agents.get_positions()[idx_agent..idx_agent + 1];
      let velocity_slice = &self.agents.get_velocities()[idx_agent..idx_agent + 1];
      let radius_slice = &self.agents.get_radii()[idx_agent..idx_agent + 1];
      let target_slice = &self.agents.get_targets()[idx_agent..idx_agent + 1];
      let desired_speed_slice = &self.agents.get_desired_speeds()[idx_agent..idx_agent + 1];
      let maximum_acceleration_slice =
        &self.agents.get_maximum_accelerations()[idx_agent..idx_agent + 1];
      let constraints = orca::compute_constraints(
        position_slice,
        &reach_target_navigator::reach_target_navigator(
          position_slice,
          velocity_slice,
          target_slice,
          desired_speed_slice,
          maximum_acceleration_slice,
          self.last_dt,
        ),
        radius_slice,
        neighborhood_slice,
        10.0,
        self.last_dt,
      );
      debug_info = debug_info.constraints(&constraints[0])
    }

    debug_info.render()
  }
  pub fn count_agents(&self) -> usize {
    self.agents.len()
  }
}

impl Default for Universe {
  fn default() -> Universe {
    Universe::new()
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use approx::assert_relative_eq;

  #[test]
  pub fn test_simple_antipodal_scenario() {
    let mut universe = Universe::new();
    universe.load_scenario(
      "{
        \"scenario\": \"AntipodalCircle\",
        \"agents_count\": 4,
        \"radius\": 10.0
      }",
    );
    assert_eq!(universe.count_agents(), 4);
    universe
      .render_agents()
      .iter()
      .zip(vec![
        10.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.35, 0.0, 10.0, 0.0, -1.0, 0.0, 0.0, 0.35, -10.0, 0., 1.0,
        0.0, 0.0, 0.0, 0.35, 0.0, -10., 0.0, 1.0, 0.0, 0.0, 0.35,
      ])
      .for_each(|(value, expected)| assert_relative_eq!(value, &expected, epsilon = 0.0001));
    (0..100).for_each(|_| {
      universe.update(0.25);
      universe.render_debug_info(0);
    });
    let end_state = universe.render_agents();
    // All should have reached their target
    assert_relative_eq!(end_state[0], -10., epsilon = 0.0001);
    assert_relative_eq!(end_state[1], 0., epsilon = 0.0001);
    assert_relative_eq!(end_state[7], 0., epsilon = 0.0001);
    assert_relative_eq!(end_state[8], -10., epsilon = 0.0001);
    assert_relative_eq!(end_state[14], 10., epsilon = 0.0001);
    assert_relative_eq!(end_state[15], 0., epsilon = 0.0001);
    assert_relative_eq!(end_state[21], 0., epsilon = 0.0001);
    assert_relative_eq!(end_state[22], 10., epsilon = 0.0001);

    assert_eq!(
      universe.render_navmesh(),
      "v -15.000 -15.000 0.0\n\
      v 15.000 -15.000 0.0\n\
      v 15.000 15.000 0.0\n\
      v -15.000 15.000 0.0\n\
      f 1 2 3\n\
      f 1 3 4\n"
    );
  }
}
