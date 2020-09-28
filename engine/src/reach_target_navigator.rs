use super::vec2::Vec2;
use itertools::izip;

pub fn reach_target_navigator(
  positions: &[Vec2],
  velocities: &[Vec2],
  targets: &[Vec2],
  desired_speeds: &[f64],
  maximum_accelerations: &[f64],
  dt: f64,
) -> Vec<Vec2> {
  izip!(
    positions,
    velocities,
    targets,
    desired_speeds,
    maximum_accelerations
  )
  .map(
    move |(&position, &velocity, &target, &desired_speed, &maximum_acceleration)| {
      let desired_velocity = (target - position).cap_norm(desired_speed);
      let desired_acceleration = (desired_velocity - velocity) / dt;
      let clamped_acceleration = desired_acceleration.cap_norm(maximum_acceleration);

      velocity + dt * clamped_acceleration
    },
  )
  .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_reach_target() {
    let p = vec![
      Vec2::new(0.0, 1.0),
      Vec2::new(0.0, 2.0),
      Vec2::new(0.0, 3.0),
    ];
    let v = vec![
      Vec2::new(0.0, 0.0),
      Vec2::new(1.0, 0.0),
      Vec2::new(4.0, 0.0),
    ];
    let t = vec![
      Vec2::new(10.0, 1.0),
      Vec2::new(10.0, 2.0),
      Vec2::new(10.0, 3.0),
    ];
    let ds = vec![2.0, 2.0, 2.0];
    let ma = vec![10.0, 1.0, 1.0];
    let updated_v: Vec<Vec2> = reach_target_navigator(&p, &v, &t, &ds, &ma, 0.5);
    itertools::assert_equal(
      updated_v,
      vec![
        Vec2::new(2.0, 0.0),
        Vec2::new(1.5, 0.0),
        Vec2::new(3.5, 0.0),
      ],
    )
  }
  #[test]
  fn test_reach_target_desired_speed_reached() {
    let p = vec![Vec2::new(1.0, 0.0)];
    let v = vec![Vec2::new(0.0, 2.0)];
    let t = vec![Vec2::new(1.0, 10.0)];
    let ds = vec![2.0];
    let ma = vec![10.0];
    let updated_v: Vec<Vec2> = reach_target_navigator(&p, &v, &t, &ds, &ma, 0.5);
    itertools::assert_equal(updated_v, vec![Vec2::new(0.0, 2.0)])
  }
}
