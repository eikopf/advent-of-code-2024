/// Computes the first difference of the given vector.
///
/// # Safety
/// We assume the input bytes are bounded above by 100, as in the
/// input data. Within this domain, u8 and i8 are bitwise identical.
/// We also assume `elems` is nonempty.
fn diff(mut elems: Vec<u8>) -> Vec<i8> {
    for i in 0..(elems.len() - 1) {
        let first = elems[i] as i8;
        let second = elems[i + 1] as i8;

        let difference = second - first;
        elems[i] = difference as u8;
    }

    // remove the last element
    elems.pop();

    // SAFETY: all u8s are valid i8s
    unsafe { std::mem::transmute(elems) }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i8)]
pub enum Direction {
    Increasing = 1,
    Decreasing = -1,
}

impl Direction {
    pub fn from_report(report: Vec<u8>) -> Option<Self> {
        let difference = diff(report);
        let (first, tail) = difference.split_first()?;

        // get the direction from the first element
        let direction = match first.signum() {
            -1 => Some(Self::Decreasing),
            1 => Some(Self::Increasing),
            _ => None,
        }?;

        // check that the first value is bounded appropriately
        if !(1..=3).contains(&first.unsigned_abs()) {
            return None;
        }

        // check that all remaining elements are correctly signed and bounded
        for d in tail {
            match d.signum() == (direction as i8) && (1..=3).contains(&d.unsigned_abs()) {
                false => return None,
                true => continue,
            }
        }

        Some(direction)
    }
}

/// Computes the solution to part 1.
pub fn count_safe_reports(reports: &str) -> usize {
    reports
        .split_terminator('\n')
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<u8>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|v| !v.is_empty())
        .map(Direction::from_report)
        .filter(Option::is_some)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"
        7 6 4 2 1
        1 2 7 8 9
        9 7 6 2 1
        1 3 2 4 5
        8 6 4 4 1
        1 3 6 7 9
        "#;

    const INPUT: &str = include_str!("../input/day02.txt");

    #[test]
    fn example_part1() {
        assert_eq!(count_safe_reports(EXAMPLE), 2);
    }

    #[test]
    fn part1() {
        assert_eq!(count_safe_reports(INPUT), 591);
    }
}
