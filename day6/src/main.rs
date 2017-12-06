use std::collections::HashMap;
use std::str::FromStr;

const INPUT: &str = "14	0	15	12	11	11	3	5	1	6	8	4	9	1	8	4";

fn main() {
    let mut banks: Vec<_> = INPUT
        .split_whitespace()
        .map(|x| usize::from_str(x).unwrap())
        .collect();

    let len = banks.len();
    let mut seen = HashMap::new();
    let mut cycles = 0;

    while !seen.contains_key(&banks) {
        seen.insert(banks.clone(), cycles);
        let blocks = *banks.iter().max().unwrap();
        let idx = banks.iter().position(|x| x == &blocks).unwrap();
        let mut i = idx + 1;
        banks[idx] = 0;
        for _ in 0..blocks {
            banks[i % len] += 1;
            i += 1;
        }
        cycles += 1;
    }

    println!("part1: {}", cycles);
    println!("part2: {}", cycles - *seen.get(&banks).unwrap());
}
