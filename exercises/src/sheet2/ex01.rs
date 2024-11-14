#![procedural::magic_macro]
//!Write a function called modify_odd that takes a mutable reference to an array slice of
//!integers and sets all odd numbers to 0.
//!
//! Hint: try using iterators, slices, or a recursive function

fn modify_odd(slice: &mut [u32]) {
    for elem in slice {
        if *elem % 2 == 1 {
            *elem = 0;
        }
    }
}

#[runtest(1.0, modify_odd)]
fn test_modify_add() {
    let mut my_slice = [0; 101];
    for (i, e) in my_slice.iter_mut().enumerate() {
        *e = i as u32;
    }

    let mut my_slice_2 = my_slice.clone();
    let mut my_slice_3 = my_slice.clone();

    modify_odd(my_slice.as_mut_slice());
    assert_eq!(&my_slice[..10], &[0, 0, 2, 0, 4, 0, 6, 0, 8, 0]);

    modify_odd(my_slice_2.as_mut_slice());
    assert_eq!(&my_slice_2[..10], &[0, 0, 2, 0, 4, 0, 6, 0, 8, 0]);

    assert_eq!(my_slice, my_slice_2);

    modify_odd_recursive(&mut my_slice_3);
    assert_eq!(&my_slice_2[..10], &[0, 0, 2, 0, 4, 0, 6, 0, 8, 0]);
}

fn modify_odd_iterators(slice: &mut [u32]) {
    slice.iter_mut().for_each(|x| {
        if *x % 2 == 1 {
            *x = 0
        }
    });
}

fn modify_odd_recursive(slice: &mut [u32]) {
    if slice.len() == 0 {
        return;
    }
    if slice[0] % 2 != 0 {
        slice[0] = 0;
    }

    modify_odd_recursive(&mut slice[1..]);
}
