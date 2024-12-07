use aoc_utils::{BadTileTypeError, Collection, Direction, Loc, ParseableCharacters};

#[cfg(test)]
pub const PUZZLE_INPUT: &str = include_str!("../../data/sample_input.txt");

#[cfg(not(test))]
pub const PUZZLE_INPUT: &str = include_str!("../../data/puzzle_input.txt");

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Tile {
    tile_type: TileType,
    visited: bool,
}

impl Tile {
    fn new(tile_type: TileType, visited: bool) -> Self {
        Self { tile_type, visited }
    }
    fn mark_visited(&mut self) {
        self.tile_type = TileType::Visited;
        self.visited = true;
    }
}
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum TileType {
    Empty,
    Obstacle,
    Guard(Direction),
    Visited,
}
impl ParseableCharacters for Tile {
    fn valid_chars() -> Vec<char> {
        vec!['#', '.', '>', '<', '^', 'v']
    }
}
impl TryFrom<char> for Tile {
    type Error = BadTileTypeError;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        let tile_type = match value {
            '#' => Ok(TileType::Obstacle),
            '.' => Ok(TileType::Empty),
            '^' => Ok(TileType::Guard(Direction::North)),
            '>' => Ok(TileType::Guard(Direction::East)),
            '<' => Ok(TileType::Guard(Direction::West)),
            'v' => Ok(TileType::Guard(Direction::South)),
            'X' => Ok(TileType::Visited),
            _ => Err(BadTileTypeError),
        }?;
        Ok(Tile::new(tile_type, false))
    }
}
pub struct Maze {
    tiles: Collection<Tile>,
    guard_loc: Loc,
    guard_direction: Direction,
    visited_locs: Vec<Loc>,
}

impl Maze {
    pub fn new(puzzle_input: &str) -> Self {
        let tiles: Collection<Tile> = Collection::from_puzzle_input(PUZZLE_INPUT);
        let guard_tile = tiles
            .tiles()
            .iter()
            .find(|t| {
                t.get_type().tile_type == TileType::Guard(Direction::West)
                    || t.get_type().tile_type == TileType::Guard(Direction::East)
                    || t.get_type().tile_type == TileType::Guard(Direction::North)
                    || t.get_type().tile_type == TileType::Guard(Direction::South)
            })
            .take()
            .unwrap();
        let guard_loc = guard_tile.loc().clone();
        let guard_direction = match guard_tile.get_type().tile_type {
            TileType::Guard(direction) => direction,
            _ => unreachable!(),
        };
        Self {
            tiles,
            guard_loc,
            guard_direction,
            visited_locs: vec![guard_loc],
        }
    }

    fn len(&self) -> usize {
        self.tiles.len()
    }
    pub fn count_visited(&self) -> usize {
        self.visited_locs.len()
    }

    pub fn advance_guard(&mut self) -> Option<Loc> {
        self.tiles
            .get_tile(self.guard_loc)
            .unwrap()
            .get_type_owned()
            .mark_visited();
        let new_loc = self.guard_loc.get_nearby(self.guard_direction, 1)?;
        let next_loc_tile = self.tiles.get_tile(new_loc)?;
        match next_loc_tile.get_type().tile_type {
            TileType::Empty | TileType::Visited | TileType::Guard(_) => {
                self.guard_loc = new_loc;
                match self.visited_locs.contains(&new_loc) {
                    true => {}
                    false => self.visited_locs.push(new_loc),
                }
            }
            TileType::Obstacle => {
                self.guard_direction = self.guard_direction.rotate_clockwise();
            }
        }
        Some(self.guard_loc)
    }
    pub fn patrol_guard(&mut self) {
        loop {
            match self.advance_guard().is_some() {
                true => continue,
                false => break,
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let actual = Maze::new(PUZZLE_INPUT);
        assert_eq!(actual.len(), 100);
    }

    #[test]
    fn test_advance_guard() {
        let mut maze = Maze::new(PUZZLE_INPUT);
        assert_eq!(maze.guard_loc, Loc::new(4, 6));
        maze.advance_guard();
        assert_eq!(maze.guard_loc, Loc::new(4, 5));
        maze.advance_guard();
        maze.advance_guard();
        maze.advance_guard();
        maze.advance_guard();
        assert_eq!(maze.guard_direction, Direction::North);
        assert_eq!(maze.guard_loc, Loc::new(4, 1));
        maze.advance_guard();
        assert_eq!(maze.guard_direction, Direction::East);
        assert_eq!(maze.guard_loc, Loc::new(4, 1));
        maze.advance_guard();
        assert_eq!(maze.guard_direction, Direction::East);
        assert_eq!(maze.guard_loc, Loc::new(5, 1));
    }

    #[test]
    fn test_patrol_guard() {
        let mut maze = Maze::new(PUZZLE_INPUT);
        maze.patrol_guard();
        let actual = maze.count_visited();
        assert_eq!(actual, 41);
    }
}
