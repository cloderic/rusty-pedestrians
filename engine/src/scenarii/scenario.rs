use crate::agents::Agents;

pub trait Scenario {
  fn generate(&self) -> Agents;
}
