#![procedural::magic_macro]
//!Write a function e_equals_mc_squared that takes as input a f32 named m representing the mass,
//!and that uses a globally-defined C constant (const C: f32 = ...)
//!containing the value of the speed of light in a vacuum (expressed in m/s).
//! The function outputs the energy equivalent of the mass input

const C: f32 = 299_792_458.0;

fn e_equals_mc_squared(m: f32) -> f32 {
    m * C.powi(2)
}

#[runtest(1.0, C)]
/// Checks if the constant C exits
fn test_c() {
    let x = C;
}

#[runtest(1.0, e_equals_mc_squared)]
/// Test the function
fn test_e_equals_mc_squared() {
    assert_eq!(C.powi(2), e_equals_mc_squared(1.))
}
