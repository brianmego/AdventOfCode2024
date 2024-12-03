use regex::Regex;

#[cfg(test)]
pub const PUZZLE_INPUT: &str = include_str!("../../data/sample_input.txt");

#[cfg(not(test))]
pub const PUZZLE_INPUT: &str = include_str!("../../data/puzzle_input.txt");

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum OpType {
    Mul(usize, usize),
    Enable,
    Disable,
}
#[derive(Debug, PartialEq, Eq)]
pub struct Operation {
    op_type: OpType,
}

impl Operation {
    pub fn new(op_type: OpType) -> Self {
        Self { op_type }
    }
    fn run(&self) -> usize {
        match self.op_type {
            OpType::Mul(x, y) => x * y,
            OpType::Enable => todo!(),
            OpType::Disable => todo!(),
        }
    }
    pub fn op_type(&self) -> OpType {
        self.op_type
    }
}

pub struct InputFile<'a>(pub &'a str);
impl<'a> InputFile<'a> {
    fn get_mul_ops(&self) -> Vec<Operation> {
        let re = Regex::new(r"mul\((\d+),(\d+)\)").expect("hardcoded");
        let matches = re
            .captures_iter(self.0)
            .map(|caps| {
                let x: usize = caps.get(1).unwrap().as_str().parse().expect("known input");
                let y: usize = caps.get(2).unwrap().as_str().parse().expect("known input");
                Operation::new(OpType::Mul(x, y))
            })
            .collect();
        matches
    }

    pub fn sum_mul_ops(&self) -> usize {
        self.get_mul_ops().iter().map(|op| op.run()).sum()
    }

}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_input() {
        let inp = InputFile(PUZZLE_INPUT);
        let actual = inp.get_mul_ops();
        assert_eq!(actual.len(), 4);
    }

    #[test]
    fn test_sum_mul_ops() {
        let inp = InputFile(PUZZLE_INPUT);
        let actual = inp.sum_mul_ops();
        assert_eq!(actual, 161);
    }
}
