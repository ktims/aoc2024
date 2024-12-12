use aoc_runner_derive::{aoc, aoc_generator};
use bitflags::bitflags;
use rayon::iter::ParallelIterator;
use rayon::slice::ParallelSlice;
use std::fmt;
use std::io::{BufRead, Lines};
use std::ops::BitAnd;

use grid::Grid;

#[aoc_generator(day6)]
pub fn get_input(input: &[u8]) -> Map {
    Map::from(input.lines())
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum FacingDirection {
    Up = 1,
    Down = 2,
    Left = 4,
    Right = 8,
}

impl FacingDirection {
    fn next(&self) -> FacingDirection {
        match self {
            FacingDirection::Up => FacingDirection::Right,
            FacingDirection::Down => FacingDirection::Left,
            FacingDirection::Left => FacingDirection::Up,
            FacingDirection::Right => FacingDirection::Down,
        }
    }
    fn pos_ofs(&self, pos: (i64, i64)) -> (i64, i64) {
        match self {
            FacingDirection::Up => (pos.0, pos.1 + -1),
            FacingDirection::Down => (pos.0, pos.1 + 1),
            FacingDirection::Left => (pos.0 + -1, pos.1),
            FacingDirection::Right => (pos.0 + 1, pos.1),
        }
    }
}

enum StepOutcome {
    LeftMap,
    LoopFound,
    Continue,
}

#[derive(Eq, PartialEq)]
enum RunOutcome {
    LeftMap,
    LoopFound,
}

bitflags! {
    #[derive(Copy, Clone, Eq, PartialEq, Debug)]
    pub struct DirectionSet: u8 {
        const Up = 1;
        const Down = 2;
        const Left = 4;
        const Right = 8;
    }
}

impl From<FacingDirection> for DirectionSet {
    fn from(value: FacingDirection) -> Self {
        match value {
            FacingDirection::Up => DirectionSet::Up,
            FacingDirection::Down => DirectionSet::Down,
            FacingDirection::Left => DirectionSet::Left,
            FacingDirection::Right => DirectionSet::Right,
        }
    }
}

impl fmt::Display for DirectionSet {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        self.0.fmt(f)
    }
}

impl BitAnd<FacingDirection> for DirectionSet {
    type Output = DirectionSet;
    fn bitand(self, rhs: FacingDirection) -> Self::Output {
        self & DirectionSet::from(rhs)
    }
}

#[derive(Clone)]
pub struct Map {
    grid: Grid<u8>,
    visited_from: Grid<DirectionSet>,
    guard_facing: FacingDirection,
    guard_pos: (i64, i64),
    path: Vec<((i64, i64), FacingDirection)>,
}

impl<T: BufRead> From<Lines<T>> for Map {
    fn from(input: Lines<T>) -> Self {
        let grid = Grid::from(input);
        let mut visited_from: Grid<DirectionSet> = Grid::new(grid.width() as i64);
        visited_from.data.resize(grid.data.len(), DirectionSet::empty());
        let guard_pos = grid.find(&b'^').expect("Guard not found");
        let guard_facing = FacingDirection::Up;
        Self {
            grid,
            guard_pos,
            guard_facing,
            visited_from,
            path: Vec::new(),
        }
    }
}

impl Map {
    fn look(&self, dir: &FacingDirection) -> Option<u8> {
        self.grid.get(&dir.pos_ofs(self.guard_pos))
    }
    /// Move one step in the facing direction, return if we are still inside the bounds
    fn step_guard<const RECORD_PATH: bool>(&mut self) -> StepOutcome {
        let new_pos = self.guard_facing.pos_ofs(self.guard_pos);
        if self
            .visited_from
            .get(&new_pos)
            .is_some_and(|dirs| dirs.contains(self.guard_facing.into()))
        {
            StepOutcome::LoopFound
        } else if self.grid.set(&new_pos, b'X').is_some() {
            if RECORD_PATH {
                self.path.push((new_pos, self.guard_facing));
            }
            self.visited_from.set(
                &new_pos,
                self.visited_from.get(&new_pos).unwrap() | self.guard_facing.into(),
            );
            self.guard_pos = new_pos;
            StepOutcome::Continue
        } else {
            StepOutcome::LeftMap
        }
    }
    fn run_guard<const RECORD_PATH: bool>(&mut self) -> RunOutcome {
        while let Some(val) = self.look(&self.guard_facing) {
            match val {
                b'#' => {
                    // obstacle, turn right
                    self.guard_facing = self.guard_facing.next();
                }
                _ => match self.step_guard::<RECORD_PATH>() {
                    StepOutcome::LeftMap => return RunOutcome::LeftMap,
                    StepOutcome::LoopFound => return RunOutcome::LoopFound,
                    StepOutcome::Continue => {}
                },
            }
        }
        RunOutcome::LeftMap
    }
}

// PROBLEM 1 solution
#[aoc(day6, part1)]
pub fn part1(map: &Map) -> u64 {
    let mut map = map.clone();
    map.run_guard::<false>();

    map.grid.count(&b'X') as u64 + 1 // 'X' path positions + 1 starting position
}

// PROBLEM 2 solution
#[aoc(day6, part2)]
pub fn part2(input_map: &Map) -> u64 {
    // Use the solution from problem 1 to reduce the number of positions where obstacle placement will change the path
    let mut path_map = input_map.clone();
    path_map.run_guard::<true>();

    path_map
        .path
        .par_windows(2)
        .filter(|prev_cur| {
            let last_posdir = prev_cur[0];
            let mut test_map = input_map.clone();
            test_map.grid.set(&prev_cur[1].0, b'#').unwrap();
            test_map.guard_pos = last_posdir.0;
            test_map.guard_facing = last_posdir.1;

            test_map.run_guard::<false>() == RunOutcome::LoopFound
        })
        .count() as u64
}

#[cfg(test)]
mod tests {
    use crate::day6::*;

    const EXAMPLE: &[u8] = b"....#.....
.........#
..........
..#.......
.......#..
..........
.#..^.....
........#.
#.........
......#...";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&get_input(EXAMPLE)), 41);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&get_input(EXAMPLE)), 6);
    }
}
