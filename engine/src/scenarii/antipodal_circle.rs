use super::scenario::Scenario;
use crate::agent::Agent;
use crate::agents::Agents;
use serde::Deserialize;

#[derive(Clone, Copy, Debug, PartialEq, Deserialize)]
pub struct AntipodalCircleScenario {
  agents_count: usize,
  radius: f64,
}

impl AntipodalCircleScenario {
  pub fn new() -> Self {
    AntipodalCircleScenario {
      agents_count: 2,
      radius: 5.0,
    }
  }

  #[allow(dead_code)]
  pub fn agents_count(mut self, agents_count: usize) -> Self {
    self.agents_count = agents_count;
    self
  }

  #[allow(dead_code)]
  pub fn radius(mut self, radius: f64) -> Self {
    self.radius = radius;
    self
  }
}

impl Default for AntipodalCircleScenario {
  fn default() -> Self {
    AntipodalCircleScenario::new()
  }
}

impl Scenario for AntipodalCircleScenario {
  fn generate(&self) -> Agents {
    (0..self.agents_count).fold(Agents::new(), |mut agents, i| {
      let angle = (i as f64) * 2.0 * std::f64::consts::PI / (self.agents_count as f64);
      let from_x = self.radius * angle.cos();
      let from_y = self.radius * angle.sin();
      let to_x = -from_x;
      let to_y = -from_y;
      agents.create_agent(Agent::new().position(from_x, from_y).target(to_x, to_y));
      agents
    })
  }
}
