use crate::vec2::Vec2;

const EPSILON: f64 = 0.000001;

fn is_vector_belonging_to_half_plane(v: &Vec2, (origin, direction): &(Vec2, Vec2)) -> bool {
  Vec2::det(*direction, *origin - *v) > 0.
}

pub fn solve_linear_program_step(
  obj_dir: &Vec2,                // The objective direction
  obj_max_norm: f64,             // The objective maximum norm
  (l_ori, l_dir): &(Vec2, Vec2), // The half plane boundary origin and direction unit vector
  half_planes: &[(Vec2, Vec2)],  // The other half planes
  maximize_norm: bool,
) -> Option<Vec2> {
  debug_assert!(obj_max_norm >= 0.);
  debug_assert!(
    (obj_dir.sqr_norm() - 1.0).abs() < EPSILON,
    "The objective direction, {}, should be normalized, got a norm of {}!",
    obj_dir,
    obj_dir.norm()
  );
  // `obj` is on the right side of the line we need to make sure we can find an vector on the left
  // side that is not longer than the objective.
  //
  //
  // Let's find the segment of the line where obj doesn't grow.
  //
  // || obj || = || l_ori + t * l_dir ||
  // 0 = || l_ori + t * l_dir || - || obj ||
  // 0 = || l_ori + t * l_dir ||^2 - || obj ||^2
  //   = (l_ori.x + t * l_dir.x)^2 + (l_ori.y + t * l_dir.y)^2 - || obj ||^2
  //   =   l_ori.x^2 + 2*l_ori.x*t*l_dir.x + t^2*l_dir.x^2
  //     + l_ori.y^2 + 2*l_ori.y*t*l_dir.y + t^2*l_dir.y^2
  //     - || obj ||^2
  //   =   (l_dir.x^2 + l_dir.y^2) * t^2
  //     + (2*l_ori.x*l_dir.x + 2*l_ori.y*l_dir.y) * t
  //     + l_ori.x^2 + l_ori.y^2 - || obj ||^2
  //   =   || l_dir ||^2 * t^2
  //     + 2 * l_ori . l_dir * t
  //     + || l_ori ||^2 - || obj ||^2
  //
  // We can solve this polynom to find out the portion of line that can be used.

  //let poly_a = 1.0;
  let poly_b = 2.0 * *l_ori * *l_dir;
  let poly_c = l_ori.sqr_norm() - obj_max_norm.powi(2);
  let poly_det = poly_b.powi(2) - 4.0 /* * poly_a */ * poly_c;

  if poly_det < 0.0 {
    // No intersections, no solutions exits
    return None;
  }

  let poly_det_sqrt = poly_det.sqrt();

  match half_planes.iter().try_fold(
    (
      (-poly_b - poly_det_sqrt) / (2.0/* * poly_a */),
      (-poly_b + poly_det_sqrt) / (2.0/* * poly_a */),
    ),
    |(mut t_left, mut t_right), (h_ori, h_dir)| {
      // We are looking for intersection betwen h and l called i.
      // h_dir_perp being the vector perpendicular to h_dir, we know that
      // 0 = h_dir_perp . (i - h_ori)
      // Because i belongs to l we can define it as i = || l_ori + t * l_dir
      // We then have
      // 0 = h_dir_perp . (|| l_ori + t * l_dir - h_ori)
      // t = (-h_dir_perp . (|| l_ori - h_ori)) / (h_dir_perp . l_dir)
      // t = det(-h_dir, || l_ori - h_ori) / det(h_dir, l_dir)
      //
      // If the denominator is 0 then l and h are parallel
      let t_numerator = Vec2::det(-*h_dir, *l_ori - *h_ori);
      let t_denominator = Vec2::det(*h_dir, *l_dir);

      if t_denominator.abs() <= EPSILON {
        // the line runs (almost) parallel to the constraint
        if t_numerator < 0. {
          // the line is outside of the constraint, there are no solutions
          None
        } else {
          // the line is inside the constraint, no intersection to update
          Some((t_left, t_right))
        }
      } else {
        let t = t_numerator / t_denominator;
        if t_denominator >= 0.0 {
          // constraint bounds the line on the right.
          t_right = t_right.min(t);
        } else {
          t_left = t_left.max(t);
        }
        if t_left > t_right {
          // No valid segment on the line, no solutions
          None
        } else {
          Some((t_left, t_right))
        }
      }
    },
  ) {
    Some((t_left, t_right)) => {
      let t = if maximize_norm {
        if *obj_dir * *l_dir > 0. {
          // obj projects towards the right of l, take rightmost valid point
          t_right
        } else {
          // take leftmost valid point
          t_left
        }
      } else {
        // Compute the intersection between the objective and the line
        let t_numerator = Vec2::det(-*obj_dir, *l_ori);
        let t_denominator = Vec2::det(*obj_dir, *l_dir);
        if t_denominator.abs() <= EPSILON {
          // the vector is (almost) parallel to the constraint
          t_left
        } else {
          let t = t_numerator / t_denominator;
          // Return the point closest to the intersection within the valid segment
          t.max(t_left).min(t_right)
        }
      };
      Some(*l_ori + t * *l_dir)
    }
    None => None,
  }
}

