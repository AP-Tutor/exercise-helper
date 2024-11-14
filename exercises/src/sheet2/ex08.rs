#![procedural::magic_macro]
//! Write a function merge that takes two &[i32] and returns a Vec<i32> . The returned
//! vector should contain the elements of the two passed elements sorted, you can assume
//! that the passed slices are sorted;

pub fn merge(left: &[i32], right: &[i32]) -> Vec<i32> {
    let mut result = vec![];
    let mut left_index = 0;
    let mut right_index = 0;
    for _ in 0..left.len() + right.len() {
        if left_index < left.len()
            && right_index < right.len()
            && left[left_index] < right[right_index]
        {
            result.push(left[left_index]);
            left_index += 1;
        } else if right_index < right.len() {
            result.push(right[right_index]);
            right_index += 1;
        } else {
            result.push(left[left_index]);
            left_index += 1;
        }
    }
    result
}

#[runtest(1.0, merge)]
// tests the merge function
pub fn test() {
    let mut vector = vec![2, 4, 5, 1, 3];
    let (left, right) = vector.split_at(3);
    let merged_vector = merge(left, right);
    vector.sort();
    assert_eq!(merged_vector, vector);
}
