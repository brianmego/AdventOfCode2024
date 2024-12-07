mod shared;
use shared::{PUZZLE_INPUT, Maze};

fn main() {
    let mut maze = Maze::new(PUZZLE_INPUT);
    maze.patrol_guard();
    println!("{}", maze.count_visited());
}
