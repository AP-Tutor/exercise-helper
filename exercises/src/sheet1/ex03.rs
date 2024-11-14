#![procedural::magic_macro]
//! Write a function bigger that takes two i32 and return the bigger number (i32) without using another function call and additional variables;

fn multiply(x: i32, y: f32, z: f64) -> f64 {
    x as f64 * y as f64 * z
}

#[runtest(1.0, multiply)]
/// Tests multiply function with a bunch of values
fn test_multiply() {
    assert_eq!(2.0, multiply(1, 1.0, 2.0));
}
