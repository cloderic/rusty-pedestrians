use super::agent::Agent;
use super::vec2::Vec2;
use serde::Serialize;
use std::vec::Vec;

#[derive(Clone, Debug, PartialEq, Serialize)]
pub struct AgentDebugInfo {
  agent: Agent,
  orca_constraints: Vec<(Vec2, Vec2)>,
}

impl AgentDebugInfo {
  pub fn new() -> Self {
    AgentDebugInfo {
      agent: Agent::default(),
      orca_constraints: Vec::new(),
    }
  }

  pub fn agent(mut self, agent: Agent) -> Self {
    self.agent = agent;
    self
  }

  pub fn constraints(mut self, constraints: &[(Vec2, Vec2)]) -> Self {
    self.orca_constraints = constraints.to_vec();
    self
  }

  pub fn render(&self) -> String {
    serde_json::to_string(self).unwrap_or_else(|error| format!("{{\"error\": \"{}\"}}", error))
  }
}

impl Default for AgentDebugInfo {
  fn default() -> Self {
    AgentDebugInfo::new()
  }
}
