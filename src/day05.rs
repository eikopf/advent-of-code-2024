use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rule {
    first: u8,
    second: u8,
}

impl FromStr for Rule {
    type Err = ParseRuleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (lhs, rhs) = s.split_once('|').ok_or(ParseRuleError::MissingBar)?;

        Ok(Self {
            first: lhs.parse::<u8>()?,
            second: rhs.parse::<u8>()?,
        })
    }
}

#[derive(Debug, Default, Clone)]
pub enum ParseRuleError {
    Int(std::num::ParseIntError),
    MissingBar,
    #[default]
    Unknown,
}

impl From<std::num::ParseIntError> for ParseRuleError {
    fn from(v: std::num::ParseIntError) -> Self {
        Self::Int(v)
    }
}

#[derive(Debug, Clone)]
pub struct RuleTable {
    successors: HashMap<u8, HashSet<u8>>,
}

impl RuleTable {
    fn check_order(&self, first: u8, second: u8) -> bool {
        self.successors
            .get(&first)
            .is_some_and(|set| set.contains(&second))
    }
}

impl FromStr for RuleTable {
    type Err = ParseRuleError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut successors = HashMap::<_, HashSet<_>>::with_capacity(100);

        for rule in s.split('\n') {
            let Rule { first, second } = rule.parse()?;

            if let Some(set) = successors.get_mut(&first) {
                set.insert(second);
            } else {
                successors.insert(first, HashSet::from([second]));
            }
        }

        Ok(Self { successors })
    }
}

/// Computes the solution to part 1.
pub fn sum_of_middle_page_numbers(input: &str) -> usize {
    let (rules, updates) = input.split_once("\n\n").unwrap();
    let rules = rules.parse::<RuleTable>().unwrap();

    updates
        .split_terminator("\n")
        .map(|raw_update| {
            raw_update
                .split(',')
                .map(u8::from_str)
                .map(Result::unwrap)
                .collect::<Box<[_]>>()
        })
        .filter(|update| {
            let (first, tail) = update.split_first().unwrap();

            tail.iter()
                .try_fold(first, |prev, next| {
                    Some(next).filter(|_| rules.check_order(*prev, *next))
                })
                .is_some()
        })
        .map(|update| update[update.len() / 2] as usize)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47"#;

    const INPUT: &str = include_str!("../input/day05.txt");

    #[test]
    fn example_part_1() {
        assert_eq!(sum_of_middle_page_numbers(EXAMPLE), 143);
    }

    #[test]
    fn part_1() {
        assert_eq!(sum_of_middle_page_numbers(INPUT), 6242);
    }
}
