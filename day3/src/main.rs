use std::collections::BTreeMap;

const DEST: i64 = 361527;

fn main() {
    // Part 1 -- Don't build, calculate.
    println!("Part 1");
    let v = DEST as f64;

    let ring = ((1.0/2.0) * ((v-1.0).sqrt() + 1.0)) as i64;
    let ring_offset = DEST - (4 * ring * ring - 4 * ring + 1) - 1;

    println!("ring {}, offset {}", ring, ring_offset);

    let sector = ring_offset / (ring * 2);
    let sector_offset = ring_offset - sector * ring * 2;

    println!("sector {}, offset {}", sector, sector_offset);

    let value = ring + (sector_offset - ring + 1).abs();

    println!("steps {}", value);

    // Part 2 -- Fine, I'll just build it.
    println!("Part 2");
    let mut grid = ValGrid::new();
    let mut x = 0isize;
    let mut y = 0isize;
    let mut last_sum = 0;

    // ring 0
    grid.set(x, y, 1u64);

    for ring in 0.. {
        println!("ring: {}", ring);

        let sector_len = if ring == 0 { 0 } else { (ring*2)-1 };

        // sector 0
        for _ in 0..sector_len-1 {
            x += 1;

            last_sum = grid.adjacent_sum(x,y);
            grid.set(x,y,last_sum);
            println!("x: {}, y: {}, sum: {}", x, y, last_sum);
        }

        // sector 1
        for _ in 0..sector_len {
            y -= 1;

            last_sum = grid.adjacent_sum(x,y);
            grid.set(x,y,last_sum);
            println!("x: {}, y: {}, sum: {}", x, y, last_sum);
        }

        // sector 2
        for _ in 0..sector_len {
            x -= 1;

            last_sum = grid.adjacent_sum(x,y);
            grid.set(x,y,last_sum);
            println!("x: {}, y: {}, sum: {}", x, y, last_sum);
        }

        // sector 3
        for _ in 0..sector_len+1 {
            y += 1;

            last_sum = grid.adjacent_sum(x,y);
            grid.set(x,y,last_sum);
            println!("x: {}, y: {}, sum: {}", x, y, last_sum);
        }

        // Eh, close enough to break here.
        if last_sum > DEST as u64{
            break;
        }
    }
}

struct ValGrid {
    raw: BTreeMap<(isize, isize), u64>
}

impl ValGrid {
    fn new() -> Self {
        Self {
            raw: BTreeMap::new(),
        }
    }

    fn get(&self, x: isize, y: isize) -> u64{
        self.raw.get(&(x,y)).map(|x| *x).unwrap_or(0)
    }

    fn set(&mut self, x: isize, y: isize, val: u64){
        self.raw.insert((x,y), val);
    }

    fn adjacent_sum(&self, x: isize, y: isize) -> u64 {
        let neighbors = [(0,1),(1,1),(1,0),(1,-1),(0,-1),(-1,-1),(-1,0),(-1,1)];

        let mut sum = 0;
        for &(xo,yo) in neighbors.iter() {
            sum += self.get(x+xo, y+yo);
        }

        sum
    }
}
