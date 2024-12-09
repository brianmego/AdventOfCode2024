mod shared;
use shared::{DiskMap, PUZZLE_INPUT};

fn main() {
    let disk_map = DiskMap::new(PUZZLE_INPUT);
    println!("{}", disk_map.to_compact_file().checksum());
}
