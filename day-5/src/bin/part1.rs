mod shared;
use shared::{sum_valid_middles, PUZZLE_INPUT};

fn main() {
    println!("{}", sum_valid_middles(PUZZLE_INPUT));
}
