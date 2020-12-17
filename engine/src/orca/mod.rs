mod compute_constraints;
mod linear_program;

pub use compute_constraints::compute_constraints;

//use crate::log;
use crate::neighborhood::AgentNeighborhood;
use crate::vec2::Vec2;
use itertools::izip;

#[allow(clippy::too_many_arguments)]
pub fn orca_navigator(
  positions: &[Vec2],
  directions: &[Vec2],
  desired_velocities: &[Vec2],
  radii: &[f64],
  maximum_speeds: &[f64],
  neighborhoods: &[AgentNeighborhood],
  time_horizon: f64,
  dt: f64,
) -> Vec<Vec2> {
  izip!(
    compute_constraints(
      positions,
      desired_velocities,
      radii,
      neighborhoods,
      time_horizon,
      dt
    ),
    directions,
    desired_velocities,
    maximum_speeds,
  )
  .map(
    |(orca_constraints, &direction, &desired_velocity, &maximum_speed)| {
      let desired_speed = desired_velocity.norm();
      let desired_direction = if desired_speed < f64::EPSILON {
        direction
      } else {
        desired_velocity / desired_speed
      };

      // log!(
      //   "$ Finding valid velocity close to {} respecting {:#?}",
      //   desired_direction.normalize_to(desired_speed),
      //   orca_constraints
      // );

      match linear_program::solve_linear_program(
        &desired_direction,
        desired_speed,
        &orca_constraints,
        true,
      ) {
        Some(corrected_velocity) => {
          //log!("$$ 1st solve worked -> {}", corrected_velocity);
          corrected_velocity
        }
        // No solution, let's try to accelerate
        None => {
          match linear_program::solve_linear_program(
            &desired_direction,
            maximum_speed,
            &orca_constraints,
            false,
          ) {
            Some(corrected_velocity) => {
              //log!("$$ 2nd solve worked -> {}", corrected_velocity);
              corrected_velocity
            }
            // No solution, let's continue on our merry way
            None => {
              //log!("$$ No solution found");
              desired_direction.normalize_to(0.9 * desired_speed)
            }
          }
        }
      }
    },
  )
  .collect()
}

#[cfg(test)]
mod tests {
  use super::super::agent::Agent;
  use super::super::agents::Agents;
  use super::*;

  #[test]
  fn test_orca_navigator_no_movement() {
    let mut agents = Agents::new();
    agents.create_agent(Agent::new().position(1.0, 0.0));
    agents.create_agent(Agent::new().position(2.0, 0.0));

    let updated_velocities: Vec<Vec2> = orca_navigator(
      agents.get_positions(),
      agents.get_directions(),
      agents.get_velocities(),
      agents.get_radii(),
      agents.get_maximum_speeds(),
      &AgentNeighborhood::compute_agents_neighborhood(
        agents.get_positions(),
        agents.get_velocities(),
        agents.get_radii(),
      ),
      10.,
      0.5,
    );
    itertools::assert_equal(
      updated_velocities,
      vec![Vec2::new(0.0, 0.0), Vec2::new(0.0, 0.0)],
    )
  }

  #[test]
  fn test_orca_navigator_diverging_movement() {
    let mut agents = Agents::new();
    agents.create_agent(Agent::new().position(1.0, 0.0).velocity(0.0, -1.0));
    agents.create_agent(Agent::new().position(2.0, 0.0).velocity(0.0, 1.0));

    let updated_velocities: Vec<Vec2> = orca_navigator(
      agents.get_positions(),
      agents.get_directions(),
      agents.get_velocities(),
      agents.get_radii(),
      agents.get_maximum_speeds(),
      &AgentNeighborhood::compute_agents_neighborhood(
        agents.get_positions(),
        agents.get_velocities(),
        agents.get_radii(),
      ),
      10.,
      0.5,
    );
    itertools::assert_equal(
      updated_velocities,
      vec![Vec2::new(0.0, -1.0), Vec2::new(0.0, 1.0)],
    )
  }

  #[test]
  fn test_orca_navigator_converging_movement() {
    let mut agents = Agents::new();
    agents.create_agent(Agent::new().position(0.0, 0.0).velocity(1.0, 0.0));
    agents.create_agent(Agent::new().position(1.0, 0.0).velocity(-1.0, 0.0));

    orca_navigator(
      agents.get_positions(),
      agents.get_directions(),
      agents.get_velocities(),
      agents.get_radii(),
      agents.get_maximum_speeds(),
      &AgentNeighborhood::compute_agents_neighborhood(
        agents.get_positions(),
        agents.get_velocities(),
        agents.get_radii(),
      ),
      10.,
      0.5,
    );

    // No assertion, let's just make sure everything works fine
  }
}
