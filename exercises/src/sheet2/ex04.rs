#![procedural::magic_macro]
//! Write a function sub_slice that takes two &Vec<i32> as input. If the second vector is
//! contained inside the first one it returns it as a slice, else it returns None.
//!
//! Hint: Try with an iterative and a recursive approach

pub fn sub_slice<'a>(vector: &'a Vec<i32>, sub_vector: &'a Vec<i32>) -> Option<&'a [i32]> {
    let slices_number = vector.len() - sub_vector.len();
    let slice_size = sub_vector.len();
    for i in 0..=slices_number {
        let slice = vector.split_at(i).1.split_at(slice_size).0;
        if slice == sub_vector {
            return Some(slice);
        }
    }
    None
}

#[runtest(1.0, sub_slice)]
pub fn test_sub_slice() {
    let vector = vec![1, 2, 3, 4, 5];
    let mut sub_vector = vec![2, 3];
    sub_slice(&vector, &sub_vector);
    //sub_slice_recursive(&vector, &sub_vector);
    sub_vector = vec![2, 4];
    sub_slice(&vector, &sub_vector);
    //sub_slice_recursive(&vector, &sub_vector);
}
