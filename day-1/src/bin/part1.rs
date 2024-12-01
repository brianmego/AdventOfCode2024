mod shared;
use std::convert::TryFrom;
use shared::{InputList, LocationList, PUZZLE_INPUT};

fn main() {
    println!("{}", get_total_distance(PUZZLE_INPUT));
}

fn get_total_distance(inp: &str) -> usize {
    let (loc_list_1, loc_list_2) = InputList::new(inp).parse_location_lists();
    loc_list_1.compare_distance(&loc_list_2)
}

impl LocationList {
    fn compare_distance(&self, other: &LocationList) -> usize {
        let mut sorted1 = self.0.clone();
        let mut sorted2 = other.0.clone();
        sorted1.sort();
        sorted2.sort();
        let mut total_diff = 0;
        for (i, x) in sorted1.iter().enumerate() {
            let distance = (x - sorted2[i]).abs();
            total_diff += distance;
        }
        usize::try_from(total_diff).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_distance() {
        let actual = get_total_distance(PUZZLE_INPUT);
        assert_eq!(actual, 11);
    }
}
