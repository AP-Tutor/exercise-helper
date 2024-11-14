#![procedural::magic_macro]
//! Write a function build_vector that takes a Iter<i32> and returns the Vec<&i32>
//! containing all the elements of the iterator;

use std::slice::Iter;

fn build_vector(iterator: Iter<i32>) -> Vec<&i32> {
    let vector: Vec<&i32> = iterator.collect();
    vector
}

#[runtest(1.0, build_vector)]
/// checks that the build_vector function works in general
pub fn test() {
    let vector = vec![2, 4, 5, 3];
    let verification = vec![&2, &4, &5, &3];
    assert_eq!(build_vector(vector.iter()), verification);
}
#[runtest(1.0, build_vector)]
/// checks what appends when an empty iterator is provided
pub fn empty_test() {
    let vector = vec![];
    let verification: Vec<&i32> = vec![];
    assert_eq!(build_vector(vector.iter()), verification);
}
