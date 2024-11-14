#![procedural::magic_macro]
//! Write a function named split_at_value that takes two arguments: a slice of i32 called
//! slice and a single i32 value called value . The function should find the first element
//! equal to value inside slice . It should then split the slice at the corresponding index
//! and return the two resulting slices wrapped in an Option.
//! If value is not found in slice , the function should return None.

#[runtest(1.0, split_at_value)]
/// Check if split function works
fn test_split_at_value() {
    let array = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9];

    let (a1, a2) = split_at_value(&array, 4).unwrap();

    assert_eq!(a1, &[0, 1, 2, 3]);
    assert_eq!(a2, &[4, 5, 6, 7, 8, 9]);

    let (a1, a2) = split_at_value(&array, 9).unwrap();

    assert_eq!(a1, &[0, 1, 2, 3, 4, 5, 6, 7, 8]);
    assert_eq!(a2, &[9]);
    let (a1, a2) = split_at_value(&array, 0).unwrap();

    assert_eq!(a1, &[]);
    assert_eq!(a2, &[0, 1, 2, 3, 4, 5, 6, 7, 8, 9]);
}

fn split_at_value(slice: &[i32], value: i32) -> Option<(&[i32], &[i32])> {
    let mut index = Option::<usize>::None;

    for (i, element) in slice.iter().enumerate() {
        if *element == value {
            index = Option::Some(i);
            break;
        }
    }

    let index = index?;

    // option 1
    let s1 = &slice[..index];
    let s2 = &slice[index..];

    return Option::Some((s1, s2));

    // option 2
    //Option::Some(slice.split_at(index))
}
