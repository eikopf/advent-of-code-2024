/// Computes the first difference of the given vector.
///
/// # Safety
/// We assume the input bytes are bounded above by 100, as in the
/// input data. Within this domain, u8 and i8 are bitwise identical.
/// We also assume `elems` is nonempty.
unsafe fn diff(mut elems: Vec<u8>) -> Vec<i8> {
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

/// Returns the index of the single problematic item in the report (if any)
/// based on its first difference. If there are too many problems, we return
/// None. The returned index always points to the left edge of the difference.
fn find_first_problem(differences: &[i8]) -> Option<usize> {
    let (mut inc, mut dec) = (0, 0);

    // count the number of increases and decreases
    for (i, &d) in differences.iter().enumerate() {
        match d {
            1..=3 => inc += 1,
            -3..=-1 => dec += 1,
            _ => return Some(i),
        }
    }

    // irreconcilable number of sign changes
    if inc >= 2 && dec >= 2 {
        return None;
    }

    // no problems
    if inc == 0 || dec == 0 {
        return None;
    }

    // otherwise find the single outlier
    differences.iter().enumerate().find_map(|(i, &d)| {
        Some(i).filter(|_| {
            if inc > dec {
                d.is_negative()
            } else {
                d.is_positive()
            }
        })
    })
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(i8)]
pub enum Direction {
    Increasing = 1,
    Decreasing = -1,
}

impl TryFrom<i8> for Direction {
    type Error = ();

    fn try_from(value: i8) -> Result<Self, Self::Error> {
        match value.signum() {
            1 => Ok(Self::Increasing),
            -1 => Ok(Self::Decreasing),
            _ => Err(()),
        }
    }
}

impl Direction {
    pub fn from_report(report: Vec<u8>) -> Option<Self> {
        let differences = unsafe { diff(report) };
        let (first, tail) = differences.split_first()?;

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

    pub fn from_report_with_dampener(mut report: Vec<u8>) -> Option<Self> {
        let differences = unsafe { diff(report.clone()) };
        let problem = find_first_problem(&differences);

        // if we have a problem, try removing either side of the difference
        if let Some(i) = problem {
            let mut left = report.clone();
            left.remove(i);

            report.remove(i + 1);
            let right = report;

            Self::from_report(left).or_else(|| Self::from_report(right))
        } else {
            Self::from_report(report)
        }
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

/// Computes the solution to part 2.
pub fn count_safe_dampened_reports(reports: &str) -> usize {
    reports
        .split_terminator('\n')
        .map(|line| {
            line.split_whitespace()
                .map(|n| n.parse::<u8>().unwrap())
                .collect::<Vec<_>>()
        })
        .filter(|v| !v.is_empty())
        .map(Direction::from_report_with_dampener)
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
    fn example_part_1() {
        assert_eq!(count_safe_reports(EXAMPLE), 2);
    }

    #[test]
    fn part_1() {
        assert_eq!(count_safe_reports(INPUT), 591);
    }

    #[test]
    fn example_part_2() {
        assert_eq!(count_safe_dampened_reports(EXAMPLE), 4);
    }

    #[test]
    fn part_2() {
        assert_eq!(count_safe_dampened_reports(INPUT), 621);
    }
}
