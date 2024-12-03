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
    prev_level: usize,
    level: usize,
}

impl Movement {
    fn new(safe: bool, direction: AccelerationType, prev_level: usize, level: usize) -> Self {
        Self {
            safe,
            direction,
            prev_level,
            level,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct Report(Vec<usize>);
impl Report {
    fn get_levels(&self) -> Vec<usize> {
        self.0.clone()
    }
    fn check_alternative_safeties(&self, index_to_remove: usize, new_tolerance: u8) -> bool {
        let mut clone1 = self.clone();
        let mut clone2 = self.clone();
        clone1.0.remove(index_to_remove);
        clone2.0.remove(index_to_remove + 1);
        match index_to_remove > 0 {
            true => {
                let mut clone3 = self.clone();
                clone3.0.remove(index_to_remove - 1);
                clone1.is_safe(new_tolerance)
                    || clone2.is_safe(new_tolerance)
                    || clone3.is_safe(new_tolerance)
            }
            false => clone1.is_safe(new_tolerance) || clone2.is_safe(new_tolerance),
        }
    }
    pub fn is_safe(&self, tolerance: u8) -> bool {
        let levels = self.get_levels();
        let mut prev_level = levels[0];
        let mut movements: Vec<Movement> = vec![];
        for (i, level) in levels[1..].iter().enumerate() {
            let safe = level.abs_diff(prev_level) <= 3;
            let movement = match level.cmp(&prev_level) {
                std::cmp::Ordering::Less => {
                    Movement::new(safe, AccelerationType::Decreasing, prev_level, *level)
                }
                std::cmp::Ordering::Equal => {
                    Movement::new(false, AccelerationType::Same, prev_level, *level)
                }
                std::cmp::Ordering::Greater => {
                    Movement::new(safe, AccelerationType::Increasing, prev_level, *level)
                }
            };
            match movement.safe {
                true => {
                    let prev_acceleration = movements.last();
                    let is_safe = match prev_acceleration {
                        Some(last) => {
                            if last.direction != movement.direction {
                                match tolerance > 0 {
                                    true => {
                                        return self.check_alternative_safeties(i, tolerance - 1)
                                    }
                                    false => {
                                        return false;
                                    }
                                }
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
                false => match tolerance > 0 {
                    true => return self.check_alternative_safeties(i, tolerance - 1),
                    false => {
                        return false;
                    }
                },
            }
        }
        true
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

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
    }

    #[test_case(Report(vec![1, 1, 2, 3, 4, 5]))]
    #[test_case(Report(vec![1, 2, 3, 4, 5, 5]))]
    #[test_case(Report(vec![1, 2, 3, 4, 5, 4]))]
    #[test_case(Report(vec![1, 11, 13, 14, 15]))]
    #[test_case(Report(vec![2, 1, 3, 4, 5]))]
    #[test_case(Report(vec![2, 5, 3, 4, 5]))]
    #[test_case(Report(vec![81, 84, 81, 80, 77, 75, 72, 69]))]
    fn test_is_safe_report_tolerance_1_valid_edgecases(inp: Report) {
        assert_eq!(inp.is_safe(1), true);
    }
}
