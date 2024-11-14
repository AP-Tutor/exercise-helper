#![procedural::magic_macro]
//! Write a function `string_reverse` that takes a `&str` as input and returns a reversed `String`;
//!
//! Try to implement it while using iterators or by implement it buy hand

#[runtest(1.0, string_reverse)]
/// Tests if string_reverse is implemented correctly
pub fn test() {
    fn __reverse(s: &str) -> String {
        s.chars().rev().collect()
    }
    let string: [String; 4] = [
        "Hello World!".to_string(),
        "test".to_string(),
        "ooooo".to_string(),
        "".to_string(),
    ];
    for c in &string {
        assert_eq!(string_reverse(c), __reverse(c));
    }
    let mut s = String::new();
    for i in 0..20 {
        s = s.clone() + &s;
    }
    assert_eq!(string_reverse(&s), __reverse(&s));
}

fn string_reverse(s: &str) -> String {
    let mut out = String::new();
    for c in s.chars().rev() {
        out.push(c);
    }
    out
}
