use super::scenario::Scenario;
use crate::agent::Agent;
use crate::agents::Agents;
use crate::navmesh::{Navmesh, NavmeshBuilder};
use crate::vec2::Vec2;
use serde::Deserialize;

#[derive(Clone, Copy, Debug, PartialEq, Deserialize)]
#[serde(default)]
pub struct CorridorScenario {
  pub agents_per_side_count: usize,
  pub length: f64,
  pub width: f64,
}

impl Default for CorridorScenario {
  fn default() -> Self {
    CorridorScenario {
      agents_per_side_count: 1,
      length: 10.,
      width: 1.,
    }
  }
}

impl Scenario for CorridorScenario {
  fn generate(&self) -> (Agents, Navmesh) {
    let h_width = self.width / 2.0;
    let h_length = self.length / 2.0;
    let agent_margin = self.width / (self.agents_per_side_count + 1) as f64;
    let length_margin = agent_margin;
    (
      [-1.0, 1.0].iter().fold(Agents::new(), |agents, &side| {
        (0..self.agents_per_side_count).fold(agents, |mut agents, i| {
          let from_x = h_length * side;
          let from_y = -h_width + (i + 1) as f64 * agent_margin;
          let to_x = -h_length * side;
          let to_y = from_y;
          agents.create_agent(
            Agent::new()
              .position(from_x, from_y)
              .direction(-side, 0.)
              .target(to_x, to_y),
          );
          agents
        })
      }),
      NavmeshBuilder::new()
        .add_cell(
          Vec2::new(-h_length - length_margin, -h_width),
          Vec2::new(h_length + length_margin, -h_width),
          Vec2::new(h_length + length_margin, h_width),
        )
        .add_cell(
          Vec2::new(-h_length - length_margin, -h_width),
          Vec2::new(-h_length - length_margin, h_width),
          Vec2::new(h_length + length_margin, h_width),
        )
        .build(),
    )
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_generate() {
    let s = CorridorScenario {
      agents_per_side_count: 3,
      length: 10.0,
      width: 3.0,
    };
    let (agents, navmesh) = s.generate();
    assert_eq!(agents.len(), 6);

    assert_eq!(navmesh.count_cells(), 2);
  }
}
