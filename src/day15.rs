use std::{
    cmp,
    fmt::Display,
    io::{BufRead, Cursor, Lines},
    iter,
    str::FromStr,
};

use aoc_runner_derive::aoc;
use grid::{AsCoord2d, Coord2d, Grid};
use itertools::{rev, Itertools};

struct Warehouse {
    map: Grid<u8>,
    robot_pos: Coord2d,
}

impl Display for Warehouse {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        self.map.fmt(f)
    }
}

impl Warehouse {
    fn step_robot(&mut self, m: Move) {
        match m {
            Move::Left => {
                let to_left = &self.map.row(self.robot_pos.y()).unwrap()[0..self.robot_pos.x() as usize];
                let left_chunks = to_left
                    .chunk_by(|a, b| a == b || (*a == b'[' && *b == b']'))
                    .collect_vec();
                match left_chunks.last().unwrap().last().unwrap() {
                    b'.' => {
                        self.map
                            .swap(&self.robot_pos, (self.robot_pos.x() - 1, self.robot_pos.y()));
                        self.robot_pos.x -= 1
                    }
                    b'O' | b'[' | b']' => {
                        if left_chunks[left_chunks.len() - 2].last().unwrap() == &b'.' {
                            let y = self.robot_pos.y();
                            // swap the whole chunk left
                            for x_target in self.robot_pos.x() - left_chunks.last().unwrap().len() as i64
                                ..=self.robot_pos.x() as i64
                            {
                                self.map.swap((x_target, y), (x_target - 1, y));
                            }
                            self.robot_pos.x -= 1;
                        }
                    }
                    b'#' => {}
                    c => panic!("unexpected char {}", c),
                }
            }
            Move::Right => {
                let to_right =
                    &self.map.row(self.robot_pos.y()).unwrap()[self.robot_pos.x() as usize + 1..self.map.width()];
                let right_chunks = to_right
                    .chunk_by(|a, b| a == b || (*a == b'[' && *b == b']'))
                    .collect_vec();
                match right_chunks[0][0] {
                    b'.' => {
                        self.map
                            .swap(&self.robot_pos, (self.robot_pos.x() + 1, self.robot_pos.y()));
                        self.robot_pos.x += 1
                    }
                    b'O' | b'[' | b']' => {
                        if right_chunks[1][0] == b'.' {
                            let y = self.robot_pos.y();
                            // swap the whole chunk right
                            for x_target in
                                (self.robot_pos.x() + 1..=self.robot_pos.x() + 1 + right_chunks[0].len() as i64).rev()
                            {
                                self.map.swap((x_target, y), (x_target - 1, y));
                            }
                            self.robot_pos.x += 1;
                        }
                    }
                    b'#' => {}
                    c => panic!("unexpected char {}", c),
                }
            }
            Move::Up => {
                let to_up = &self.map.col(self.robot_pos.x()).unwrap()[0..self.robot_pos.y() as usize];
                let up_chunks = to_up.chunk_by(|a, b| a == b).collect_vec();
                match up_chunks.last().unwrap().last().unwrap() {
                    b'.' => {
                        self.map
                            .swap(&self.robot_pos, (self.robot_pos.x(), self.robot_pos.y() - 1));
                        self.robot_pos.y -= 1
                    }
                    b'O' => {
                        if **up_chunks[up_chunks.len() - 2].last().unwrap() == b'.' {
                            let x = self.robot_pos.x();
                            // swap the whole chunk left
                            for y_target in
                                self.robot_pos.y() - up_chunks.last().unwrap().len() as i64..=self.robot_pos.y() as i64
                            {
                                self.map.swap((x, y_target), (x, y_target - 1));
                            }
                            self.robot_pos.y -= 1;
                        }
                    }
                    b'#' => {}
                    c => panic!("unexpected char {}", c),
                }
            }
            Move::Down => {
                let to_down =
                    &self.map.col(self.robot_pos.x()).unwrap()[self.robot_pos.y() as usize + 1..self.map.height()];
                let down_chunks = to_down.chunk_by(|a, b| a == b).collect_vec();
                match down_chunks[0][0] {
                    b'.' => {
                        self.map
                            .swap(&self.robot_pos, (self.robot_pos.x(), self.robot_pos.y() + 1));
                        self.robot_pos.y += 1;
                    }
                    b'O' => {
                        if *down_chunks[1][0] == b'.' {
                            let x = self.robot_pos.x();
                            // swap the whole chunk down
                            for y_target in
                                (self.robot_pos.y() + 1..=self.robot_pos.y() + 1 + down_chunks[0].len() as i64).rev()
                            {
                                self.map.swap((x, y_target), (x, y_target - 1));
                            }
                            self.robot_pos.y += 1;
                        }
                    }
                    b'#' => {}
                    c => panic!("unexpected char {}", c),
                }
            }
        }
    }

    fn step_robot_2(&mut self, dir: Move) {
        let start = self.robot_pos.clone();
        if self.push(&start, &dir) {
            self.robot_pos = match dir {
                Move::Left => (self.robot_pos.x() - 1, self.robot_pos.y()).to_coord(),
                Move::Right => (self.robot_pos.x() + 1, self.robot_pos.y()).to_coord(),
                Move::Up => (self.robot_pos.x(), self.robot_pos.y() - 1).to_coord(),
                Move::Down => (self.robot_pos.x(), self.robot_pos.y() + 1).to_coord(),
            }
        }
    }

