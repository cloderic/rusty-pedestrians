use serde::Serialize;

use super::vec2::Vec2;

pub const DEFAULT_DESIRED_SPEED: f64 = 2.1;
pub const DEFAULT_MAXIMUM_SPEED: f64 = 3.0;
pub const DEFAULT_MAXIMUM_ACCELERATION: f64 = 3.0;
pub const DEFAULT_RADIUS: f64 = 0.35;

#[derive(Clone, Copy, Debug, PartialEq, Serialize)]
pub struct Agent {
  pub position: Vec2,
  pub velocity: Vec2,
  pub direction: Vec2,
  pub target: Vec2,
  pub desired_speed: f64,
  pub maximum_speed: f64,
  pub maximum_acceleration: f64,
  pub radius: f64,
}

impl Agent {
  pub fn new() -> Self {
    Agent {
      position: Vec2::new(0., 0.),
      velocity: Vec2::new(0., 0.),
      direction: Vec2::new(1., 0.),
      target: Vec2::new(0., 0.),
      desired_speed: DEFAULT_DESIRED_SPEED,
      maximum_speed: DEFAULT_MAXIMUM_SPEED,
      maximum_acceleration: DEFAULT_MAXIMUM_ACCELERATION,
      radius: DEFAULT_RADIUS,
    }
  }

  pub fn position(mut self, x: f64, y: f64) -> Self {
    self.position = Vec2::new(x, y);
    self
  }

  pub fn direction(mut self, x: f64, y: f64) -> Self {
    self.direction = Vec2::new(x, y);
    let norm = self.direction.norm();
    if norm < f64::EPSILON {
      self.direction = Vec2::new(1., 0.);
    } else {
      self.direction /= norm
    }
    self
  }

  pub fn velocity(mut self, x: f64, y: f64) -> Self {
    self.velocity = Vec2::new(x, y);
    self
  }

  pub fn target(mut self, x: f64, y: f64) -> Self {
    self.target = Vec2::new(x, y);
    self
  }

  pub fn desired_speed(mut self, s: f64) -> Self {
    self.desired_speed = s;
    self
  }

  pub fn maximum_speed(mut self, s: f64) -> Self {
    self.maximum_speed = s;
    self
  }

  pub fn maximum_acceleration(mut self, s: f64) -> Self {
    self.maximum_acceleration = s;
    self
  }

  pub fn radius(mut self, r: f64) -> Self {
    self.radius = r;
    self
  }
}

impl Default for Agent {
  fn default() -> Self {
    Agent::new()
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_default_direction() {
    assert_eq!(Agent::new().direction, Vec2::new(1.0, 0.0));
    assert_eq!(Agent::default().direction, Vec2::new(1.0, 0.0));
  }

  #[test]
  fn test_given_direction() {
    assert_eq!(
      Agent::new().direction(3.0, 2.0).direction,
      Vec2::new(3.0, 2.0).normalize()
    );
  }
}
