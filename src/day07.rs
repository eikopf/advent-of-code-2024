const OPERAND_BUFFER_CAPACITY: usize = 16;

#[derive(Debug, Clone, Copy)]
pub struct EqnRef<'a> {
    value: usize,
    operands: &'a [u16],
}

impl<'a> EqnRef<'a> {
    /// Parses the next equation from `s` (if any), using `buf` as a backing buffer
    /// for the `EqnRef` it returns.
    pub fn parse_next<'b: 'a>(s: &mut &str, buf: &'b mut Vec<u16>) -> Option<Self> {
        if s.is_empty() {
            return None;
        }

        let (eqn, tail) = s.split_once('\n').unwrap_or((*s, ""));
        *s = tail;

        let (raw_value, operands) = eqn.split_once(": ").unwrap();
        let value = raw_value.parse::<usize>().unwrap();

        let operands = operands
            .split_whitespace()
            .map(|s| s.parse::<u16>().unwrap());

        buf.clear();
        buf.extend(operands);

        Some(EqnRef {
            value,
            operands: buf,
        })
    }

    pub fn is_solvable(&self) -> bool {
        match self.operands {
            [] => panic!("ran into an equation with no operands"),
            [x] => (*x as usize) == self.value,
            // we recurse from right-to-left because the equations are
            // _evaluated_ from left-to-right; this is effectively undoing
            // the operands one-by-one
            [operands @ .., x] => {
                let x = *x as usize;

                x <= self.value
                    && (EqnRef {
                        value: self.value - x,
                        operands,
                    }
                    .is_solvable()
                        || divides(self.value, x)
                            && EqnRef {
                                value: self.value / x,
                                operands,
                            }
                            .is_solvable())
            }
        }
    }
}

/// Returns `true` iff `rhs` is a factor of `lhs`.
fn divides(lhs: usize, rhs: usize) -> bool {
    let quot = (lhs as f64) / (rhs as f64);
    quot.floor() == quot
}

/// Computes the solution to part 1.
pub fn total_calibration_result(input: &str) -> usize {
    let mut source = input;
    let mut buf = Vec::with_capacity(OPERAND_BUFFER_CAPACITY);

    let mut sum = 0;
    while let Some(eqn) = EqnRef::parse_next(&mut source, &mut buf) {
        if eqn.is_solvable() {
            sum += eqn.value;
        }
    }

    sum
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20"#;

    const INPUT: &str = include_str!("../input/day07.txt");

    #[test]
    fn example_part_1() {
        assert_eq!(total_calibration_result(EXAMPLE), 3749);
    }

    #[test]
    fn part_1() {
        assert_eq!(total_calibration_result(INPUT), 538191549061);
    }
}
