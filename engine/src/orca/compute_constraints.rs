use crate::neighborhood::AgentNeighborhood;
use crate::vec2::Vec2;

use itertools::izip;

pub fn compute_constraints(
  positions: &[Vec2],
  desired_velocities: &[Vec2],
  radii: &[f64],
  neighborhoods: &[AgentNeighborhood],
  time_horizon: f64,
  dt: f64,
) -> Vec<Vec<(Vec2, Vec2)>> {
  let inv_time_horizon = 1.0 / time_horizon;
  let inv_dt = 1.0 / dt;
  izip!(positions, desired_velocities, radii, neighborhoods)
    .map(
      move |(&position, &desired_velocity, &radius, &neighborhood)| {
        izip!(
          neighborhood.get_neighbors_positions(),
          neighborhood.get_neighbors_velocities(),
          neighborhood.get_neighbors_radii()
        )
        .map(
          move |(&neighbor_position, &neighbor_velocity, &neighbor_radius)| {
            let relative_position = neighbor_position - position;
            let relative_velocity = neighbor_velocity - desired_velocity;
            let sqr_dist = relative_position.sqr_norm();
            let combined_radii = radius + neighbor_radius;
            let sqr_combined_radii = combined_radii.powi(2);
            let constraint_dir;
            let u;

            if sqr_dist <= sqr_combined_radii {
              // In collision. find u such as desired_velocity + u brings us out of collision */
              let w = -inv_dt * relative_position - relative_velocity;
              let w_norm = w.norm();
              let unit_w = w / w_norm;

              constraint_dir = Vec2::new(-unit_w.y(), unit_w.x());
              u = (combined_radii * inv_dt - w_norm) * unit_w;
            } else {
              // No collision atm.

              // Compute the vector from cutoff center to relative velocity
              let w = -inv_time_horizon * relative_position - relative_velocity;
              let w_sqr_norm = w.sqr_norm();
              let dot_product_1 = w * relative_position;

              if dot_product_1 < 0. && dot_product_1.powi(2) > sqr_combined_radii * w_sqr_norm {
                // Project on cut-off circle.
                let w_norm = w_sqr_norm.sqrt();
                let unit_w = w / w_norm;

                constraint_dir = Vec2::new(-unit_w.y(), unit_w.x());
                u = (combined_radii * inv_time_horizon - w_norm) * unit_w;
              } else {
                /* Project on legs. */
                let leg = (sqr_dist - sqr_combined_radii).sqrt();

                if Vec2::det(relative_position, w) > 0.0 {
                  /* Project on left leg. */
                  constraint_dir = -Vec2::new(
                    relative_position.x() * leg - relative_position.y() * combined_radii,
                    relative_position.x() * combined_radii + relative_position.y() * leg,
                  ) / sqr_dist;
                } else {
                  /* Project on right leg. */
                  constraint_dir = Vec2::new(
                    relative_position.x() * leg + relative_position.y() * combined_radii,
                    -relative_position.x() * combined_radii + relative_position.y() * leg,
                  ) / sqr_dist;
                }

                let dot_product_2 = -relative_velocity * constraint_dir;
                u = dot_product_2 * constraint_dir + relative_velocity;
              }
            }
            (
              desired_velocity + 0.5 * u, // Take responsability for half of the avoidance.
              constraint_dir,
            )
          },
        )
        .collect()
      },
    )
    .collect()
}
