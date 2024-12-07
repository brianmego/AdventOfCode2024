use itertools::iproduct;
use std::fmt::Debug;

#[cfg(test)]
pub const PUZZLE_INPUT: &str = include_str!("../../data/sample_input.txt");

#[cfg(not(test))]
pub const PUZZLE_INPUT: &str = include_str!("../../data/puzzle_input.txt");

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Operator {
    Add,
    Mult,
}
impl Operator {
    pub fn all() -> Vec<Operator> {
        vec![Operator::Add, Operator::Mult]
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct Equation {
    test_value: usize,
    inputs: Vec<usize>,
}

impl From<&str> for Equation {
    fn from(value: &str) -> Self {
        let (test_value, inputs_str) = value.split_once(": ").unwrap();
        let test_value: usize = test_value.parse().unwrap();
        let inputs: Vec<usize> = inputs_str.split(' ').map(|x| x.parse().unwrap()).collect();
        Self { test_value, inputs }
    }
}

impl Equation {
    pub fn get_test_value(&self) -> usize {
        self.test_value
    }
    fn could_be_valid(&self, ops: Vec<Operator>) -> bool {
        let possible_combos = get_possible_combinations(self.inputs.len() - 1, Operator::all());
        for combo in possible_combos.iter() {
            let mut calibration_result = self.inputs[0];
            for (i, operator) in combo.iter().enumerate() {
                match operator {
                    Operator::Add => calibration_result += self.inputs[i + 1],
                    Operator::Mult => calibration_result *= self.inputs[i + 1],
                };
            }
            if calibration_result == self.test_value {
                return true;
            }
        }
        return false;
    }
}

fn get_possible_combinations<T: Debug + Clone>(len: usize, items: Vec<T>) -> Vec<Vec<T>> {
    let mut combinations = vec![];
    match len {
        1 => {
            for item in items {
                combinations.push(vec![item])
            }
        }
        2 => {
            for (i, j) in iproduct!(items.clone(), items.clone()) {
                combinations.push(vec![i, j])
            }
        }
        3 => {
            for (i, j, k) in iproduct!(items.clone(), items.clone(), items.clone()) {
                combinations.push(vec![i, j, k])
            }
        }
        4 => {
            for (i, j, k, l) in
                iproduct!(items.clone(), items.clone(), items.clone(), items.clone(),)
            {
                combinations.push(vec![i, j, k, l])
            }
        }
        5 => {
            for (i, j, k, l, m) in iproduct!(
                items.clone(),
                items.clone(),
                items.clone(),
                items.clone(),
                items.clone(),
            ) {
                combinations.push(vec![i, j, k, l, m])
            }
        }
        6 => {
            for (i, j, k, l, m, n) in iproduct!(
                items.clone(),
                items.clone(),
                items.clone(),
                items.clone(),
                items.clone(),
                items.clone(),
            ) {
                combinations.push(vec![i, j, k, l, m, n])
            }
        }
        7 => {
            for (i, j, k, l, m, n, o) in iproduct!(
                items.clone(),
                items.clone(),
                items.clone(),
                items.clone(),
                items.clone(),
                items.clone(),
                items.clone(),
            ) {
                combinations.push(vec![i, j, k, l, m, n, o])
            }
        }
        8 => {
            for (i, j, k, l, m, n, o, p) in iproduct!(
                items.clone(),
                items.clone(),
                items.clone(),
                items.clone(),
                items.clone(),
                items.clone(),
                items.clone(),
                items.clone(),
            ) {
                combinations.push(vec![i, j, k, l, m, n, o, p])
            }
        }
        9 => {
            for (i, j, k, l, m, n, o, p, q) in iproduct!(
                items.clone(),
                items.clone(),
                items.clone(),
                items.clone(),
                items.clone(),
                items.clone(),
                items.clone(),
                items.clone(),
                items.clone(),
            ) {
                combinations.push(vec![i, j, k, l, m, n, o, p, q])
            }
        }
        10 => {
            for (i, j, k, l, m, n, o, p, q, r) in iproduct!(
                items.clone(),
                items.clone(),
                items.clone(),
                items.clone(),
                items.clone(),
                items.clone(),
                items.clone(),
                items.clone(),
                items.clone(),
                items.clone(),
            ) {
                combinations.push(vec![i, j, k, l, m, n, o, p, q, r])
            }
        }
        11 => {
            for (i, j, k, l, m, n, o, p, q, r, s) in iproduct!(
                items.clone(),
                items.clone(),
                items.clone(),
                items.clone(),
                items.clone(),
                items.clone(),
                items.clone(),
                items.clone(),
                items.clone(),
                items.clone(),
                items.clone(),
            ) {
                combinations.push(vec![i, j, k, l, m, n, o, p, q, r, s])
            }
        }
        _ => {
            panic!("Unsupported number: {}", len)
        }
    }
    combinations
}

#[derive(Debug)]
pub struct CalibrationList {
    equations: Vec<Equation>,
}

impl CalibrationList {
    fn len(&self) -> usize {
        self.equations.len()
    }
    pub fn get_valid_equations(&self, valid_ops: Vec<Operator>) -> Vec<&Equation> {
        self.equations
            .iter()
            .filter(|e| e.could_be_valid(valid_ops.clone()))
            .collect()
    }

    pub fn get_total_calibration_result(&self) -> usize {
        self.get_valid_equations(Operator::all())
            .iter()
            .map(|e| e.get_test_value())
            .sum()
    }
}

impl From<&str> for CalibrationList {
    fn from(value: &str) -> Self {
        let equations: Vec<Equation> = PUZZLE_INPUT
            .lines()
            .into_iter()
            .map(|l| Equation::from(l))
            .collect();
        Self { equations }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn test_parse_input() {
        let actual = CalibrationList::from(PUZZLE_INPUT);
        assert_eq!(actual.len(), 9);
    }

    #[test]
    fn test_parse_equation() {
        let actual = Equation::from("21037: 9 7 18 13");
        assert_eq!(
            actual,
            Equation {
                test_value: 21037,
                inputs: vec![9, 7, 18, 13]
            }
        )
    }

    #[test]
    fn test_get_possible_combinations_one() {
        let actual = get_possible_combinations(1, vec![Operator::Add, Operator::Mult]);
        assert_eq!(actual, vec![vec![Operator::Add], vec![Operator::Mult],]);
    }
    #[test]
    fn test_get_possible_combinations_two() {
        let actual = get_possible_combinations(2, vec![Operator::Add, Operator::Mult]);
        assert_eq!(
            actual,
            vec![
                vec![Operator::Add, Operator::Add],
                vec![Operator::Add, Operator::Mult],
                vec![Operator::Mult, Operator::Add],
                vec![Operator::Mult, Operator::Mult],
            ]
        );
    }
    #[test]
    fn test_get_possible_combinations_three() {
        let actual = get_possible_combinations(3, vec![Operator::Add, Operator::Mult]);
        assert_eq!(
            actual,
            vec![
                vec![Operator::Add, Operator::Add, Operator::Add],
                vec![Operator::Add, Operator::Add, Operator::Mult],
                vec![Operator::Add, Operator::Mult, Operator::Add],
                vec![Operator::Add, Operator::Mult, Operator::Mult],
                vec![Operator::Mult, Operator::Add, Operator::Add],
                vec![Operator::Mult, Operator::Add, Operator::Mult],
                vec![Operator::Mult, Operator::Mult, Operator::Add],
                vec![Operator::Mult, Operator::Mult, Operator::Mult],
            ]
        );
    }
    #[test_case("190: 10 19", true)]
    #[test_case("3267: 81 40 27", true)]
    #[test_case("83: 17 5", false)]
    #[test_case("156: 15 6", false)]
    #[test_case("7290: 6 8 6 15", false)]
    #[test_case("161011: 16 10 13", false)]
    #[test_case("192: 17 8 14", false)]
    #[test_case("21037: 9 7 18 13", false)]
    #[test_case("292: 11 6 16 20", true)]
    fn test_could_be_valid(inp: &str, expected: bool) {
        let equation = Equation::from(inp);
        let actual = equation.could_be_valid(Operator::all());
        assert_eq!(actual, expected);
    }
}
