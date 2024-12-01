use std::str::FromStr;

/// The number of lines in the problem input.
const LINES: usize = 1000;

/// The two lists in the input.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Data {
    left: Vec<u32>,
    right: Vec<u32>,
}

impl Data {
    pub fn with_capacity(capacity: usize) -> Self {
        Self {
            left: Vec::with_capacity(capacity),
            right: Vec::with_capacity(capacity),
        }
    }

    pub fn sort_unstable(&mut self) {
        self.left.sort_unstable();
        self.right.sort_unstable();
    }

    /// Computes the solution for part 1 of the problem.
    pub fn total_difference(mut self) -> u32 {
        self.sort_unstable();

        self.left
            .into_iter()
            .zip(self.right)
            .fold(0u32, |total, (left, right)| total + left.abs_diff(right))
    }
}

impl FromStr for Data {
    type Err = std::num::ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut data = Data::with_capacity(LINES);
        let mut raw_digits = s.split_whitespace();

        // we assume the input lists are of equal length, so we can always
        // take two elements at a time
        loop {
            match raw_digits.next() {
                None => break,
                Some(first) => {
                    let first = first.parse::<u32>()?;
                    let second = raw_digits.next().unwrap().parse::<u32>()?;

                    data.left.push(first);
                    data.right.push(second);
                }
            }
        }

        Ok(data)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example() {
        const EXAMPLE: &str = r#"
            3   4
            4   3
            2   5
            1   3
            3   9
            3   3
            "#;

        let data: Data = EXAMPLE.parse().unwrap();
        assert_eq!(data.total_difference(), 11);
    }

    #[test]
    fn part_1() {
        let source = std::fs::read_to_string("input/day01.txt").unwrap();
        let data: Data = source.parse().unwrap();
        assert_eq!(data.total_difference(), 1320851);
    }
}
