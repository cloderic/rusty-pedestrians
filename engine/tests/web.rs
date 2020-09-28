//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate wasm_bindgen_test;

use assert_approx_eq::assert_approx_eq;
use wasm_bindgen_test::*;

extern crate rusty_pedestrians_engine;
use rusty_pedestrians_engine::Universe;

wasm_bindgen_test_configure!(run_in_browser);

#[wasm_bindgen_test]
pub fn test_initial_universe() {
  let universe = Universe::new();
  assert_eq!(universe.count_agents(), 0);
  assert_eq!(universe.render().len(), 0);
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
    .render()
    .iter()
    .zip(vec![
      10.0, 0.0, 0.0, 0.0, 0.35, 0.0, 10.0, 0.0, 0.0, 0.35, -10.0, 0., 0.0, 0.0, 0.35, 0.0, -10.,
      0.0, 0.0,
    ])
    .for_each(|(value, expected)| assert_approx_eq!(value, expected));

  (0..100).for_each(|_| {
    universe.update(0.25);
    let debug_info = universe.render_debug_info(0);
  });

  universe
    .render()
    .iter()
    .zip(vec![
      -10.0, 0.0, 0.0, 0.0, 0.35, 0.0, -10.0, 0.0, 0.0, 0.35, 10.0, 0., 0.0, 0.0, 0.35, 0.0, 10.,
      0.0, 0.0,
    ])
    .for_each(|(value, expected)| assert_approx_eq!(value, expected));
}
