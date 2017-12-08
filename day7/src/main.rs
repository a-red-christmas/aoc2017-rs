use std::collections::HashMap;

use std::io::{self, Read};

fn part1(input: &str) -> String {
    let mut discs: HashMap<&str, bool> = HashMap::new();
    for line in input.lines() {
        let words: Vec<_> = line.split_whitespace().collect();
        let name = words[0];
        let weight: usize = words[1]
            .trim_matches(|c| c == '(' || c == ')')
            .parse()
            .unwrap();
        if !discs.contains_key(name) {
            discs.insert(name, false);
        }
        if words.len() > 2 {
            words[3..]
                .iter()
                .map(|word| word.trim_matches(','))
                .for_each(|x| { discs.insert(x, true); });
        }
    }
    let disc = discs.iter().find(|x| !*x.1).unwrap();
    disc.0.to_string()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    println!("part1: {:?}", part1(&input));
}
