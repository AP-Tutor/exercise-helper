#![procedural::magic_macro]
//!Given a vector of i32, create a function that prints a tuple with the minimum and the maximum value inside the vector;
//!
//! Try to do it recursive and iterative

pub fn max_min(v: Vec<i32>) -> (i32, i32) {
    let mut max = 0;
    let mut min = 0;
    for num in v {
        if num >= max {
            max = num;
        }

        if num <= min {
            min = num;
        }
    }

    (max, min)
}

#[runtest(1.0, max_min)]

fn test_max_min() {
    let v = vec![0, 1, 2, 3, 4, 5];
    let b = vec![-7, -1, 0, -8];

    assert_eq!(max_min(v.clone()), (5, 0));
    assert_eq!(max_min(b.clone()), (0, -8));
    /*let mut max = 0;
    let mut min = 0;
    max_min_recursive(b.clone(), 0, b.len(),  &mut max, &mut min);
    assert_eq!(max, 0);
    assert_eq!(min, -8);*/
}
