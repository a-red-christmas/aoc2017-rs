use std::collections::HashMap;

const INPUT: usize = 347991;

enum Direction {
    Top,
    Down,
    Left,
    Right,
}

struct Position {
    x: isize,
    y: isize,
    dir: Direction,
}

impl Position {
    fn new() -> Position {
        Position {
            x: 0,
            y: 0,
            dir: Direction::Right,
        }
    }

    fn x(&self) -> isize {
        self.x
    }

    fn y(&self) -> isize {
        self.y
    }

    fn step(&mut self) {
        use Direction::*;
        match self.dir {
            Top => {
                self.y += 1;
            }
            Down => {
                self.y -= 1;
            }
            Left => {
                self.x -= 1;
            }
            Right => {
                self.x += 1;
            }
        }
    }

    fn turn(&mut self) {
        use Direction::*;
        self.dir = match self.dir {
            Top => Left,
            Down => Right,
            Left => Down,
            Right => Top,
        }
    }
}

fn part1() -> usize {
    let mut pos = Position::new();
    let mut n = 1;
    let mut turn = 1;
    let mut steps = 1;

    'outer: while n <= INPUT {
        for _ in 0..steps {
            pos.step();
            n += 1;
            if n == INPUT {
                break 'outer;
            }
        }
        pos.turn();
        if turn % 2 == 0 {
            // every 2nd turn we increase steps taken by 1
            steps += 1;
        }
        turn += 1;
    }

    pos.x().abs() as usize + pos.y().abs() as usize
}

fn part2() -> usize {
    let mut pos = Position::new();
    let mut grid: HashMap<(isize, isize), usize> = HashMap::new();
    grid.insert((0, 0), 1);
    let mut turn = 1;
    let mut steps = 1;

    'outer: loop {
        for _ in 0..steps {
            static NEIGHBOURS: [(isize, isize); 8] = [
                (1, 0),
                (1, 1),
                (0, 1),
                (-1, 1),
                (-1, 0),
                (-1, -1),
                (0, -1),
                (1, -1),
            ];
            pos.step();
            let n = NEIGHBOURS
                .iter()
                .map(|&(x, y)| {
                    *grid.get(&(pos.x() + x, pos.y() + y)).unwrap_or(&0)
                })
                .sum();
            grid.insert((pos.x(), pos.y()), n);
            if n > INPUT {
                break 'outer;
            }
        }
        pos.turn();
        if turn % 2 == 0 {
            // every 2nd turn we increase steps taken by 1
            steps += 1;
        }
        turn += 1;
    }

    *grid.get(&(pos.x(), pos.y())).unwrap()
}

fn main() {
    println!("part1: {}", part1());
    println!("part2: {}", part2());
}
