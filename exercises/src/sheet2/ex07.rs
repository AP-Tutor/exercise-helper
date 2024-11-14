#![procedural::magic_macro]
//!Write a function pancake_sort that takes a &mut Vec<i32> and sorts it using the
//!pancake sort algorithm (https://en.wikipedia.org/wiki/Pancake_sorting);
//!
//! Hint: try recursive and iterative approach

fn flip(vector: &mut Vec<i32>, k: usize) {
    let (left, _) = vector.split_at_mut(k + 1);
    left.reverse();
}

fn find_max(vector: &[i32]) -> usize {
    let mut index = 0;
    for i in 0..vector.len() {
        if &vector[i] > &vector[index] {
            index = i;
        }
    }
    index
}

pub fn pancake_sort(vector: &mut Vec<i32>) {
    let mut index = vector.len();
    while index > 0 {
        let (first_half, _) = vector.split_at(index);
        index -= 1;
        let max_index = find_max(first_half);
        if index != max_index {
            flip(vector, max_index);
            flip(vector, index);
        }
    }
}

#[runtest(1.0, pancake_sort)]
pub fn test() {
    let mut vector1 = vec![1, 2, 5, 3, 4];
    pancake_sort(&mut vector1);
    assert_eq!(vector1, vec![1, 2, 3, 4, 5]);
}
