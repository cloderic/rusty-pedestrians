mod antipodal_circle;
mod corridor;
mod empty_scenario;
mod scenario;

use crate::agents::Agents;
use crate::navmesh::Navmesh;
use serde::Deserialize;

pub use antipodal_circle::AntipodalCircleScenario;
pub use corridor::CorridorScenario;
pub use empty_scenario::EmptyScenario;
pub use scenario::Scenario;

#[derive(Clone, Copy, Debug, PartialEq, Deserialize)]
#[serde(tag = "scenario")]
enum Scenarii {
  AntipodalCircle(AntipodalCircleScenario),
  Corridor(CorridorScenario),
  Empty(EmptyScenario),
}

impl Scenario for Scenarii {
  fn generate(&self) -> (Agents, Navmesh) {
    match self {
      Scenarii::Corridor(s) => s.generate(),
      Scenarii::AntipodalCircle(s) => s.generate(),
      Scenarii::Empty(s) => s.generate(),
    }
  }
}

fn load_concrete_scenario(data: &str) -> Scenarii {
  serde_json::from_str::<Scenarii>(data)
    .unwrap_or_else(|_| Scenarii::Empty(EmptyScenario::default()))
}

pub fn load_scenario(data: &str) -> impl Scenario {
  load_concrete_scenario(data)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_load_scenario_empty() {
    match load_concrete_scenario("{}") {
      Scenarii::Empty(s) => assert_eq!(s, EmptyScenario::new()),
      _ => panic!("Expecting an EmptyScenario."),
    }
  }

  #[test]
  fn test_load_scenario_antipodal_circle() {
    match load_concrete_scenario(
      "{
        \"scenario\": \"AntipodalCircle\",
        \"agents_count\": 3,
        \"radius\": 4.0
      }",
    ) {
      Scenarii::AntipodalCircle(s) => assert_eq!(
        s,
        AntipodalCircleScenario {
          agents_count: 3,
          radius: 4.0,
        }
      ),
      _ => panic!("Expecting an AntipodalCircleScenario."),
    }
  }

  #[test]
  fn test_load_scenario_corridor() {
    match load_concrete_scenario(
      "{
        \"scenario\": \"Corridor\",
        \"agents_per_side_count\": 3,
        \"length\": 10,
        \"width\": 3.0
      }",
    ) {
      Scenarii::Corridor(s) => assert_eq!(
        s,
        CorridorScenario {
          agents_per_side_count: 3,
          length: 10.0,
          width: 3.0,
        }
      ),
      _ => panic!("Expecting an CorridorScenario."),
    }
  }

  #[test]
  fn test_load_scenario_corridor_default() {
    match load_concrete_scenario(
      "{
        \"scenario\": \"Corridor\"
      }",
    ) {
      Scenarii::Corridor(s) => assert_eq!(
        s,
        CorridorScenario {
          agents_per_side_count: 1,
          length: 10.0,
          width: 1.0,
        }
      ),
      _ => panic!("Expecting an CorridorScenario."),
    }
  }
}
