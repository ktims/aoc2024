use aoc_runner_derive::aoc;
use grid::{AsCoord2d, Coord2d, Grid};
use itertools::Itertools;
use std::{fmt::Display, io::Cursor, str::FromStr};

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
    fn step_robot(&mut self, dir: Move) {
        let start = self.robot_pos;
        if self.push(&start, &dir) {
            self.robot_pos = &self.robot_pos + dir.ofs();
        }
    }

    fn push(&mut self, pos: &Coord2d, dir: &Move) -> bool {
        if self.can_push(pos, dir) {
            let target = pos + dir.ofs();
            match self.map.get(&target).unwrap() {
                b'#' => {}
                b'.' => self.map.swap(target, pos),
                b'O' => {
                    self.push(&target, dir);
                    self.map.swap(target, pos);
                }
                b'[' | b']' if *dir == Move::Left || *dir == Move::Right => {
                    self.push(&target, dir);
                    self.map.swap(target, pos)
                }
                b']' => {
                    // move both parts
                    self.push(&target, dir);
                    self.push(&(&target + (-1, 0)), dir);
                    self.map.swap(target, pos);
                }
                b'[' => {
                    self.push(&target, dir);
                    self.push(&(&target + (1, 0)), dir);
                    self.map.swap(target, pos);
                }
                c => panic!("unexpected char {}", c),
            }
            return true;
        }
        false
    }

    fn can_push(&mut self, pos: &Coord2d, dir: &Move) -> bool {
        let target = pos + dir.ofs();
        match self.map.get(&target).unwrap() {
            b'#' => false,
            b'.' => true,
            b'O' => self.can_push(&target, dir),
            b'[' | b']' if *dir == Move::Left || *dir == Move::Right => self.can_push(&target, dir),
            b']' => self.can_push(&target, dir) && self.can_push(&(&target + (-1, 0)), dir),
            b'[' => self.can_push(&target, dir) && self.can_push(&(&target + (1, 0)), dir),
            c => panic!("unexpected char {}", c),
        }
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

    fn score(&self) -> i64 {
        self.map
            .data
            .iter()
            .enumerate()
            .filter(|(_, v)| **v == b'O' || **v == b'[')
            .map(|(i, _)| self.map.coord(i as i64).unwrap().y() * 100 + self.map.coord(i as i64).unwrap().x())
            .sum()
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
        Ok(MovePlan(s.chars().filter(|c| *c != '\n').map(Move::from).collect()))
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
    for m in moves.0 {
        wh.step_robot(m);
    }
    wh.score()
}

#[aoc(day15, part2)]
pub fn part2(input: &str) -> i64 {
    let (mut wh, moves) = parse(input);
    wh.embiggen();

    for m in moves.0 {
        wh.step_robot(m);
    }
    wh.score()
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