pub fn solve_linear_program(
  obj_dir: &Vec2,
  obj_max_norm: f64,
  half_planes: &[(Vec2, Vec2)],
  maximize_norm: bool,
) -> Option<Vec2> {
  half_planes
    .iter()
    .enumerate()
    .try_fold(*obj_dir * obj_max_norm, |v_im1, (i, h_i)| {
      if is_vector_belonging_to_half_plane(&v_im1, h_i) {
        // v_i-1 is already a solution.
        Some(v_im1)
      } else {
        // either v_i (the updated solution) belongs to the bounding line of h_i or there is no solution.
        solve_linear_program_step(
          obj_dir,
          obj_max_norm,
          h_i,
          &half_planes[0..i],
          maximize_norm,
        )
      }
    })
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn linear_program_one_vt_plane_1() {
    let half_planes = vec![(Vec2::new(2.0, 0.0), Vec2::new(0.0, 1.0))];
    // (1.0, 0.0) can't be extended to belong to the half plane
    assert_eq!(
      solve_linear_program(&Vec2::new(1.0, 0.0), 1.0, &half_planes, false),
      None
    );
    // (3.0, 0.0) already belongs to the half plane it should be returned as-is
    assert_eq!(
      solve_linear_program(&Vec2::new(1.0, 0.0), 3.0, &half_planes, false).unwrap(),
      Vec2::new(3.0, 0.0)
    );
  }

  #[test]
  fn linear_program_one_vt_plane_2() {
    let half_planes = vec![(Vec2::new(1.5, 12.0), Vec2::new(0.0, -1.0))];
    // (1.0, 0.0) already belongs to the half plane it should be returned as-is
    assert_eq!(
      solve_linear_program(&Vec2::new(1.0, 0.0), 1.0, &half_planes, false).unwrap(),
      Vec2::new(1.0, 0.0)
    );
    // (3.0, 0.0) can be shrinked to belong to the half plane
    assert_eq!(
      solve_linear_program(&Vec2::new(1.0, 0.0), 3.0, &half_planes, false).unwrap(),
      Vec2::new(1.5, 0.0)
    );
  }

  #[test]
  fn linear_program_one_hz_plane_1() {
    let half_planes = vec![(Vec2::new(12.0, -2.0), Vec2::new(-1.0, 0.0))];
    // (1.0, 0.0) already belongs to the half plane it should be returned as-is
    assert_eq!(
      solve_linear_program(&Vec2::new(1.0, 0.0), 1.0, &half_planes, false).unwrap(),
      Vec2::new(1.0, 0.0)
    );
    // (3.0, 0.0) already belongs to the half plane it should be returned as-is
    assert_eq!(
      solve_linear_program(&Vec2::new(1.0, 0.0), 3.0, &half_planes, false).unwrap(),
      Vec2::new(3.0, 0.0)
    );
    // (0.0, -3.0) can be shrinked to belong to the half plane
    assert_eq!(
      solve_linear_program(&Vec2::new(0.0, -1.0), 3.0, &half_planes, false).unwrap(),
      Vec2::new(0.0, -2.0)
    );
    // (2.0, -4.0) can be shrinked to belong to the half plane
    let vec = Vec2::new(2.0, -4.0);
    assert_eq!(
      solve_linear_program(&vec.normalize(), vec.norm(), &half_planes, false).unwrap(),
      Vec2::new(1.0, -2.0)
    );
  }
  #[test]
  fn linear_program_two_planes() {
    let half_planes = vec![
      (Vec2::new(2.0, -2.0), Vec2::new(-1.0, -1.0).normalize()),
      (Vec2::new(12.0, -2.0), Vec2::new(-1.0, 0.0)),
    ];
    let half_planes_other_order = vec![
      (Vec2::new(12.0, -2.0), Vec2::new(-1.0, 0.0)),
      (Vec2::new(2.0, -2.0), Vec2::new(-1.0, -1.0).normalize()),
    ];
    // (1.0, 0.0) already belongs to the region it should be returned as-is
    assert_eq!(
      solve_linear_program(&Vec2::new(1.0, 0.0), 1.0, &half_planes, false).unwrap(),
      Vec2::new(1.0, 0.0)
    );
    assert_eq!(
      solve_linear_program(&Vec2::new(1.0, 0.0), 1.0, &half_planes, false).unwrap(),
      solve_linear_program(&Vec2::new(1.0, 0.0), 1.0, &half_planes_other_order, false).unwrap(),
    );
    // (0.0, -3.0) can be shrinked to belong to the region
    assert_eq!(
      solve_linear_program(&Vec2::new(0.0, -1.0), 3.0, &half_planes, false).unwrap(),
      Vec2::new(0.0, -2.0)
    );
    assert_eq!(
      solve_linear_program(&Vec2::new(0.0, -1.0), 3.0, &half_planes, false).unwrap(),
      solve_linear_program(&Vec2::new(0.0, -1.0), 3.0, &half_planes_other_order, false).unwrap(),
    );
    // (1.0, -4.0) can be shrinked to belong to the region
    let vec = Vec2::new(1.0, -4.0);
    assert_eq!(
      solve_linear_program(&vec.normalize(), vec.norm(), &half_planes, false).unwrap(),
      Vec2::new(0.5, -2.0)
    );
    assert_eq!(
      solve_linear_program(&vec.normalize(), vec.norm(), &half_planes, false).unwrap(),
      solve_linear_program(
        &vec.normalize(),
        vec.norm(),
        &half_planes_other_order,
        false
      )
      .unwrap(),
    );
  }
}
