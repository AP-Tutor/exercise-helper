#![procedural::magic_macro]
//!Write a function lord_farquaad that takes a String and outputs another String in which every
//!character 'e' is substituted by the character '💥'
//!
//! Hint: try to use the builtin functions, and to write your own version

fn lord_farquaad(ee: String) -> String {
    let mut new_ee = String::new();
    for c in ee.chars() {
        if c == 'e' {
            new_ee.push_str("💥");
        } else {
            new_ee.push(c);
        }
    }
    new_ee
}

#[runtest(1.0, max_min)]

/// Tests the lord_farquaad function
fn test_farquaad() {
    assert_eq!(lord_farquaad("cheesecake".to_string()), "ch💥💥s💥cak💥");
    assert_eq!(lord_farquaad("shrek".to_string()), "shr💥k");
}
