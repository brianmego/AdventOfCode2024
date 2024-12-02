mod shared;
use shared::{InputList, Report, PUZZLE_INPUT};

fn main() {
    let reports = InputList(PUZZLE_INPUT).get_reports();
    println!("{}", get_safe_count(reports));
}

#[derive(PartialEq, Eq)]
enum AccelerationType {
    Increasing,
    Decreasing,
    Same,
}
struct Movement {
    safe: bool,
    direction: AccelerationType,
}

impl Movement {
    fn new(safe: bool, direction: AccelerationType) -> Self {
        Self { safe, direction }
    }
}

impl Report {
    fn is_safe(&self) -> bool {
        let levels = self.get_levels();
        let mut prev_acceleration: Option<AccelerationType> = None;
        let mut prev_level = levels[0];
        for level in levels[1..].iter() {
            let safe = level.abs_diff(prev_level) <= 3;
            let movement = match level.cmp(&prev_level) {
                std::cmp::Ordering::Less => Movement::new(safe, AccelerationType::Decreasing),
                std::cmp::Ordering::Equal => Movement::new(false, AccelerationType::Same),
                std::cmp::Ordering::Greater => Movement::new(safe, AccelerationType::Increasing),
            };
            match movement.safe {
                true => {
                    match prev_acceleration {
                        Some(last) => {
                            if last != movement.direction {
                                return false;
                            }
                        }
                        None => {}
                    }
                    prev_level = *level;
                    prev_acceleration = Some(movement.direction);
                }
                false => return false,
            }
        }
        true
    }
}

fn get_safe_count(reports: Vec<Report>) -> usize {
    reports.iter().filter(|r| r.is_safe()).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_safe_report() {
        let reports = InputList(PUZZLE_INPUT).get_reports();
        assert_eq!(reports[0].is_safe(), true);
        assert_eq!(reports[1].is_safe(), false);
        assert_eq!(reports[2].is_safe(), false);
        assert_eq!(reports[3].is_safe(), false);
        assert_eq!(reports[4].is_safe(), false);
        assert_eq!(reports[5].is_safe(), true);
    }

    #[test]
    fn test_count_safe_reports() {
        let reports = InputList(PUZZLE_INPUT).get_reports();
        let actual = get_safe_count(reports);
        assert_eq!(actual, 2);
    }
}
