mod shared;
use shared::{WordSearch, PUZZLE_INPUT};

fn main() {
    let word_search = WordSearch::from(PUZZLE_INPUT);
    let words = word_search.search_puzzle_for_words();
    println!("{}", words.len());
}
