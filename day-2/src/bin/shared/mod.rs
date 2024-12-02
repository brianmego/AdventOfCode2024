#[cfg(test)]
pub const PUZZLE_INPUT: &str = include_str!("../../data/sample_input.txt");

#[cfg(not(test))]
pub const PUZZLE_INPUT: &str = include_str!("../../data/puzzle_input.txt");

pub struct InputList<'a>(pub &'a str);

impl<'a> InputList<'a> {
    pub fn new(inp: &'a str) -> Self {
        Self(inp)
    }
    pub fn get_reports(&self) -> Vec<Report> {
        self.0.lines().into_iter().map(|x| {
            Report(
                x.split_whitespace()
                    .into_iter()
                    .map(|level| level.parse().expect("known input")).collect(),
            )
        }).collect()
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Report(Vec<usize>);
impl Report {
    pub fn get_levels(&self) -> Vec<usize> {
        self.0.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let inp = InputList(PUZZLE_INPUT);
        let actual = inp.get_reports();
        let expected = vec![
            Report(vec![7, 6, 4, 2, 1]),
            Report(vec![1, 2, 7, 8, 9]),
            Report(vec![9, 7, 6, 2, 1]),
            Report(vec![1, 3, 2, 4, 5]),
            Report(vec![8, 6, 4, 4, 1]),
            Report(vec![1, 3, 6, 7, 9])
        ];
        assert_eq!(actual, expected);
    }
}
