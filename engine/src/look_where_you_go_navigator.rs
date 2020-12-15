use super::vec2::Vec2;
use itertools::izip;

pub fn look_where_you_go_navigator(directions: &[Vec2], velocities: &[Vec2]) -> Vec<Vec2> {
  izip!(directions, velocities)
    .map(move |(&direction, &velocity)| {
      let speed = velocity.norm();
      if speed < f64::EPSILON {
        direction
      } else {
        velocity / speed
      }
    })
    .collect()
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_simple_look_where_you_go_navigator() {
    let d = vec![Vec2::new(1.0, 0.0), Vec2::new(1.0, 1.0).normalize()];
    let v = vec![Vec2::new(-1.0, 1.0), Vec2::new(0.0, 0.0)];
    let updated_d: Vec<Vec2> = look_where_you_go_navigator(&d, &v);
    assert_eq!(updated_d[0], v[0].normalize());
    assert_eq!(updated_d[1], d[1]);
  }
}
