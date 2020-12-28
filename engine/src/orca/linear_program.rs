use crate::vec2::Vec2;

const EPSILON: f64 = 0.000001;

fn is_vector_belonging_to_half_plane(v: &Vec2, (origin, direction): &(Vec2, Vec2)) -> bool {
  Vec2::det(*direction, *v - *origin) >= 0.
}

pub fn solve_linear_program_step(
  obj_dir: &Vec2,                        // The objective direction
  obj_max_norm: f64,                     // The objective maximum norm
  (h_i_ori, h_i_dir): &(Vec2, Vec2),     // The half plane boundary origin and direction unit vector
  previous_half_planes: &[(Vec2, Vec2)], // The previous half planes constraints
  maximize_norm: bool,
) -> Option<Vec2> {
  debug_assert!(obj_max_norm >= 0.);
  debug_assert!(
    (obj_dir.sqr_norm() - 1.0).abs() < EPSILON,
    "The objective direction, {}, should be normalized, got a norm of {}!",
    obj_dir,
    obj_dir.norm()
  );
  // Let's find the segment of h boundary intersection the circle of radius `obj_max_norm`
  //
  // obj_max_norm = || h_i_ori + t * h_i_dir ||
  // 0 = || h_i_ori + t * h_i_dir || - obj_max_norm
  // 0 = || h_i_ori + t * h_i_dir ||^2 - obj_max_norm^2
  //   = (h_i_ori.x + t * h_i_dir.x)^2 + (h_i_ori.y + t * h_i_dir.y)^2 - obj_max_norm^2
  //   =   h_i_ori.x^2 + 2*h_i_ori.x*t*h_i_dir.x + t^2*h_i_dir.x^2
  //     + h_i_ori.y^2 + 2*h_i_ori.y*t*h_i_dir.y + t^2*h_i_dir.y^2
  //     - obj_max_norm^2
  //   =   (h_i_dir.x^2 + h_i_dir.y^2) * t^2
  //     + (2*h_i_ori.x*h_i_dir.x + 2*h_i_ori.y*h_i_dir.y) * t
  //     + h_i_ori.x^2 + h_i_ori.y^2 - obj_max_norm^2
  //   =   || h_i_dir ||^2 * t^2
  //     + 2 * h_i_ori . h_i_dir * t
  //     + || h_i_ori ||^2 - obj_max_norm^2
  //
  // We can solve this polynom

  //let poly_a = 1.0;
  let poly_b = 2.0 * *h_i_ori * *h_i_dir;
  let poly_c = h_i_ori.sqr_norm() - obj_max_norm.powi(2);
  let poly_det = poly_b.powi(2) - 4.0 /* * poly_a */ * poly_c;

  if poly_det < 0.0 {
    // println!("* Can't belong to Hi while keeping its norm, no solution");
    return None;
  }

  let poly_det_sqrt = poly_det.sqrt();

  match previous_half_planes.iter().try_fold(
    (
      (-poly_b - poly_det_sqrt) / (2.0/* * poly_a */),
      (-poly_b + poly_det_sqrt) / (2.0/* * poly_a */),
    ),
    |(mut t_left, mut t_right), (h_k_ori, h_k_dir)| {
      // println!(
      //   "* Vi belongs to Hi if Vi ∈ [{}, {}] * {} + {}, adding further constraint Hk=({}, {})",
      //   t_left, t_right, h_i_dir, h_i_ori, h_k_ori, h_k_dir
      // );
      // We are looking for intersection betwen h_k and h_i called inter.
      // h_k_dir_perp being the vector perpendicular to h_k_dir, we know that
      // 0 = h_k_dir_perp . (inter - h_k_ori)
      // Because inter belongs to the boundary of h we can define it as inter = h_i_ori + t * h_i_dir
      // We then have
      // 0 = h_k_dir_perp . (h_i_ori + t * h_i_dir - h_k_ori)
      // 0 = h_k_dir_perp . h_i_ori + t * h_k_dir_perp . h_i_dir - h_k_dir_perp . h_k_ori
      // t * h_k_dir_perp . h_i_dir = h_k_dir_perp . h_k_ori - h_k_dir_perp . h_i_ori +
      // t * h_k_dir_perp . h_i_dir = h_k_dir_perp . (h_k_ori - h_i_ori)
      // t = (h_k_dir_perp . (h_k_ori - h_i_ori)) / (h_k_dir_perp . h_i_dir)
      // t = det(h_k_dir, h_k_ori - h_i_ori) / det(h_k_dir, h_i_dir)
      //
      // If the denominator is 0 then h_i and h_k are parallel
      let t_numerator = Vec2::det(*h_k_dir, *h_k_ori - *h_i_ori);
      let t_denominator = Vec2::det(*h_k_dir, *h_i_dir);

      if t_denominator.abs() <= EPSILON {
        //println!("** Hk // Hi");
        if t_numerator > 0. {
          //println!("*** Hk on the right side of Hi => No solution");
          None
        } else {
          //println!("*** Hk on the left of Hi, it has no impact");
          Some((t_left, t_right))
        }
      } else {
        let t = t_numerator / t_denominator;
        //println!("** Hk intersects Hi at {}", t);
        if t_denominator < 0.0 {
          t_right = t_right.min(t);
        } else {
          t_left = t_left.max(t);
        }
        //println!("*** Vi range updated to [{}, {}]", t_left, t_right);
        if t_left > t_right {
          //println!("*** No valid segment belonging to Hi => No solution");
          None
        } else {
          Some((t_left, t_right))
        }
      }
    },
  ) {
    Some((t_h_i_left, t_h_i_right)) => {
      // println!(
      //   "* Vi belongs to Hi while respecting H[0..i-1] if Vi ∈ [{}, {}] * {} + {}",
      //   t_h_i_left, t_h_i_right, h_i_dir, h_i_ori
      // );
      let t_h_i = {
        // Compute the intersection between the objective and the line
        let t_h_i_numerator = Vec2::det(*obj_dir, -*h_i_ori);
        let t_h_i_denominator = Vec2::det(*obj_dir, *h_i_dir);
        if t_h_i_denominator.abs() <= EPSILON {
          // println!(
          //   "** Vi // Hi, taking the rightmost valid intersection at {}",
          //   t_h_i_right
          // );
          t_h_i_right
        } else {
          let t_h_i = t_h_i_numerator / t_h_i_denominator;
          if maximize_norm {
            //println!("** Maximizing Vi norm");
            if (t_h_i - t_h_i_left).abs() < (t_h_i - t_h_i_right).abs() {
              //println!("** Vi is the leftmost valid Hi bounds at {}", t_h_i_left);
              t_h_i_left
            } else {
              //println!("** Vi is the rightmost valid Hi bounds at {}", t_h_i_right);
              t_h_i_right
            }
          } else {
            //println!("** Preserve obj direction");
            // println!(
            //   "** Vi intersects Hi at {}, which is clamped to {}",
            //   t_h_i,
            //   t_h_i.max(t_h_i_left).min(t_h_i_right)
            // );
            // Return the point closest to the intersection within the valid segment
            t_h_i.max(t_h_i_left).min(t_h_i_right)
          }
        }
      };
      let candidate = *h_i_ori + t_h_i * *h_i_dir;
      //println!("** Candidate Vi = {}", candidate);
      if candidate * *obj_dir < 0.0 {
        //println!("*£ Candidate Vi goes backward");
        None
      } else {
        //println!("* Candidate Vi validated");
        Some(candidate)
      }
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
        //println!("Vi-1={} ∈ Hi=({}, {}) => Vi = Vi-1", v_im1, h_i.0, h_i.1);
        Some(v_im1)
      } else {
        //println!("Vi-1={} ∉ Hi=({}, {})", v_im1, h_i.0, h_i.1);
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
  use approx::assert_relative_eq;

  #[test]
  fn test_is_vector_belonging_to_half_plane() {
    assert_eq!(
      is_vector_belonging_to_half_plane(
        &Vec2::new(1.0, 1.0),
        &(Vec2::new(0.0, 2.0), Vec2::new(1.0, 0.0))
      ),
      false
    );

    assert_eq!(
      is_vector_belonging_to_half_plane(
        &Vec2::new(3.0, 3.0),
        &(Vec2::new(0.0, 2.0), Vec2::new(1.0, 0.0))
      ),
      true
    );

    assert_eq!(
      is_vector_belonging_to_half_plane(
        &Vec2::new(1.0, 0.0),
        &(Vec2::new(-1.0, 0.0), Vec2::new(-1.0, -1.0).normalize())
      ),
      true
    );
  }

  fn check_solve_linear_program(
    obj: &Vec2,
    half_planes: &[(Vec2, Vec2)],
    maximize_norm: bool,
    expected_solution: Option<Vec2>,
  ) -> Option<Vec2> {
    let solution = solve_linear_program(&obj.normalize(), obj.norm(), &half_planes, maximize_norm);
    match expected_solution {
      Some(expected_valid_solution) => {
        let valid_solution = solution.unwrap();

        half_planes.iter().for_each(|&(h_dir, h_ori)| {
          assert!(
            is_vector_belonging_to_half_plane(&valid_solution, &(h_dir, h_ori)),
            "Computed solution {} ∉ input half-plane ({}, {})",
            valid_solution,
            h_dir,
            h_ori,
          )
        });

        assert_relative_eq!(valid_solution, expected_valid_solution);
      }
      None => assert_eq!(solution, None),
    };
    solution
  }

  #[test]
  fn linear_program_one_vt_half_plane_1() {
    let half_planes = vec![(Vec2::new(2.0, 0.0), Vec2::new(0.0, -1.0))];
    // (1.0, 0.0) can't be extended to belong to the half plane
    check_solve_linear_program(&Vec2::new(1.0, 0.0), &half_planes, false, None);
    // (3.0, 0.0) already belongs to the half plane it should be returned as-is
    check_solve_linear_program(
      &Vec2::new(3.0, 0.0),
      &half_planes,
      false,
      Some(Vec2::new(3.0, 0.0)),
    );
  }

  #[test]
  fn linear_program_one_vt_half_plane_2() {
    let half_planes = vec![(Vec2::new(3.0, 12.0), Vec2::new(0.0, 1.0))];
    // (1.0, 0.0) already belongs to the half plane it should be returned as-is
    check_solve_linear_program(
      &Vec2::new(1.0, 0.0),
      &half_planes,
      false,
      Some(Vec2::new(1.0, 0.0)),
    );
    // (5.0, 0.0) can be shrinked to belong to the half plane
    check_solve_linear_program(
      &Vec2::new(5.0, 0.0),
      &half_planes,
      false,
      Some(Vec2::new(3.0, 0.0)),
    );
    // (5.0, 0.0) can be turned to maximize its norm and belong to the half plane
    check_solve_linear_program(
      &Vec2::new(5.0, 0.0),
      &half_planes,
      true,
      Some(Vec2::new(3.0, 4.0)),
    );
  }

  #[test]
  fn linear_program_one_hz_half_plane_1() {
    let half_planes = vec![(Vec2::new(12.0, -2.0), Vec2::new(1.0, 0.0))];
    // (1.0, 0.0) already belongs to the half plane it should be returned as-is
    check_solve_linear_program(
      &Vec2::new(1.0, 0.0),
      &half_planes,
      false,
      Some(Vec2::new(1.0, 0.0)),
    );
    // (3.0, 0.0) already belongs to the half plane it should be returned as-is
    check_solve_linear_program(
      &Vec2::new(3.0, 0.0),
      &half_planes,
      false,
      Some(Vec2::new(3.0, 0.0)),
    );
    // (0.0, -3.0) can be shrinked to belong to the half plane
    check_solve_linear_program(
      &Vec2::new(0.0, -3.0),
      &half_planes,
      false,
      Some(Vec2::new(0.0, -2.0)),
    );
    // (2.0, -4.0) can be shrinked to belong to the half plane
    check_solve_linear_program(
      &Vec2::new(2.0, -4.0),
      &half_planes,
      false,
      Some(Vec2::new(1.0, -2.0)),
    );
  }

  #[test]
  fn linear_program_one_half_plane() {
    let half_planes = vec![(Vec2::new(0.5, 0.), Vec2::new(-1.0, -2.0).normalize())];
    // (3.0, 0.0) aleady belong the halfplane
    check_solve_linear_program(
      &Vec2::new(3.0, 0.0),
      &half_planes,
      true,
      Some(Vec2::new(3.0, 0.0)),
    );
    // (-3.0, 0.0) would need to be reversed to belong to the halfplane
    check_solve_linear_program(&Vec2::new(-3.0, 0.0), &half_planes, true, None);
    // (-1.0, -2.0) would need to be reversed to belong to the halfplane
    check_solve_linear_program(
      &Vec2::new(-1.0, -2.0),
      &half_planes,
      true,
      Some(Vec2::new(-0.5797958971132715, -2.159591794226543)),
    );
  }

  #[test]
  fn linear_program_two_half_planes_1() {
    let half_planes = vec![
      (Vec2::new(2.0, -2.0), Vec2::new(1.0, 1.0).normalize()),
      (Vec2::new(12.0, -2.0), Vec2::new(1.0, 0.0)),
    ];
    let mut half_planes_other_order = half_planes.clone();
    half_planes_other_order.reverse();
    // (1.0, 0.0) already belongs to the region it should be returned as-is
    {
      let sol = check_solve_linear_program(
        &Vec2::new(1.0, 0.0),
        &half_planes,
        false,
        Some(Vec2::new(1.0, 0.0)),
      );
      check_solve_linear_program(&Vec2::new(1.0, 0.0), &half_planes_other_order, false, sol);
    }
    // (0.0, -3.0) can be shrinked to belong to the region
    {
      let sol = check_solve_linear_program(
        &Vec2::new(0.0, -3.0),
        &half_planes,
        false,
        Some(Vec2::new(0.0, -2.0)),
      );
      check_solve_linear_program(&Vec2::new(0.0, -3.0), &half_planes_other_order, false, sol);
    }
    // (0.0, -3.0) can be turned to belong to the region
    {
      let sol = check_solve_linear_program(
        &Vec2::new(0.0, -3.0),
        &half_planes,
        true,
        Some(Vec2::new(2.0, -2.0)),
      );
      check_solve_linear_program(&Vec2::new(0.0, -3.0), &half_planes_other_order, true, sol);
    }
    // (1.0, -4.0) can be shrinked to belong to the region
    {
      let sol = check_solve_linear_program(
        &Vec2::new(1.0, -4.0),
        &half_planes,
        false,
        Some(Vec2::new(0.5, -2.0)),
      );
      check_solve_linear_program(&Vec2::new(1.0, -4.0), &half_planes_other_order, false, sol);
    }
  }

  #[test]
  fn linear_program_three_half_planes_2_parallels() {
    let half_planes = vec![
      (Vec2::new(-2., -2.), Vec2::new(2., 2.).normalize()),
      (Vec2::new(0., 1.), Vec2::new(-1., 0.)),
      (Vec2::new(2., 0.), Vec2::new(2., 2.).normalize()),
    ];
    let mut half_planes_other_order = half_planes.clone();
    half_planes_other_order.reverse();
    // (0., 0.5) already belongs to the region it should be returned as-is
    check_solve_linear_program(
      &Vec2::new(0., 0.5),
      &half_planes,
      false,
      Some(Vec2::new(0., 0.5)),
    );
    // (2., 2.) can be shrinked to belong to the region
    check_solve_linear_program(
      &Vec2::new(2., 2.),
      &half_planes,
      false,
      Some(Vec2::new(1., 1.)),
    );
    // (8., 8.) can be shrinked to belong to the region
    check_solve_linear_program(
      &Vec2::new(8., 8.),
      &half_planes_other_order,
      false,
      Some(Vec2::new(1., 1.)),
    );
    // (8., 8.) can be turned to belong to the region
    check_solve_linear_program(
      &Vec2::new(4., 6.),
      &half_planes,
      true,
      Some(Vec2::new(1., 1.)),
    );
    // (8., 8.) can be turned to belong to the region (with half-planes in the reverse order)
    check_solve_linear_program(
      &Vec2::new(4., 6.),
      &half_planes_other_order,
      true,
      Some(Vec2::new(1., 1.)),
    );
  }
}
