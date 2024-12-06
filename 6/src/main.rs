use bitflags::bitflags;
use std::fmt;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::ops::BitAnd;
use std::time::{Duration, Instant};

// BOILERPLATE
type InputIter = Lines<BufReader<File>>;

pub fn get_input() -> InputIter {
    let f = File::open("input").unwrap();
    let br = BufReader::new(f);
    br.lines()
}

fn duration_format(duration: Duration) -> String {
    match duration.as_secs_f64() {
        x if x > 1.0 => format!("{:.3}s", x),
        x if x > 0.010 => format!("{:.3}ms", x * 1e3),
        x => format!("{:.3}us", x * 1e6),
    }
}

fn main() {
    let input = get_input();
    let start = Instant::now();
    let ans1 = problem1(input);
    let duration1 = start.elapsed();
    println!("Problem 1 solution: {} [{}]", ans1, duration_format(duration1));

    let input = get_input();
    let start = Instant::now();
    let ans2 = problem2(input);
    let duration2 = start.elapsed();
    println!("Problem 2 solution: {} [{}]", ans2, duration_format(duration2));
    println!("Total duration: {}", duration_format(duration1 + duration2));
}

#[repr(u8)]
#[derive(Copy, Clone, Debug, Eq, PartialEq, Hash)]
enum FacingDirection {
    Up = b'^',
    Down = b'v',
    Left = b'<',
    Right = b'>',
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

enum RunOutcome {
    LeftMap,
    LoopFound,
    Stuck,
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
struct Map {
    grid: grid::Grid<u8>,
    visited_from: grid::Grid<DirectionSet>,
    guard_facing: FacingDirection,
    guard_pos: (i64, i64),
}

impl<T: BufRead> From<Lines<T>> for Map {
    fn from(input: Lines<T>) -> Self {
        let grid = grid::Grid::from(input);
        let mut visited_from: grid::Grid<DirectionSet> = grid::Grid::new(grid.width() as i64);
        visited_from.data.resize(grid.data.len(), DirectionSet::empty());
        let guard_pos = grid.find(b'^').expect("Guard not found");
        let guard_facing = FacingDirection::Up;
        Self {
            grid,
            guard_pos,
            guard_facing,
            visited_from,
        }
    }
}

impl Map {
    fn look(&self, dir: &FacingDirection) -> Option<u8> {
        match dir {
            FacingDirection::Up => self.grid.get(self.guard_pos.0, self.guard_pos.1 - 1),
            FacingDirection::Down => self.grid.get(self.guard_pos.0, self.guard_pos.1 + 1),
            FacingDirection::Left => self.grid.get(self.guard_pos.0 - 1, self.guard_pos.1),
            FacingDirection::Right => self.grid.get(self.guard_pos.0 + 1, self.guard_pos.1),
        }
    }
    /// Move one step in the facing direction, return if we are still inside the bounds
    fn step_guard(&mut self) -> StepOutcome {
        let new_pos = self.guard_facing.pos_ofs(self.guard_pos);
        if self
            .visited_from
            .get(new_pos.0, new_pos.1)
            .is_some_and(|dirs| dirs.contains(self.guard_facing.into()))
        {
            return StepOutcome::LoopFound;
        }
        if self.grid.set(new_pos.0, new_pos.1, b'X') {
            self.visited_from.set(
                new_pos.0,
                new_pos.1,
                self.visited_from.get(new_pos.0, new_pos.1).unwrap() | self.guard_facing.into(),
            );
            self.guard_pos = new_pos;
            StepOutcome::Continue
        } else {
            StepOutcome::LeftMap
        }
    }
    fn run_guard(&mut self) -> RunOutcome {
        // if the guard is surrounded by obstacles, bail out
        if self
            .grid
            .get(self.guard_pos.0 - 1, self.guard_pos.1)
            .is_some_and(|v| v == b'#')
            && self
                .grid
                .get(self.guard_pos.0 + 1, self.guard_pos.1)
                .is_some_and(|v| v == b'#')
            && self
                .grid
                .get(self.guard_pos.0, self.guard_pos.1 - 1)
                .is_some_and(|v| v == b'#')
            && self
                .grid
                .get(self.guard_pos.0, self.guard_pos.1 + 1)
                .is_some_and(|v| v == b'#')
        {
            return RunOutcome::Stuck;
        }
        while let Some(val) = self.look(&self.guard_facing) {
            match val {
                b'#' => {
                    // obstacle, turn right
                    self.guard_facing = self.guard_facing.next();
                }
                _ => match self.step_guard() {
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

fn problem1<T: BufRead>(input: Lines<T>) -> u64 {
    let mut map = Map::from(input);
    map.run_guard();

    (map.grid.count(b'X') + map.grid.count(b'-') + map.grid.count(b'|') + map.grid.count(b'^')) as u64
}

// PROBLEM 2 solution
fn problem2<T: BufRead>(input: Lines<T>) -> u64 {
    let input_map = Map::from(input);
    // Use the solution from problem 1 to reduce the number of positions where obstacle placement will change the path
    let mut part1 = input_map.clone();
    part1.run_guard();

    let mut loop_count = 0u64;
    for pos in part1
        .grid
        .data
        .iter()
        .enumerate()
        .filter_map(|(pos, c)| if *c == b'X' { Some(pos) } else { None })
    {
        let mut test_map = input_map.clone();
        test_map.grid.data[pos] = b'#';
        if let RunOutcome::LoopFound = test_map.run_guard() {
            loop_count += 1
        }
    }

    loop_count
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::io::Cursor;

    const EXAMPLE: &str = &"....#.....
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
    fn problem1_example() {
        let c = Cursor::new(EXAMPLE);
        assert_eq!(problem1(c.lines()), 41);
    }

    #[test]
    fn problem2_example() {
        let c = Cursor::new(EXAMPLE);
        assert_eq!(problem2(c.lines()), 6);
    }
}
