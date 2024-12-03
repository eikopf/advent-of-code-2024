use logos::{Lexer, Logos};

#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub enum LexError {
    Int(std::num::ParseIntError),
    #[default]
    Unknown,
}

impl From<std::num::ParseIntError> for LexError {
    fn from(v: std::num::ParseIntError) -> Self {
        Self::Int(v)
    }
}

#[derive(Debug, Clone, Copy, Logos)]
#[logos(error = LexError)]
pub enum Token {
    #[regex(r#"mul\([0-9]+\,[0-9]+\)"#, process_mul)]
    Mul((usize, usize)),
    #[regex(".")]
    Junk,
}

impl Token {
    pub fn as_mul(self) -> Option<(usize, usize)> {
        if let Self::Mul(v) = self {
            Some(v)
        } else {
            None
        }
    }
}

fn process_mul(lexer: &mut Lexer<Token>) -> Result<(usize, usize), std::num::ParseIntError> {
    let (lhs, tail) = lexer
        .slice()
        .strip_prefix("mul(")
        .unwrap()
        .split_once(",")
        .unwrap();

    let lhs = lhs.parse::<usize>()?;
    let rhs = tail.strip_suffix(")").unwrap().parse::<usize>()?;
    Ok((lhs, rhs))
}

/// Computes the solution to part 1.
pub fn uncorrupted_mul_sum(input: &str) -> usize {
    Token::lexer(input)
        .filter_map(|tok| tok.ok().and_then(Token::as_mul))
        .map(|(lhs, rhs)| lhs * rhs)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"
        xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))
            "#;

    const INPUT: &str = include_str!("../input/day03.txt");

    #[test]
    fn example_part1() {
        assert_eq!(uncorrupted_mul_sum(EXAMPLE), 161);
    }

    #[test]
    fn part1() {
        assert_eq!(uncorrupted_mul_sum(INPUT), 170068701);
    }
}
