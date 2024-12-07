mod shared;
use shared::{CalibrationList, PUZZLE_INPUT};

fn main() {
    let calibration_list = CalibrationList::from(PUZZLE_INPUT);
    let actual = calibration_list.get_total_calibration_result();
    println!("{}", actual);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total_calibration_result() {
        let calibration_list = CalibrationList::from(PUZZLE_INPUT);
        let actual = calibration_list.get_total_calibration_result();
        assert_eq!(actual, 3749);
    }
}
