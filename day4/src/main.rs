use std::collections::HashSet;
use std::hash::Hash;
use std::io::{self, Read};

fn has_dupes<T: Eq + Hash, I: Iterator<Item = T>>(mut iter: I) -> bool {
    let mut set = HashSet::new();
    iter.all(move |x| set.insert(x))
}

fn part1<'a, Matrix: AsRef<[Row]>, Row: AsRef<[&'a str]>>(lines: Matrix) -> usize {
    lines
        .as_ref()
        .iter()
        .filter(|line| has_dupes(line.as_ref().iter()))
        .count()
}

fn part2<'a, Matrix: AsRef<[Row]>, Row: AsRef<[&'a str]>>(lines: Matrix) -> usize {
    lines
        .as_ref()
        .iter()
        .filter(|words| {
            let sorted_letters: Vec<Vec<_>> = words
                .as_ref()
                .iter()
                .map(move |word| {
                    let mut letters: Vec<_> = word.chars().collect();
                    letters.sort();
                    letters
                })
                .collect();
            has_dupes(sorted_letters.iter())
        })
        .count()
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let lines: Vec<Vec<_>> = input
        .lines()
        .map(|line| line.split_whitespace().collect())
        .collect();
    println!("part1: {}", part1(&lines));
    println!("part2: {}", part2(&lines));
}
