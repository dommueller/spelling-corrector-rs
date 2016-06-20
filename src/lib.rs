extern crate regex;

use std::collections::HashMap;
use std::collections::HashSet;


pub fn get_words(corpus: &str) -> Vec<&str> {
    let re = regex::Regex::new(r"[a-z]+").unwrap();
    re.captures_iter(corpus).map(|x| x.at(0).unwrap()).collect()
}

pub fn train(features: Vec<&str>) -> HashMap<&str, i32> {
    let mut nwords = HashMap::<&str, i32>::new();
    for word in features {
        let counter = nwords.entry(word).or_insert(1);
        *counter += 1;
    }
    return nwords;
}

pub fn edits1(words: Vec<&str>) -> Vec<&str> {
    unimplemented!();
}

#[derive(Debug)]
struct SimpleSpellChecker<'a> {
    nwords: HashMap<&'a str, i32>
}

static alphabet: &'static str = "abcdefghijklmnopqrstuvwxyz";

impl<'a> SimpleSpellChecker<'a> {
    fn new(corpus: &str) -> SimpleSpellChecker {
        SimpleSpellChecker{nwords: train(get_words(&corpus))}
    }



    fn known_edits2(&self, words: Vec<&str>) -> Vec<&str> {
        unimplemented!();
    }

    fn known(&self, words: Vec<&str>) -> Vec<&str> {
        unimplemented!();
    }

    fn correct(word: &str) -> &str {
        unimplemented!();
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn check_words() {
        let input = "This is a test. Don't be fooled!".to_lowercase();
        let output = vec!["this", "is", "a", "test", "don", "t", "be", "fooled"];
        let words = get_words(&input);
        assert_eq!(words.len(), output.len());
        assert_eq!(words, output);
    }

    #[test]
    fn check_counting() {
        let input = vec!["foo", "bar", "foo", "foo", "bar"];
        let counter = train(input);
        assert_eq!(counter.len(), 2);
        assert_eq!(counter.get("foo"), Some(&4));
        assert_eq!(counter.get("bar"), Some(&3));
        assert_eq!(counter.get("baz"), None);
    }

    #[test]
    fn it_works() {
    }
}
