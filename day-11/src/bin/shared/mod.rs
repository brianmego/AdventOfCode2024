#[cfg(test)]
pub const PUZZLE_INPUT: &str = include_str!("../../data/sample_input.txt");

#[cfg(not(test))]
pub const PUZZLE_INPUT: &str = include_str!("../../data/puzzle_input.txt");

#[derive(Clone, Debug, PartialEq, Eq)]
struct MagicStone(String);
impl MagicStone {
    fn blink(&self) -> Vec<MagicStone> {
        if self.0 == "0" {
            vec![MagicStone::from("1")]
        } else if self.0.len() % 2 == 0 {
            let (left, right) = self.0.split_at(self.0.len() / 2);
            vec![MagicStone::from(left), MagicStone::from(right)]
        } else {
            let stone_as_num: usize = self.0.parse().unwrap();
            let mult_stone = (stone_as_num * 2024).to_string();
            vec![MagicStone(mult_stone)]
        }
    }
}
impl From<&str> for MagicStone {
    fn from(value: &str) -> Self {
        Self(value.to_string())
    }
}

#[derive(Debug, PartialEq, Eq)]
struct StoneList {
    stones: Vec<MagicStone>,
}

impl StoneList {
    fn len(&self) -> usize {
        self.stones.len()
    }
    fn blink(&self, count: usize) -> StoneList {
        let mut stones = self.stones.clone();
        for i in 0..count {
            dbg!(i, &stones);
            let mut new_stones = vec![];
            for stone in stones {
                let after_blink = stone.blink();
                new_stones.extend(after_blink)
            }
            stones = new_stones.clone()
        }
        StoneList { stones }
    }
}
impl From<&str> for StoneList {
    fn from(value: &str) -> Self {
        let stones: Vec<MagicStone> = value
            .split_whitespace()
            .map(|x| MagicStone::from(x))
            .collect();
        Self { stones }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn test_parse_input() {
        let actual = StoneList::from(PUZZLE_INPUT);
        assert_eq!(actual.len(), 5);
    }

    #[test_case("0", vec![MagicStone::from("1")])]
    #[test_case("1", vec![MagicStone::from("2024")])]
    #[test_case("10", vec![MagicStone::from("1"), MagicStone::from("0")])]
    #[test_case("99", vec![MagicStone::from("9"), MagicStone::from("9")])]
    #[test_case("999", vec![MagicStone::from("2021976")])]
    fn test_blink_conditions(inp: &str, expected: Vec<MagicStone>) {
        let stone = MagicStone::from(inp);
        let actual = stone.blink();
        assert_eq!(actual, expected);
    }

    #[test_case("0 1 10 99 999", 1, "1 2024 1 0 9 9 2021976")]
    #[test_case("125 17", 1, "253000 1 7")]
    #[test_case("125 17", 2, "253 0 2024 14168")]
    // #[test_case("125 17", 6, "2097446912 14168 4048 2 0 2 4 40 48 2024 40 48 80 96 2 8 6 7 6 0 3 2")]
    fn test_list_blinks(inp: &str, blinks: usize, expected: &str) {
        let stone_list = StoneList::from(inp);
        let expected_list = StoneList::from(expected);
        let actual = stone_list.blink(blinks);
        assert_eq!(actual, expected_list)
    }
}
