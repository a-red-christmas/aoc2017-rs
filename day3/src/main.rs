const DEST: i64 = 361527;

fn main() {
    let v = DEST as f64;

    let ring = ((1.0/2.0) * ((v-1.0).sqrt() + 1.0)) as i64;
    let ring_offset = DEST - (4 * ring * ring - 4 * ring + 1) - 1;

    println!("ring {}, offset {}", ring, ring_offset);

    let sector = ring_offset / (ring * 2);
    let sector_offset = ring_offset - sector * ring * 2;

    println!("sector {}, offset {}", sector, sector_offset);

    let value = ring + (sector_offset - ring + 1).abs();

    println!("steps {}", value);
}
