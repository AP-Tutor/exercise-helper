#![procedural::magic_macro]
//! Create a module called sentence that has a public struct Sentence with a public ﬁeld words: Vec<String>.
//! Create the following methods for the struct Sentence :
//! - new_default that initializes the ﬁeld words with nothing in it.
//! - new that takes a &str , splits it by whitespaces and inserts every word inside the words ﬁeld.
//! 
//! Create another module test with the function magic_sentence that mutually borrows a HashMap<i32, Sentence> , a i: i32 and a j: i32 and returns a Result<Sentence, &str>
//! The function checks if the sentences at the two indexes exist and if so, creates a Sentence with all the equal words in the same position (same index) present in the Sentence s.
//! If no words are found or if the indexes are not present in the HashMap, reutrn an Err(&str).
//! 
//! Example the sentence "Hello my name was cool yesterday" and the sentence "Hi my name is cool"
//! should result in the sentence "my name cool".
#![dependency("oorandom=\"11.1\"")]

#[runtest(1.0)]
/// checks if 
fn check_if_sentence_is_correct(){
    sentence::Sentence{
        words: Vec::<String>::new(),
    };
}
#[runtest(1.0)]
/// checks if new_default is correctly implemented 
fn check_new_default(){
    let s = sentence::Sentence::new_default();
    assert_eq!(s.words, Vec::<String>::new());
}

#[runtest(1.0)]
/// checks if new is correctly implemented 
fn check_new(){
    let s = sentence::Sentence::new("checks if new is correctly implemented");
    assert_eq!(s.words, vec!["checks", "if", "new", "is", "correctly", "implemented"]);
}

#[runtest(1.0)]
#[overwrite(impl Sentence::new in sentence)]
/// checks if magic_sentence is correctly implemented 
fn check_magic_sentence(){
    let mut map = HashMap::<i32, Sentence>::new();
    let s1 = Sentence::new("Ciao come stai funziona forse");
    let s2 = Sentence::new("Hola come stai non forse funziona");

    map.insert(0, s1);
    map.insert(1, s2);

    let s3 = magic_sentence(&map, 0, 1).unwrap();
    println!("{}", s3);
    assert_eq!(s3.words, vec!["come", "stai", "forse"]);

    let s3 = Sentence::new("Hello my name is cool wow");
    let s4 = Sentence::new("Hi my name was cool");

    map.insert(3, s3);
    map.insert(4, s4);

    let s3 = magic_sentence(&map, 3, 4).unwrap();
    println!("{}", s3);
    assert_eq!(s3.words, vec!["my", "name", "cool"]);
}




use self::sentence::Sentence;
use std::collections::HashMap;

fn magic_sentence(map: &HashMap<i32, Sentence>, i: i32, j: i32) -> Result<Sentence, &str> {
    let si = match map.get(&i) {
        Some(e) => e,
        None => return Err("i not found"),
    };

    let sj = match map.get(&j) {
        Some(e) => e,
        None => return Err("j not found"),
    };

    let mut sentence = Sentence::new_default();

    for (wordi, wordj) in si.words.iter().zip(sj.words.iter()) {
        if wordi == wordj {
            sentence.add_word(wordi.clone());
        }
    }

    Ok(sentence)
}

mod sentence{
    use std::fmt::Display;

    pub struct Sentence {
        pub words: Vec<String>,
    }

    impl Display for Sentence {
        fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
            write!(f, "{}", self.words.join(" "))
        }
    }

    impl Sentence {
        pub fn new_default() -> Self {
            Sentence { words: vec![] }
        }

        pub fn new(s: &str) -> Self {
            Sentence {
                words: s.split_whitespace().map(|str| str.to_string()).collect(),
            }
        }

        pub fn add_word(&mut self, word: String) {
            self.words.push(word);
        }
    }

}