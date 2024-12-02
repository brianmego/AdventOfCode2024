mod shared;
use shared::{InputList, Report, PUZZLE_INPUT};

fn main() {
    let reports = InputList(PUZZLE_INPUT).get_reports();
    println!("{}", get_safe_count(reports));
}

fn get_safe_count(reports: Vec<Report>) -> usize {
    reports.iter().filter(|r| r.is_safe(1)).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_count_safe_reports() {
        let reports = InputList(PUZZLE_INPUT).get_reports();
        let actual = get_safe_count(reports);
        assert_eq!(actual, 4);
    }
}
