
#[cfg(test)]
pub const PUZZLE_INPUT: &str = include_str!("../../data/sample_input.txt");

#[cfg(not(test))]
pub const PUZZLE_INPUT: &str = include_str!("../../data/puzzle_input.txt");

pub struct InputList<'a>(pub &'a str);

#[derive(Debug, PartialEq, Eq)]
pub struct LocationList(pub Vec<isize>);

impl<'a> InputList<'a> {
    pub fn parse_location_lists(&self) -> (LocationList, LocationList) {
        let lines = self.0.lines();
        let mut loc_list_1 = LocationList(vec![]);
        let mut loc_list_2 = LocationList(vec![]);
        for line in lines {
            let (left, right) = line.split_once("   ").expect("known input");
            loc_list_1
                .0
                .push(left.parse::<isize>().expect("known input"));
            loc_list_2
                .0
                .push(right.parse::<isize>().expect("known input"));
        }
        (loc_list_1, loc_list_2)
    }

    pub fn new(inp: &'a str) -> Self {
        Self(inp)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_location_lists() {
        let inp = InputList::new(PUZZLE_INPUT);
        let actual = inp.parse_location_lists();
        let expected = (
            LocationList(vec![3, 4, 2, 1, 3, 3]),
            LocationList(vec![4, 3, 5, 3, 9, 3]),
        );
        assert_eq!(actual, expected);
    }
}
