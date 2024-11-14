#![procedural::magic_macro]
//!Write a function bigger that takes two i32 and return the bigger number (i32) without using another function call and additional variables;

fn bigger(int1: i32, int2: i32) -> i32 {
    if int1 >= int2 {
        return int1;
    } else {
        return int2;
    }
}

#[runtest(1.0, bigger)]
/// Tries the bigger function on some values
fn test() {
    let a = 1;
    let b = 2;
    assert_eq!(bigger(a, b), 2);
}
