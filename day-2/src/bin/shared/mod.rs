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
        self.0
            .lines()
            .into_iter()
            .map(|x| {
                Report(
                    x.split_whitespace()
                        .into_iter()
                        .map(|level| level.parse().expect("known input"))
                        .collect(),
                )
            })
            .collect()
    }
}
#[derive(PartialEq, Eq, Debug)]
enum AccelerationType {
    Increasing,
    Decreasing,
    Same,
}
#[derive(Debug)]
struct Movement {
    safe: bool,
    direction: AccelerationType,
}

impl Movement {
    fn new(safe: bool, direction: AccelerationType) -> Self {
        Self { safe, direction }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Report(Vec<usize>);
impl Report {
    fn get_levels(&self) -> Vec<usize> {
        self.0.clone()
    }
    pub fn is_safe(&self, tolerance: u8) -> bool {
        let levels = self.get_levels();
        let mut prev_level = levels[0];
        let mut movements: Vec<Movement> = vec![];
        let mut unsafe_count = 0;
        for level in levels[1..].iter() {
            let safe = level.abs_diff(prev_level) <= 3;
            let movement = match level.cmp(&prev_level) {
                std::cmp::Ordering::Less => Movement::new(safe, AccelerationType::Decreasing),
                std::cmp::Ordering::Equal => Movement::new(false, AccelerationType::Same),
                std::cmp::Ordering::Greater => Movement::new(safe, AccelerationType::Increasing),
            };
            match movement.safe {
                true => {
                    let prev_acceleration = movements.last();
                    let is_safe = match prev_acceleration {
                        Some(last) => {
                            if last.direction != movement.direction {
                                unsafe_count += 1;
                                false
                            } else {
                                true
                            }
                        }
                        None => true,
                    };
                    if is_safe == true {
                        prev_level = *level;
                        movements.push(movement);
                    }
                }
                false => unsafe_count += 1,
            }
            if unsafe_count > tolerance {
                return false;
            }
        }
        dbg!(movements);
        true
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
            Report(vec![1, 3, 6, 7, 9]),
        ];
        assert_eq!(actual, expected);
    }

    #[test]
    fn test_is_safe_report() {
        let reports = InputList(PUZZLE_INPUT).get_reports();
        assert_eq!(reports[0].is_safe(0), true);
        assert_eq!(reports[1].is_safe(0), false);
        assert_eq!(reports[2].is_safe(0), false);
        assert_eq!(reports[3].is_safe(0), false);
        assert_eq!(reports[4].is_safe(0), false);
        assert_eq!(reports[5].is_safe(0), true);
    }

    #[test]
    fn test_is_safe_report_tolerance_1() {
        let reports = InputList(PUZZLE_INPUT).get_reports();
        assert_eq!(reports[0].is_safe(1), true);
        assert_eq!(reports[1].is_safe(1), false);
        assert_eq!(reports[2].is_safe(1), false);
        assert_eq!(reports[3].is_safe(1), true);
        assert_eq!(reports[4].is_safe(1), true);
        assert_eq!(reports[5].is_safe(1), true);
        assert!(false);
    }
}
