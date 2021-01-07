use super::scenario::Scenario;
use crate::agent::Agent;
use crate::agents::Agents;
use crate::navmesh::{Navmesh, NavmeshBuilder};
use crate::vec2::Vec2;
use serde::Deserialize;

#[derive(Clone, Copy, Debug, PartialEq, Deserialize)]
#[serde(default)]
pub struct AntipodalCircleScenario {
  pub agents_count: usize,
  pub radius: f64,
}

impl Default for AntipodalCircleScenario {
  fn default() -> Self {
    AntipodalCircleScenario {
      agents_count: 2,
      radius: 5.0,
    }
  }
}

impl Scenario for AntipodalCircleScenario {
  fn generate(&self) -> (Agents, Navmesh) {
    let s = self.radius * 1.5;
    (
      (0..self.agents_count).fold(Agents::new(), |mut agents, i| {
        let angle = (i as f64) * 2.0 * std::f64::consts::PI / (self.agents_count as f64);
        let from_x = self.radius * angle.cos();
        let from_y = self.radius * angle.sin();
        let to_x = -from_x;
        let to_y = -from_y;
        agents.create_agent(
          Agent::new()
            .position(from_x, from_y)
            .direction(to_x - from_x, to_y - from_y)
            .target(to_x, to_y),
        );
        agents
      }),
      NavmeshBuilder::new()
        .add_cell(Vec2::new(-s, -s), Vec2::new(s, -s), Vec2::new(s, s))
        .add_cell(Vec2::new(-s, -s), Vec2::new(-s, s), Vec2::new(s, s))
        .build(),
    )
  }
}
