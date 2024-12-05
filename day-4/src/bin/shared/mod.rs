use aoc_utils::{BadTileTypeError, Collection, Direction, Loc, ParseableCharacters, Tile, parse_collection};
use rayon::prelude::*;

#[cfg(test)]
pub const PUZZLE_INPUT: &str = include_str!("../../data/sample_input.txt");

#[cfg(not(test))]
pub const PUZZLE_INPUT: &str = include_str!("../../data/puzzle_input.txt");

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Letter {
    X,
    M,
    A,
    S,
}
trait GetNext {
    fn get_next(&self) -> Option<Self>
    where
        Self: Sized;
}
impl GetNext for Letter {
    fn get_next(&self) -> Option<Self> {
        match self {
            Letter::X => Some(Letter::M),
            Letter::M => Some(Letter::A),
            Letter::A => Some(Letter::S),
            Letter::S => None,
        }
    }
}
impl TryFrom<char> for Letter {
    type Error = BadTileTypeError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(Self::X),
            'M' => Ok(Self::M),
            'A' => Ok(Self::A),
            'S' => Ok(Self::S),
            _ => unreachable!(),
        }
    }
}
impl ParseableCharacters for Letter {
    fn valid_chars() -> Vec<char> {
        vec!['X', 'M', 'A', 'S']
    }
}

// enum NotWordError {
//     StartWithNonX,
//     NoM,
//     NoA,
//     NoS,
// }
type Word = Vec<Tile<Letter>>;
pub struct WordSearch(Collection<Letter>);

impl From<&str> for WordSearch {
    fn from(value: &str) -> Self {
        Self(parse_collection(value).unwrap().1)
    }
}
impl WordSearch {
    pub fn search_puzzle_for_words(&self) -> Vec<Word> {
        self.0.tiles()
            .par_iter()
            .map(|t| self.search_loc_for_words(*t.loc()))
            .flatten()
            .collect()
    }

    fn search_loc_for_words(&self, loc: Loc) -> Vec<Word> {
        let words: Vec<Word> = Direction::get_all()
            .par_iter()
            .map(|d| {
                let mut new_loc = loc;
                let tile = self.0.get_tile(new_loc).unwrap();
                let mut word: Vec<Tile<Letter>> = vec![*tile];
                while let Some(expected_next_letter) = word.last().unwrap().tile_type().get_next() {
                    if let Some(next_loc) = new_loc.get_nearby(*d, 1) {
                        if let Some(next_tile) = self.0.get_tile(next_loc) {
                            if next_tile.tile_type() == &expected_next_letter {
                                word.push(*next_tile);
                                new_loc = next_loc;
                            } else {
                                break;
                            }
                        } else {
                            break;
                        }
                    } else {
                        break;
                    }
                }
                word
            })
            .filter(|x| x.len() == 4)
            .collect();
        words
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let actual = WordSearch::from(PUZZLE_INPUT);
        assert_eq!(actual.0.len(), 100);
    }

    #[test]
    fn test_word_search_loc() {
        let inp = WordSearch::from(PUZZLE_INPUT);
        let loc = Loc::new(5, 9);
        let actual: Vec<Word> = inp.search_loc_for_words(loc);
        assert_eq!(actual.len(), 3);
        // assert_eq!(next_tile, &Tile::new(Letter::M, Loc::new(6, 0)));
    }

    #[test]
    fn test_word_search() {
        let inp = WordSearch::from(PUZZLE_INPUT);
        let actual: Vec<Word> = inp.search_puzzle_for_words();
        assert_eq!(actual.len(), 18);
    }
}
