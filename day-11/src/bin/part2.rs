mod shared;
use shared::{PUZZLE_INPUT, StoneList};

fn main() {
    let stone_list = StoneList::from(PUZZLE_INPUT);
    let new_list = stone_list.blink(75);
    println!("{}", new_list.len());
}
