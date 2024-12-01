mod shared;
use std::convert::TryFrom;
use shared::{InputList, LocationList, PUZZLE_INPUT};

fn main() {
    println!("{}", get_total_similarity(PUZZLE_INPUT));
}

fn get_total_similarity(inp: &str) -> usize {
    let (loc_list_1, loc_list_2) = InputList::new(inp).parse_location_lists();
    loc_list_1.compare_similarity(&loc_list_2)
}

impl LocationList {
    fn compare_similarity(&self, other: &LocationList) -> usize {
        let mut total_similarity = 0;
        for x in self.0.iter() {
            let similarity_count = other.0.iter().filter(|item| item == &x).count();
            total_similarity += usize::try_from(*x).unwrap() * similarity_count;
        }
        total_similarity
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_similarity() {
        let actual = get_total_similarity(PUZZLE_INPUT);
        assert_eq!(actual, 31);
    }
}
