#![procedural::magic_macro]
//!Write a function count_character that takes a string consisting of ASCII characters
//!string as input and returns a HashMap. The keys of the HashMap should be the
//!characters in the string, and the values should be an u32 representing how many times
//!each character appears in the string.
//!
//! Hint: Recursive, or character

use std::collections::HashMap;

#[runtest(1.0, count_character)]
/// tests if the count_character function works
fn test_count_character() {
    let my_map = HashMap::from([('H', 1), ('e', 1), ('l', 2), ('o', 1)]);

    let s = String::from("Hello");

    // note that the function that take as input an &str, can also take as input an &String.
    // the opposite is not true... You should always use &str instead of &String
    assert_eq!(count_character(&s), my_map);
}

fn count_character(string: &str) -> HashMap<char, u32> {
    let mut map = HashMap::<char, u32>::new();

    for c in string.chars() {
        if let Some(val) = map.get_mut(&c) {
            *val += 1;
        } else {
            map.insert(c, 1);
        }
    }

    map
}
