use crate::agents::Agents;
use crate::navmesh::Navmesh;

pub trait Scenario {
  fn generate(&self) -> (Agents, Navmesh);
}
