#![procedural::magic_macro]
//! Write the following functions, for each of the functions think carefully about what is the
//! best way to pass the arguments (&, &mut or passing ownership):
//! - Write a function max that takes a Vec of i32 and returns the maximum value inside it.
//! - Write a function swap that swaps the first and last element of a vector of i32.
//! - Write a function is_sorted that takes a Vec of i32 and returns a boolean indicating
//!   whether the vector is sorted in non-decreasing order.
//! - Write a function insert_if_longer that takes a Vec of String ( vec ) and a String
//! ( string ). This function should insert string into vec only if the length of string
//! is greater than 10.
//! Also, when possible, implement these functions recursively, not iteratively.
//!
//! Hint: try recursive and iterative approach

#[runtest(1.0, max)]
/// checks if the max function works as intended
fn test_max() {
    let mut v = vec![1, 2, 5, 3, 5, 3, 6, 4, 3, 3];
    assert_eq!(max(&v), Option::Some(6));
}

#[runtest(1.0, swap)]
/// checks if the swap function works as intended
fn test_swap() {
    let mut v = vec![1, 2, 5, 3, 5, 3, 6, 4, 3, 3];
    swap(&mut v);
    assert_eq!(v, vec![3, 2, 5, 3, 5, 3, 6, 4, 3, 1]);
}

#[runtest(1.0, is_sorted)]
/// checks if the function is_sorted works as intended
fn test_is_sorted() {
    let v = vec![1, 2, 5, 3, 5, 3, 6, 4, 3, 3];
    assert_eq!(is_sorted(&v), false);
    let v = vec![1, 2, 3, 4, 5, 6];
    assert_eq!(is_sorted(&vec![1, 2, 3, 4, 5, 6]), true);
}

#[runtest(1.0, insert_if_longer)]
/// checks if the function insert_if_longer works as intended
fn test_insert_if_longer() {
    let mut t = vec!["ciao".to_string()];
    insert_if_longer(&mut t, "testalo".to_string());
    assert_eq!(t, vec!["ciao".to_string()]);
    insert_if_longer(&mut t, "1234567890".to_string());
    assert_eq!(t, vec!["ciao".to_string()]);
    insert_if_longer(&mut t, "12345678901".to_string());
    assert_eq!(t, vec!["ciao".to_string(), "12345678901".to_string()]);
}

fn max(vec: &[i32]) -> Option<i32> {
    if vec.len() == 0 {
        return Option::None;
    }
    let mut max = vec[0];
    for val in vec.iter() {
        if *val > max {
            max = *val;
        }
    }
    Option::Some(max)
}

fn swap(vec: &mut [i32]) {
    if vec.len() <= 1 {
        return;
    }

    let last_index = vec.len() - 1;

    // option 1
    let x = vec[0];
    vec[0] = vec[last_index];
    vec[last_index] = x;
}

fn is_sorted(vec: &Vec<i32>) -> bool {
    if vec.len() == 0 {
        return true;
    }

    let mut prev = vec[0];

    for i in vec {
        if *i < prev {
            return false;
        }
        prev = *i;
    }
    true
}

fn insert_if_longer(vec: &mut Vec<String>, string: String) {
    if string.len() > 10 {
        vec.push(string);
    }
}
