use std::collections::{BTreeMap, BTreeSet};

fn main() {
    use std::io;
    use std::io::prelude::*;

    let mut validator = Validator::new();
    let mut anag_validator = AnagramValidator::new();

    let stdin = io::stdin();
    for line in stdin.lock().lines() {
        let passphrase = line.unwrap();
        println!("{}: {} ({}/{}) | {} ({}/{})",
            passphrase,
            validator.validate(&passphrase),
            validator.counts().0,
            validator.counts().1,
            anag_validator.validate(&passphrase),
            anag_validator.counts().0,
            anag_validator.counts().1);
    }
}

struct Validator {
    set: BTreeSet<String>,
    count_tested: u64,
    count_valid: u64,
}

impl Validator {
    fn new() -> Self {
        Self {
            set: BTreeSet::new(),
            count_tested: 0,
            count_valid: 0,
        }
    }

    fn validate(&mut self, passphrase: &str) -> bool {
        self.set.clear();
        self.count_tested += 1;

        for word in passphrase.split(" ") {
            match self.set.insert(word.to_string()) {
                true => (),
                false => return false,
            }
        }

        self.count_valid += 1;
        true
    }

    fn counts(&self) -> (u64, u64) {
        (self.count_valid, self.count_tested)
    }
}

struct AnagramValidator {
    set: BTreeSet<BTreeMap<char, u64>>,
    count_tested: u64,
    count_valid: u64,
}

impl AnagramValidator {
    fn new() -> Self {
        Self {
            set: BTreeSet::new(),
            count_tested: 0,
            count_valid: 0,
        }
    }

    fn validate(&mut self, passphrase: &str) -> bool {
        self.set.clear();
        self.count_tested += 1;

        for word in passphrase.split(" ") {
            let mut counts = BTreeMap::new();

            for c in word.chars() {
                *counts.entry(c).or_insert(0) += 1;
            }

            match self.set.insert(counts) {
                true => (),
                false => return false,
            }
        }

        self.count_valid += 1;
        true
    }

    fn counts(&self) -> (u64, u64) {
        (self.count_valid, self.count_tested)
    }
}

#[test]
fn valid_basic() {
    let val = "aa bb cc dd ee"; // is valid.

    let mut validator = Validator::new();
    assert_eq!(validator.validate(val), true);
    assert_eq!(validator.counts(), (1, 1));
}

#[test]
fn invalid_basic() {
    let val = "aa bb cc dd aa"; // is not valid - the word aa appears more than once.

    let mut validator = Validator::new();
    assert_eq!(validator.validate(val), false);
    assert_eq!(validator.counts(), (0, 1));
}

#[test]
fn valid_len_diff() {
    let val = "aa bb cc dd aaa"; // is valid - aa and aaa count as different words.

    let mut validator = Validator::new();
    assert_eq!(validator.validate(val), true);
    assert_eq!(validator.counts(), (1, 1));
}


#[test]
fn valid_by_anag() {
    let val = "abcde fghiji"; // is a valid passphrase.

    let mut validator = AnagramValidator::new();
    assert_eq!(validator.validate(val), true);
    assert_eq!(validator.counts(), (1, 1));
}

#[test]
fn invalid_by_anag() {
    let val = "abcde xyz ecdab"; // is not valid
    //the letters from the third word can be rearranged to form the first word.

    let mut validator = AnagramValidator::new();
    assert_eq!(validator.validate(val), false);
    assert_eq!(validator.counts(), (0, 1));
}

#[test]
fn valid_by_anag_only_partial() {
    let val = "a ab abc abd abf abj"; // is a valid passphrase,
    // because all letters need to be used when forming another word.

    let mut validator = AnagramValidator::new();
    assert_eq!(validator.validate(val), true);
    assert_eq!(validator.counts(), (1, 1));
}

#[test]
fn valid_by_anag_letter_counts() {
    let val = "iiii oiii ooii oooi oooo"; // is valid.

    let mut validator = AnagramValidator::new();
    assert_eq!(validator.validate(val), true);
    assert_eq!(validator.counts(), (1, 1));
}

#[test]
fn invalid_by_anag_all_same_count() {
    let val = "oiii ioii iioi iiio"; // is not valid
        //any of these words can be rearranged to form any other word.

    let mut validator = AnagramValidator::new();
    assert_eq!(validator.validate(val), false);
    assert_eq!(validator.counts(), (0, 1));
}

