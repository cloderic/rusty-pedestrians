use super::scenario::Scenario;
use crate::agents::Agents;
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
  fn generate(&self) -> Agents {
    Agents::new()
  }
}
