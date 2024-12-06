use std::{collections::HashSet, str::FromStr};

use nalgebra as na;
use rayon::iter::{IntoParallelIterator, ParallelIterator as _};

#[derive(Debug, Clone)]
pub struct Area {
    map: na::DMatrix<Position>,
    guard: Guard,
}

impl Area {
    pub fn next_state(&mut self) -> Action {
        let action = self.next_guard_action();
        self.run_action(action);
        action
    }

    pub fn next_guard_index(&self) -> Option<usize> {
        if self.guard_will_leave() {
            return None;
        }

        let index: isize = self.guard.index.try_into().unwrap();

        let offset = match self.guard.direction {
            Direction::N => -1,
            Direction::E => self.map.nrows() as isize,
            Direction::S => 1,
            Direction::W => -(self.map.nrows() as isize),
        };

        usize::try_from(index + offset).ok()
    }

    pub fn next_guard_action(&self) -> Action {
        match self.next_guard_index() {
            None => Action::Leave,
            Some(index) => match self.map[index] {
                Position::Clear => Action::Advance { index },
                Position::Obstructed => Action::Rotate,
            },
        }
    }

    pub fn run_action(&mut self, action: Action) {
        match action {
            Action::Advance { index } => {
                self.guard.index = index;
            }
            Action::Rotate => {
                self.guard.direction = self.guard.direction.turn_right();
            }
            Action::Leave => {
                self.guard.index = usize::MAX;
            }
        }
    }

    pub fn guard_will_leave(&self) -> bool {
        let ncols = self.map.ncols();
        let nrows = self.map.nrows();
        let index = self.guard.index;

        match self.guard.direction {
            Direction::N if index % nrows == 0 => true,
            Direction::E if index / ncols == ncols - 1 => true,
            Direction::S if index % nrows == nrows - 1 => true,
            Direction::W if index / ncols == 0 => true,
            _ => false,
        }
    }
}

impl FromStr for Area {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let ncols = s.find('\n').ok_or(())?;
        let nrows = s.chars().filter(|&c| c != '\n').count() / ncols;

        let map = na::DMatrix::from_row_iterator(
            nrows,
            ncols,
            s.split('\n')
                .flat_map(|s| s.chars().map(|c| Position::try_from(c).unwrap())),
        );

        let guard = {
            // find raw index in the input
            let raw_index = s.find(Guard::is_guard_char).ok_or(())?;
            // adjust for newline characters to get the row-major index
            let row_index = raw_index - (raw_index / nrows) + 1;
            // convert to column major index
            let index = (row_index % ncols) * nrows + (row_index / nrows);

            let direction = s
                .chars()
                .nth(raw_index) // use raw_index to access the source char
                .ok_or(())?
                .try_into()
                .map_err(|_| ())?;

            Guard { index, direction }
        };

        Ok(Area { map, guard })
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Action {
    Advance { index: usize },
    Rotate,
    Leave,
}

impl Action {
    pub fn is_leave(&self) -> bool {
        matches!(self, Self::Leave)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Guard {
    index: usize,
    direction: Direction,
}

impl Guard {
    pub fn is_guard_char(c: char) -> bool {
        matches!(c, '^' | '>' | 'V' | '<')
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq)]
pub enum Position {
    #[default]
    Clear,
    Obstructed,
}

impl Position {
    pub fn is_obstructed(&self) -> bool {
        matches!(self, Self::Obstructed)
    }
}

impl TryFrom<char> for Position {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Position::Obstructed),
            '.' | '^' | '>' | 'V' | '<' => Ok(Position::Clear),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    N,
    E,
    S,
    W,
}

impl TryFrom<char> for Direction {
    type Error = ();

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '^' => Ok(Direction::N),
            '>' => Ok(Direction::E),
            'V' => Ok(Direction::S),
            '<' => Ok(Direction::W),
            _ => Err(()),
        }
    }
}

impl Direction {
    pub fn turn_right(self) -> Self {
        match self {
            Direction::N => Direction::E,
            Direction::E => Direction::S,
            Direction::S => Direction::W,
            Direction::W => Direction::N,
        }
    }
}

/// Computes the solution to part 1.
pub fn count_distinct_patrol_positions(input: &str) -> usize {
    let mut area = input.parse::<Area>().unwrap();
    let mut positions = HashSet::new();

    loop {
        positions.insert(area.guard.index);

        if area.next_state().is_leave() {
            break;
        }
    }

    positions.len()
}

/// Computes the solution to part 2.
pub fn count_possible_loops(input: &str) -> usize {
    // brute force because i kinda hate this problem

    // roughly the lowest fuel value that produces a valid answer
    const FUEL: usize = 6000;
    let area = input.parse::<Area>().unwrap();

    // obstructions have to be placed on the guard's path, so we grab them first
    // to reduce the number of permutations that actually need to be checked
    let positions = {
        let mut set = HashSet::new();
        let mut area = area.clone();

        loop {
            set.insert(area.guard.index);

            if area.next_state().is_leave() {
                break;
            }
        }

        set
    };

    // rayon drops the processing time in the full input case from ~5s to 0.16s
    // on my 2021 macbook pro
    positions
        .into_par_iter()
        .map_with(area, |area, i| {
            let mut area = area.clone();
            area.map[i] = Position::Obstructed;

            let mut not_a_loop = false;
            for _ in 0..FUEL {
                if area.next_state().is_leave() {
                    not_a_loop = true;
                    break;
                }
            }

            !not_a_loop
        })
        .filter(|&x| x)
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = r#"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#..."#;

    const INPUT: &str = include_str!("../input/day06.txt");

    #[test]
    fn example_part_1() {
        assert_eq!(count_distinct_patrol_positions(EXAMPLE), 41);
    }

    #[test]
    fn part_1() {
        assert_eq!(count_distinct_patrol_positions(INPUT), 5030);
    }

    #[test]
    fn example_part_2() {
        assert_eq!(count_possible_loops(EXAMPLE), 6);
    }

    #[test]
    fn part_2() {
        assert_eq!(count_possible_loops(INPUT), 1928);
    }
}
