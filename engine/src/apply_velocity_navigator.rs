use super::vec2::Vec2;
use itertools::izip;

pub fn apply_velocity_navigator(positions: &[Vec2], velocities: &[Vec2], dt: f64) -> Vec<Vec2> {
  izip!(positions, velocities)
    .map(move |(&position, &velocity)| position + velocity * dt)
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_simple_apply_velocity() {
    let p = vec![Vec2::new(1.0, 2.0), Vec2::new(3.0, 4.0)];
    let v = vec![Vec2::new(-1.0, 1.0), Vec2::new(1.0, -1.0)];
    let updated_p: Vec<Vec2> = apply_velocity_navigator(&p, &v, 0.5);
    assert_eq!(updated_p[0], Vec2::new(0.5, 2.5));
  }
}
