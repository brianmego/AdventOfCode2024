use std::collections::HashMap;

#[cfg(test)]
pub const PUZZLE_INPUT: &str = include_str!("../../data/sample_input.txt");

#[cfg(not(test))]
pub const PUZZLE_INPUT: &str = include_str!("../../data/puzzle_input.txt");

fn parse_input(inp: &str) -> (RuleSet, Vec<Update>) {
    let (rule_inp, update_inp) = inp.split_once("\n\n").expect("Known input");
    let mut ruleset = RuleSet::default();
    for i in rule_inp.lines() {
        let (page, dependent) = i.split_once('|').unwrap();
        ruleset.add_rule(Rule::new(page.parse().unwrap(), dependent.parse().unwrap()));
    }
    let updates: Vec<Update> = update_inp
        .lines()
        .into_iter()
        .map(|l| {
            Update::new(
                l.split(",")
                    .into_iter()
                    .map(|page| page.parse().unwrap())
                    .collect(),
            )
        })
        .collect();
    (ruleset, updates)
}

pub fn sum_valid_middles(inp: &str) -> usize {
    let (ruleset, updates) = parse_input(inp);
    updates
        .iter()
        .filter(|u| u.is_valid(&ruleset))
        .map(|u| usize::from(u.get_middle()))
        .sum()
}

struct Rule {
    page: u8,
    dependent: u8,
}

impl Rule {
    fn new(page: u8, dependent: u8) -> Self {
        Self { page, dependent }
    }
}

#[derive(Default, Debug, Eq, PartialEq)]
struct RuleSet(HashMap<u8, Vec<u8>>);
impl RuleSet {
    fn add_rule(&mut self, rule: Rule) {
        if !self.0.contains_key(&rule.page) {
            self.0.insert(rule.page, vec![]);
        }
        let _ = &mut self.0.get_mut(&rule.page).unwrap().push(rule.dependent);
    }
    fn len(&self) -> usize {
        self.0.len()
    }
    fn get_rule(&self, page: u8) -> Option<&Vec<u8>> {
        self.0.get(&page)
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Update {
    pages: Vec<u8>,
    ruleset: RuleSet
}
impl Update {
    fn new(pages: Vec<u8>) -> Self {
        Self { pages, ruleset: RuleSet::default() }
    }

    fn is_valid(&self, ruleset: &RuleSet) -> bool {
        let mut processed_pages: Vec<u8> = vec![];
        for page in &self.pages {
            if let Some(dependents) = ruleset.get_rule(*page) {
                for dependent_page in dependents {
                    if processed_pages.contains(dependent_page) {
                        return false;
                    }
                }
            }
            processed_pages.push(*page)
        }
        true
    }

    fn get_middle(&self) -> u8 {
        self.pages[(self.pages.len() - 1) / 2]
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn test_parse_input() {
        let (ruleset, updates) = parse_input(PUZZLE_INPUT);
        assert_eq!(ruleset.len(), 6);
        assert_eq!(ruleset.get_rule(47), Some(&vec![53, 13, 61, 29]));
        assert_eq!(ruleset.get_rule(29), Some(&vec![13]));
        assert_eq!(ruleset.get_rule(61), Some(&vec![13, 53, 29]));
        assert_eq!(updates.len(), 6);
        assert_eq!(updates[0], Update::new(vec![75, 47, 61, 53, 29]));
    }

    #[test_case(Update::new(vec![75, 47, 61, 53, 29]), true)]
    #[test_case(Update::new(vec![97, 61, 53, 29, 13]), true)]
    #[test_case(Update::new(vec![75, 29, 13]), true)]
    #[test_case(Update::new(vec![75, 97, 47, 61, 53]), false)]
    #[test_case(Update::new(vec![61, 13, 29]), false)]
    #[test_case(Update::new(vec![97, 13, 75, 29, 47]), false)]
    fn test_is_valid(inp: Update, is_valid: bool) {
        let (ruleset, _) = parse_input(PUZZLE_INPUT);
        assert_eq!(inp.is_valid(&ruleset), is_valid);
    }

    #[test_case(Update::new(vec![75, 47, 61, 53, 29]), 61)]
    #[test_case(Update::new(vec![97, 61, 53, 29, 13]), 53)]
    #[test_case(Update::new(vec![75, 29, 13]), 29)]
    #[test_case(Update::new(vec![75, 97, 47, 61, 53]), 47)]
    #[test_case(Update::new(vec![61, 13, 29]), 13)]
    #[test_case(Update::new(vec![97, 13, 75, 29, 47]), 75)]
    fn test_get_middle(inp: Update, expected: u8) {
        assert_eq!(inp.get_middle(), expected);
    }

    #[test]
    fn test_sum_valid_middles() {
        let actual = sum_valid_middles(PUZZLE_INPUT);
        assert_eq!(actual, 143)
    }
}
