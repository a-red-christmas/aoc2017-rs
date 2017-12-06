use std::io::{self, Read};
use std::str::FromStr;

fn part1(maze: &mut [isize]) -> usize {
    let mut jumps = 0;
    let mut idx = 0isize;
    while let Some(i) = maze.get_mut(idx as usize) {
        idx += *i;
        *i += 1;
        jumps += 1
    }
    jumps
}

fn part2(maze: &mut [isize]) -> usize {
    let mut jumps = 0;
    let mut idx = 0isize;
    while let Some(i) = maze.get_mut(idx as usize) {
        idx += *i;
        *i += if *i >= 3 { -1 } else { 1 };
        jumps += 1
    }
    jumps
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let maze: Vec<isize> = input
        .lines()
        .map(|x| isize::from_str(x).unwrap())
        .collect();
    println!("part1: {}", part1(&mut maze.clone()));
    println!("part2: {}", part2(&mut maze.clone()));
}
