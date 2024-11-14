#![procedural::magic_macro]
//! Write a function `append` that takes a `String`, appends the word "foobar" to it and returns it;
fn append(mut s: String) -> String {
    s.push_str("foobar");
    s
}

#[runtest(1.0, apend)]
/// Tests append function
fn check_append() {
    let s1 = "test ".to_string();
    let s2 = append(s1.clone());
    assert_eq!(s2, "test foobar");
}
