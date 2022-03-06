//! Test suite for the Web and headless browsers.

#![cfg(target_arch = "wasm32")]

extern crate rules_engine;
extern crate wasm_bindgen_test;

use rules_engine::components::*;
use wasm_bindgen_test::*;

wasm_bindgen_test_configure!(run_in_browser);

// :: ---

#[wasm_bindgen_test]
fn pass() {
    assert_eq!(1 + 1, 2);
}

#[wasm_bindgen_test]
fn example_a() {
    let tabletop = tabletop::Tabletop::new(5, 5).unwrap();
    let mut robot = robot::Robot::create(&tabletop).unwrap();

    assert!(robot.evaluate_command("PLACE 0,0,NORTH").is_ok());
    assert!(robot.evaluate_command("MOVE").is_ok());

    let status = robot.evaluate_command("REPORT").unwrap();

    assert_eq!(status, "0,1,NORTH");
}

#[wasm_bindgen_test]
fn example_b() {
    let tabletop = tabletop::Tabletop::new(5, 5).unwrap();
    let mut robot = robot::Robot::create(&tabletop).unwrap();

    assert!(robot.evaluate_command("PLACE 0,0,NORTH").is_ok());
    assert!(robot.evaluate_command("LEFT").is_ok());

    let status = robot.evaluate_command("REPORT").unwrap();

    assert_eq!(status, "0,0,WEST");
}

#[wasm_bindgen_test]
fn example_c() {
    let tabletop = tabletop::Tabletop::new(5, 5).unwrap();
    let mut robot = robot::Robot::create(&tabletop).unwrap();

    assert!(robot.evaluate_command("PLACE 1,2,EAST").is_ok());
    assert!(robot.evaluate_command("MOVE").is_ok());
    assert!(robot.evaluate_command("MOVE").is_ok());
    assert!(robot.evaluate_command("LEFT").is_ok());
    assert!(robot.evaluate_command("MOVE").is_ok());

    let status = robot.evaluate_command("REPORT").unwrap();

    assert_eq!(status, "3,3,NORTH");
}
