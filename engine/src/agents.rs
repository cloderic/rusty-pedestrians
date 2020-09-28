use super::agent::Agent;
use super::vec2::Vec2;
use std::vec::Vec;

#[derive(Clone, Debug, PartialEq)]
pub struct Agents {
  positions: Vec<Vec2>,
  velocities: Vec<Vec2>,
  targets: Vec<Vec2>,
  desired_speeds: Vec<f64>,
  maximum_speeds: Vec<f64>,
  maximum_accelerations: Vec<f64>,
  radii: Vec<f64>,
}

impl Agents {
  pub fn new() -> Self {
    Agents {
      positions: Vec::new(),
      velocities: Vec::new(),
      targets: Vec::new(),
      desired_speeds: Vec::new(),
      maximum_speeds: Vec::new(),
      maximum_accelerations: Vec::new(),
      radii: Vec::new(),
    }
  }
  pub fn get_positions(&self) -> &[Vec2] {
    &self.positions
  }
  pub fn set_positions(&mut self, new_positions: &[Vec2]) {
    self.positions = new_positions.to_vec();
  }
  pub fn get_velocities(&self) -> &[Vec2] {
    &self.velocities
  }
  pub fn set_velocities(&mut self, new_velocities: &[Vec2]) {
    self.velocities = new_velocities.to_vec();
  }
  pub fn get_targets(&self) -> &[Vec2] {
    &self.targets
  }
  pub fn set_targets(&mut self, new_targets: &[Vec2]) {
    self.targets = new_targets.to_vec();
  }
  pub fn get_desired_speeds(&self) -> &[f64] {
    &self.desired_speeds
  }
  pub fn set_desired_speeds(&mut self, new_desired_speeds: &[f64]) {
    self.desired_speeds = new_desired_speeds.to_vec();
  }
  pub fn get_maximum_speeds(&self) -> &[f64] {
    &self.maximum_speeds
  }
  pub fn set_maximum_speeds(&mut self, new_maximum_speeds: &[f64]) {
    self.maximum_speeds = new_maximum_speeds.to_vec();
  }
  pub fn get_maximum_accelerations(&self) -> &[f64] {
    &self.maximum_accelerations
  }
  pub fn set_maximum_accelerations(&mut self, new_maximum_accelerations: &[f64]) {
    self.maximum_accelerations = new_maximum_accelerations.to_vec();
  }
  pub fn get_radii(&self) -> &[f64] {
    &self.radii
  }
  pub fn set_radii(&mut self, new_radii: &[f64]) {
    self.radii = new_radii.to_vec();
  }
  pub fn len(&self) -> usize {
    self.positions.len()
  }
  pub fn create_agent(&mut self, agent: Agent) {
    self.positions.push(agent.position);
    self.velocities.push(agent.velocity);
    self.targets.push(agent.target);
    self.desired_speeds.push(agent.desired_speed);
    self.maximum_speeds.push(agent.maximum_speed);
    self.maximum_accelerations.push(agent.maximum_acceleration);
    self.radii.push(agent.radius);
  }
  pub fn retrieve_agent(&self, idx_agent: usize) -> Agent {
    Agent {
      position: self.positions[idx_agent],
      velocity: self.velocities[idx_agent],
      target: self.targets[idx_agent],
      desired_speed: self.desired_speeds[idx_agent],
      maximum_speed: self.maximum_speeds[idx_agent],
      maximum_acceleration: self.maximum_accelerations[idx_agent],
      radius: self.radii[idx_agent],
    }
  }
}

impl Default for Agents {
  fn default() -> Self {
    Agents::new()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_new() {
    let agents = Agents::new();
    assert_eq!(agents.len(), 0);
  }

  #[test]
  fn test_create_agent() {
    let mut agents = Agents::new();
    let created_agent = Agent::new().position(2.0, 3.0).velocity(0., 0.);
    agents.create_agent(created_agent);
    assert_eq!(agents.len(), 1);
    itertools::assert_equal(agents.get_positions(), &vec![Vec2::new(2.0, 3.0)]);
    itertools::assert_equal(agents.get_velocities(), &vec![Vec2::new(0., 0.)]);
    assert_eq!(agents.retrieve_agent(0), created_agent);
  }
}