    fn push(&mut self, pos: &Coord2d, dir: &Move) -> bool {
        if self.can_push(pos, dir) {
            let target = pos + dir.ofs();
            match self.map.get(&target).unwrap() {
                b'#' => {}
                b'.' => self.map.swap(target, pos),
                b'[' | b']' if *dir == Move::Left || *dir == Move::Right => {
                    self.push(&target, dir);
                    self.map.swap(target, pos)
                }
                b']' => {
                    // move both parts
                    self.push(&target, dir);
                    self.push(&(&target + (-1, 0)), dir);
                    self.map.swap(&target, pos);
                }
                b'[' => {
                    self.push(&target, dir);
                    self.push(&(&target + (1, 0)), dir);
                    self.map.swap(&target, pos);
                }
                c => panic!("unexpected char {}", c),
            }
            return true;
        }
        false
    }

    fn can_push(&mut self, pos: &Coord2d, dir: &Move) -> bool {
        let target = pos + dir.ofs();
        return match self.map.get(&target).unwrap() {
            b'#' => false,
            b'.' => true,
            b'O' => self.can_push(&target, dir),
            b'[' | b']' if *dir == Move::Left || *dir == Move::Right => self.can_push(&target, dir),
            b']' => self.can_push(&target, dir) && self.can_push(&(&target + (-1, 0)), dir),
            b'[' => self.can_push(&target, dir) && self.can_push(&(&target + (1, 0)), dir),
            c => panic!("unexpected char {}", c),
        };
    }

    fn embiggen(&mut self) {
        let new_lines = (0..self.map.height())
            .map(|r| self.map.row(r as i64).unwrap())
            .map(|row| {
                row.iter()
                    .flat_map(|c| match c {
                        b'#' => ['#', '#'],
                        b'O' => ['[', ']'],
                        b'.' => ['.', '.'],
                        b'@' => ['@', '.'],
                        c => panic!("unexpected character {}", c),
                    })
                    .collect::<String>()
            })
            .join("\n");
        self.map = Grid::from(Cursor::new(new_lines.as_str()));
        self.robot_pos = self.map.find(&b'@').unwrap().to_coord();
    }
}

#[derive(Debug, Eq, PartialEq)]
enum Move {
    Left,
    Right,
    Up,
    Down,
}

impl Display for Move {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Left => f.write_str("Left"),
            Self::Right => f.write_str("Right"),
            Self::Up => f.write_str("Up"),
            Self::Down => f.write_str("Down"),
        }
    }
}

impl From<char> for Move {
    fn from(c: char) -> Self {
        match c {
            '<' => Self::Left,
            '>' => Self::Right,
            '^' => Self::Up,
            'v' => Self::Down,
            c => panic!("invalid move {}", c),
        }
    }
}

impl Move {
    fn ofs(&self) -> (i64, i64) {
        match self {
            Move::Left => (-1, 0),
            Move::Right => (1, 0),
            Move::Up => (0, -1),
            Move::Down => (0, 1),
        }
    }
}

#[derive(Debug)]
struct MovePlan(Vec<Move>);

impl FromStr for MovePlan {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(MovePlan(
            s.chars().filter(|c| *c != '\n').map(|c| Move::from(c)).collect(),
        ))
    }
}

fn parse(input: &str) -> (Warehouse, MovePlan) {
    let lines = input.lines().collect_vec();
    let parts = lines.split(|l| l.is_empty()).map(|ls| ls.join("\n")).collect_vec();
    let map: Grid<u8> = parts[0].parse().unwrap();
    let wh = Warehouse {
        robot_pos: map.find(&b'@').unwrap().to_coord(),
        map,
    };
    let moves = parts[1].parse().unwrap();

    (wh, moves)
}

#[aoc(day15, part1)]
pub fn part1(input: &str) -> i64 {
    let (mut wh, moves) = parse(input);
    // println!("map:\n {}\nmoves: {:?}", wh, moves);
    for m in moves.0 {
        // println!("{}", m);
        wh.step_robot(m);
        // println!("{}", wh);
    }
    wh.map
        .data
        .iter()
        .enumerate()
        .filter(|(i, v)| **v == b'O')
        .map(|(i, _)| wh.map.coord(i as i64).unwrap().y() * 100 + wh.map.coord(i as i64).unwrap().x())
        .sum()
}

#[aoc(day15, part2)]
pub fn part2(input: &str) -> i64 {
    let (mut wh, moves) = parse(input);
    wh.embiggen();

    println!("{}", wh);
    for m in moves.0 {
        // println!("{}", m);
        wh.step_robot_2(m);
        // println!("{}", wh);
    }
    println!("{}", wh);
    let mut sum = 0;
    wh.map
        .data
        .iter()
        .enumerate()
        .filter(|(i, v)| **v == b'[')
        .map(|(i, _)| wh.map.coord(i as i64).unwrap().y() * 100 + wh.map.coord(i as i64).unwrap().x())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE1: &str = "########
#..O.O.#
##@.O..#
#...O..#
#.#.O..#
#...O..#
#......#
########

<^^>>>vv<v>>v<<";
    const EXAMPLE2: &str = "##########
#..O..O.O#
#......O.#
#.OO..O.O#
#..O@..O.#
#O#..O...#
#O..O..O.#
#.OO.O.OO#
#....O...#
##########

<vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^
vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v
><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<
<<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^
^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><
^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^
>^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^
<><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>
^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>
v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^";

    const EXAMPLE3: &str = "#######
#...#.#
#.....#
#..OO@#
#..O..#
#.....#
#######

<vv<<^^<<^^";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE1), 2028);
        assert_eq!(part1(EXAMPLE2), 10092);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE3), 618);
        assert_eq!(part2(EXAMPLE2), 9021);
    }
}
