use super::vec2::Vec2;

use itertools::izip;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct AgentNeighborhood {
  neighbors_len: usize,
  neighbors_positions: [Vec2; 10],
  neighbors_velocities: [Vec2; 10],
  neighbors_radii: [f64; 10],
  neighbors_distances: [f64; 10],
}

impl AgentNeighborhood {
  fn new() -> Self {
    AgentNeighborhood {
      neighbors_len: 0,
      neighbors_positions: [Vec2::new(0., 0.); 10],
      neighbors_velocities: [Vec2::new(0., 0.); 10],
      neighbors_radii: [super::agent::DEFAULT_RADIUS; 10],
      neighbors_distances: [0.0; 10],
    }
  }
  #[allow(dead_code)]
  pub fn neighbors_len(&self) -> usize {
    self.neighbors_len
  }
  pub fn get_neighbors_positions(&self) -> &[Vec2] {
    &self.neighbors_positions[0..self.neighbors_len]
  }
  pub fn get_neighbors_velocities(&self) -> &[Vec2] {
    &self.neighbors_velocities[0..self.neighbors_len]
  }
  pub fn get_neighbors_radii(&self) -> &[f64] {
    &self.neighbors_radii[0..self.neighbors_len]
  }
  #[allow(dead_code)]
  pub fn get_neighbors_distances(&self) -> &[f64] {
    &self.neighbors_distances[0..self.neighbors_len]
  }
  pub fn compute_agents_neighborhood(
    agent_positions: &[Vec2],
    agent_velocities: &[Vec2],
    agent_radii: &[f64],
  ) -> Vec<Self> {
    agent_positions
      .iter()
      .map(|&agent_position| {
        let mut agent_neighborhood = AgentNeighborhood::new();

        // Retrieve all the neighbot agents and compute their distance to the current agent
        let mut neighbor_agents = izip!(
          agent_positions.iter(),
          agent_velocities.iter(),
          agent_radii.iter()
        )
        .map(
          |(&neighbor_position, &neighbor_velocity, &neighbor_radius)| {
            (
              neighbor_position,
              neighbor_velocity,
              neighbor_radius,
              (neighbor_position - agent_position).norm(),
            )
          },
        )
        .collect::<Vec<(Vec2, Vec2, f64, f64)>>();

        // Sort by distance
        neighbor_agents.sort_by(|a, b| a.3.partial_cmp(&b.3).unwrap());

        izip!(
          agent_neighborhood.neighbors_positions.iter_mut(),
          agent_neighborhood.neighbors_velocities.iter_mut(),
          agent_neighborhood.neighbors_radii.iter_mut(),
          agent_neighborhood.neighbors_distances.iter_mut(),
          neighbor_agents.iter().skip(1).take(10)
        )
        .for_each(|(position, velocity, radius, distance, &input)| {
          *position = input.0;
          *velocity = input.1;
          *radius = input.2;
          *distance = input.3;
        });

        agent_neighborhood.neighbors_len = std::cmp::min(neighbor_agents.len() - 1, 10);
        agent_neighborhood
      })
      .collect()
  }
}

#[cfg(test)]
mod tests {
  use super::super::agent::Agent;
  use super::super::agents::Agents;
  use super::*;

  #[test]
  fn test_compute_agents_neighborhood() {
    let mut agents = Agents::new();
    agents.create_agent(Agent::new().position(1.0, 1.0));
    agents.create_agent(Agent::new().position(-2.0, 2.0));
    agents.create_agent(Agent::new().position(-3.0, -3.0));
    agents.create_agent(Agent::new().position(4.0, -4.0));
    assert_eq!(agents.len(), 4);

    let agents_neighborhood = AgentNeighborhood::compute_agents_neighborhood(
      agents.get_positions(),
      agents.get_velocities(),
      agents.get_radii(),
    );
    assert_eq!(agents_neighborhood[0].neighbors_len(), 3);
    assert_eq!(agents_neighborhood[1].neighbors_len(), 3);
    assert_eq!(agents_neighborhood[2].neighbors_len(), 3);
    assert_eq!(agents_neighborhood[3].neighbors_len(), 3);

    itertools::assert_equal(
      agents_neighborhood[0].get_neighbors_positions().iter(),
      vec![
        Vec2::new(-2.0, 2.0),
        Vec2::new(-3.0, -3.0),
        Vec2::new(4.0, -4.0),
      ]
      .iter(),
    );
    itertools::assert_equal(
      agents_neighborhood[0].get_neighbors_radii().iter(),
      vec![0.35, 0.35, 0.35].iter(),
    );
  }
}
