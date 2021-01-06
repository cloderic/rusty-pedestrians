use super::scenario::Scenario;
use crate::agents::Agents;
use crate::navmesh::{Navmesh, NavmeshBuilder};
use crate::vec2::Vec2;
use serde::Deserialize;

#[derive(Clone, Copy, Debug, PartialEq, Deserialize)]
pub struct EmptyScenario {}

impl EmptyScenario {
  pub fn new() -> Self {
    EmptyScenario {}
  }
}

impl Default for EmptyScenario {
  fn default() -> Self {
    EmptyScenario::new()
  }
}

impl Scenario for EmptyScenario {
  fn generate(&self) -> (Agents, Navmesh) {
    (
      Agents::new(),
      NavmeshBuilder::new()
        .add_cell(Vec2::new(0., 0.), Vec2::new(1., 0.), Vec2::new(1., 1.))
        .add_cell(Vec2::new(0., 0.), Vec2::new(0., 1.), Vec2::new(1., 1.))
        .build(),
    )
  }
}
