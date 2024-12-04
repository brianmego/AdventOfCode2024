use aoc_utils::{BadTileTypeError, Collection, Direction, Loc, ParseableCharacters, Tile};

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

enum NotWordError {
    StartWithNonX,
    NoM,
    NoA,
    NoS,
}
trait WordSearch {
    fn search_loc_for_words(&self, loc: Loc) -> Vec<Tile<Letter>>;
}
impl WordSearch for Collection<Letter> {
    fn search_loc_for_words(&self, loc: Loc) -> Vec<Tile<Letter>> {
        let new_loc = loc;
        Direction::get_all()
            .iter()
            .map(|d| {
                let tile = self.get_tile(new_loc).unwrap();
                let mut word: Vec<&Tile<Letter>> = vec![tile];
                while let Some(next_letter) = word.last().unwrap().tile_type().get_next() {
                }
                            // let next_loc = loc.get_nearby(*d, 1).unwrap();
                            // if let Some(next_tile) = self.get_tile(next_loc) {
                            //     if next_tile.tile_type() == &next_letter {
                            //         word.push(next_tile);
                            //         println!("{:?} {:?} is a good start", loc, d);
                            //     }
                            // }
                    // }
                // }
                word
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_utils::{parse_collection, Collection, Loc};

    #[test]
    fn test_parse_input() {
        let actual: (&str, Collection<Letter>) = parse_collection(PUZZLE_INPUT).unwrap();
        assert_eq!(actual.0.len(), 0);
        assert_eq!(actual.1.len(), 100);
    }

    #[test]
    fn test_word_search() {
        let actual: (&str, Collection<Letter>) = parse_collection(PUZZLE_INPUT).unwrap();
        let loc = Loc::new(5, 9);
        actual.1.search_loc_for_words(loc);
        assert!(false);
        // assert_eq!(next_tile, &Tile::new(Letter::M, Loc::new(6, 0)));
    }
}
