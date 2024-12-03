mod shared;
use shared::{InputFile, OpType, Operation, PUZZLE_INPUT};
use regex::Regex;

fn main() {
    let inp = InputFile(PUZZLE_INPUT);
    println!("{}", inp.run_all_ops())
}

struct Accum {
    enabled: bool,
    total: usize,
}
impl Default for Accum {
    fn default() -> Self {
        Self {
            enabled: true,
            total: 0,
        }
    }
}
impl Accum {
    fn enable(&mut self) {
        self.enabled = true;
    }
    fn disable(&mut self) {
        self.enabled = false;
    }
    fn add(&mut self, x: usize) {
        if self.enabled {
            self.total += x;
        }
    }
}

impl<'a> InputFile<'a> {
    pub fn run_all_ops(&self) -> usize {
        let mut accum = Accum::default();
        for i in self.get_all_ops() {
            match i.op_type() {
                OpType::Mul(x, y) => accum.add(x * y),
                OpType::Enable => accum.enable(),
                OpType::Disable => accum.disable(),
            }
        }
        accum.total
    }
    fn get_all_ops(&self) -> Vec<Operation> {
        let re = Regex::new(r"don't\(\)|do\(\)|mul\(\d+,\d+\)").expect("hardcoded");
        let mul_re = Regex::new(r"\((\d+),(\d+)\)").expect("hardcoded");
        re.captures_iter(self.0)
            .map(|caps| {
                let str_op = caps.get(0).expect("known input").as_str();
                if str_op.starts_with("mul") {
                    let mul_match = mul_re.captures(str_op).expect("known input");
                    let x = mul_match
                        .get(1)
                        .unwrap()
                        .as_str()
                        .parse()
                        .expect("known input");
                    let y = mul_match
                        .get(2)
                        .unwrap()
                        .as_str()
                        .parse()
                        .expect("known input");
                    Operation::new(OpType::Mul(x, y))
                } else if str_op.starts_with("don't") {
                    Operation::new(OpType::Disable)
                } else if str_op.starts_with("do") {
                    Operation::new(OpType::Enable)
                } else {
                    unreachable!();
                }
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    pub const PUZZLE_INPUT_2: &str = include_str!("../data/sample_input_2.txt");

    #[test]
    fn test_get_all_ops() {
        let inp = InputFile(PUZZLE_INPUT_2);
        let actual = inp.get_all_ops();
        assert_eq!(actual.len(), 6);
        assert_eq!(
            actual,
            vec![
                Operation::new(OpType::Mul(2, 4)),
                Operation::new(OpType::Disable),
                Operation::new(OpType::Mul(5, 5)),
                Operation::new(OpType::Mul(11, 8)),
                Operation::new(OpType::Enable),
                Operation::new(OpType::Mul(8, 5)),
            ]
        );
    }

    #[test]
    fn test_run_all_ops() {
        let inp = InputFile(PUZZLE_INPUT_2);
        let actual = inp.run_all_ops();
        assert_eq!(actual, 48);
    }
}
