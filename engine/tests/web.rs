//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;

use approx::assert_relative_eq;
use wasm_bindgen_test::*;

extern crate rusty_pedestrians_engine;
use rusty_pedestrians_engine::Universe;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
pub fn test_initial_universe() {
  let universe = Universe::new();
  assert_eq!(universe.count_agents(), 0);
  assert_eq!(universe.render_agents().len(), 0);

  assert_eq!(
    universe.render_navmesh(),
    "v 0.000 0.000 0.0\n\
    v 1.000 0.000 0.0\n\
    v 1.000 1.000 0.0\n\
    v 0.000 1.000 0.0\n\
    f 1 2 3\n\
    f 1 3 4\n"
  );
}

#[wasm_bindgen_test]
pub fn test_simple_antipodal_scenario() {
  let mut universe = Universe::new();
  universe.load_scenario(
    "{
      \"scenario\": \"AntipodalCircle\",
      \"agents_count\": 4,
      \"radius\": 10.0
    }",
  );
  assert_eq!(universe.count_agents(), 4);
  universe
    .render_agents()
    .iter()
    .zip(vec![
      10.0, 0.0, -1.0, 0.0, 0.0, 0.0, 0.35, 0.0, 10.0, 0.0, -1.0, 0.0, 0.0, 0.35, -10.0, 0., 1.0,
      0.0, 0.0, 0.0, 0.35, 0.0, -10., 0.0, 1.0, 0.0, 0.0, 0.35,
    ])
    .for_each(|(value, expected)| assert_relative_eq!(value, &expected, epsilon = 0.0001));

  (0..100).for_each(|_| {
    universe.update(0.25);
    let _debug_info = universe.render_debug_info(0);
  });

  let end_state = universe.render_agents();
  // All should have reached their target
  assert_relative_eq!(end_state[0], -10., epsilon = 0.0001);
  assert_relative_eq!(end_state[1], 0., epsilon = 0.0001);
  assert_relative_eq!(end_state[7], 0., epsilon = 0.0001);
  assert_relative_eq!(end_state[8], -10., epsilon = 0.0001);
  assert_relative_eq!(end_state[14], 10., epsilon = 0.0001);
  assert_relative_eq!(end_state[15], 0., epsilon = 0.0001);
  assert_relative_eq!(end_state[21], 0., epsilon = 0.0001);
  assert_relative_eq!(end_state[22], 10., epsilon = 0.0001);

  assert_eq!(
    universe.render_navmesh(),
    "v -15.000 -15.000 0.0\n\
    v 15.000 -15.000 0.0\n\
    v 15.000 15.000 0.0\n\
    v -15.000 15.000 0.0\n\
    f 1 2 3\n\
    f 1 3 4\n"
  );
}
