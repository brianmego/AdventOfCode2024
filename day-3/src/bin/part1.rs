mod shared;
use shared::{InputFile, PUZZLE_INPUT};

fn main() {
    let inp = InputFile(PUZZLE_INPUT);
    println!("{}", inp.sum_mul_ops())
}
