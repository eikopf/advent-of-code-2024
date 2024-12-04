use std::str::FromStr;

use nalgebra as na;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u8)]
pub enum Xmas {
    X,
    M,
    A,
    S,
}

impl TryFrom<char> for Xmas {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'X' => Ok(Self::X),
            'M' => Ok(Self::M),
            'A' => Ok(Self::A),
            'S' => Ok(Self::S),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
pub struct XmasGrid {
    grid: na::DMatrix<Xmas>,
}

impl FromStr for XmasGrid {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let data = s
            .split_whitespace()
            .flat_map(str::chars)
            .map(|c| Xmas::try_from(c).unwrap())
            .collect::<Vec<_>>();

        let nrows = s.find('\n').unwrap();
        let ncols = data.len() / nrows;

        // we initialize the matrix in column-major order, so we then need
        // to transpose it to get the data correctly formed
        let mut grid = na::DMatrix::from_vec(ncols, nrows, data);
        grid.transpose_mut();

        Ok(Self { grid })
    }
}

impl XmasGrid {
    /// Returns an iterator over the column-major indices for all occurrences of `token` in `self`.
    pub fn iter_positions_of(&self, token: Xmas) -> impl Iterator<Item = usize> + use<'_> {
        self.grid
            .iter()
            .enumerate()
            .filter_map(move |(n, elem)| Some(n).filter(|_| *elem == token))
    }

    pub fn count_xmas_sequences_at_index(&self, index: usize) -> usize {
        let nrows: isize = self.grid.nrows().try_into().unwrap();

        let offsets = [
            [-1, -2, -3],                                 // N
            [nrows - 1, 2 * nrows - 2, 3 * nrows - 3],    // NE
            [nrows, 2 * nrows, 3 * nrows],                // E
            [nrows + 1, 2 * nrows + 2, 3 * nrows + 3],    // SE
            [1, 2, 3],                                    // S
            [-nrows + 1, -2 * nrows + 2, -3 * nrows + 3], // SW
            [-nrows, -2 * nrows, -3 * nrows],             // W
            [-nrows - 1, -2 * nrows - 2, -3 * nrows - 3], // NW
        ];

        let mut total = 0;

        let x: isize = index.try_into().unwrap();
        for [m, a, s] in offsets {
            let Ok(m): Result<usize, _> = (x + m).try_into() else {
                continue;
            };
            let Ok(a): Result<usize, _> = (x + a).try_into() else {
                continue;
            };
            let Ok(s): Result<usize, _> = (x + s).try_into() else {
                continue;
            };

            // check that the distances between the sequence elements are correct
            if self.chebyshev(index, m) > 1 || self.chebyshev(m, a) > 1 || self.chebyshev(a, s) > 1
            {
                continue;
            }

            if self.grid.get(m).is_some_and(|t| t == &Xmas::M)
                && self.grid.get(a).is_some_and(|t| t == &Xmas::A)
                && self.grid.get(s).is_some_and(|t| t == &Xmas::S)
            {
                total += 1;
            }
        }

        total
    }

    /// Computes the Chebyshev distance between `a` and `b` on `self`.
    pub fn chebyshev(&self, a: usize, b: usize) -> usize {
        let (row_a, col_a) = self.index_to_position(a);
        let (row_b, col_b) = self.index_to_position(b);

        usize::max(row_a.abs_diff(row_b), col_a.abs_diff(col_b))
    }

    #[inline(always)]
    fn index_to_position(&self, index: usize) -> (usize, usize) {
        let ncols = self.grid.ncols();
        (index % ncols, index / ncols)
    }
}

/// Computes the solution to part 1.
pub fn count_xmas_occurrences(input: &str) -> usize {
    let grid = input.parse::<XmasGrid>().unwrap();
    grid.iter_positions_of(Xmas::X)
        .map(|index| grid.count_xmas_sequences_at_index(index))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"MMMSXXMASM
                             MSAMXMSMSA
                             AMXSXMAAMM
                             MSAMASMSMX
                             XMASAMXAMM
                             XXAMMXXAMA
                             SMSMSASXSS
                             SAXAMASAAA
                             MAMMMXMMMM
                             MXMXAXMASX"#;

    const INPUT: &str = include_str!("../input/day04.txt");

    #[test]
    fn example_part_1() {
        assert_eq!(count_xmas_occurrences(EXAMPLE), 18);
    }

    #[test]
    fn part_1() {
        assert_eq!(count_xmas_occurrences(INPUT), 2514);
    }
}
