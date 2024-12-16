use aoc_runner_derive::aoc;
use colored::Colorize;
use grid::{AsCoord2d, Grid};
use misc::CustomWrapped;
use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map_res, opt, recognize},
    sequence::{preceded, separated_pair},
    IResult,
};
use std::{fmt::Display, str::FromStr};

type Coord = (CustomWrapped<i64>, CustomWrapped<i64>);
struct Robot {
    pos: Coord,
    vel: (i64, i64),
}

struct Robots {
    robots: Vec<Robot>,
    width: i64,
    height: i64,
}

#[derive(Debug, Eq, PartialEq)]
enum Quadrant {
    NW = 0,
    NE = 1,
    SW = 2,
    SE = 3,
}

fn nom_i64(input: &str) -> IResult<&str, i64> {
    let (i, number) = map_res(recognize(preceded(opt(tag("-")), digit1)), |s| i64::from_str(s))(input)?;
    Ok((i, number))
}
fn nom_i64_pair(input: &str) -> IResult<&str, (i64, i64)> {
    let (i, pair) = separated_pair(nom_i64, tag(","), nom_i64)(input)?;
    Ok((i, pair))
}

impl Robot {
    fn from_str(s: &str, bounds: (i64, i64)) -> Self {
        let (s, pos) = preceded(tag("p="), nom_i64_pair)(s).unwrap();
        let (_, vel) = preceded(tag(" v="), nom_i64_pair)(s).unwrap();
        Self {
            pos: (CustomWrapped::new(pos.0, bounds.0), CustomWrapped::new(pos.1, bounds.1)),
            vel,
        }
    }
    fn step(&mut self, count: i64) {
        self.pos.0 += self.vel.x() * count;
        self.pos.1 += self.vel.y() * count;
    }
    fn quad(&self, bounds: (i64, i64)) -> Option<Quadrant> {
        let splits = (bounds.0 / 2, bounds.1 / 2);
        if self.pos.0 < splits.0 && self.pos.1 < splits.1 {
            Some(Quadrant::NW)
        } else if self.pos.0 > splits.0 && self.pos.1 < splits.1 {
            Some(Quadrant::NE)
        } else if self.pos.0 < splits.0 && self.pos.1 > splits.1 {
            Some(Quadrant::SW)
        } else if self.pos.0 > splits.0 && self.pos.1 > splits.1 {
            Some(Quadrant::SE)
        } else {
            None
        }
    }
}

impl Robots {
    fn from_vec(robots: Vec<Robot>, width: i64, height: i64) -> Self {
        Self { robots, width, height }
    }
    fn as_grid(&self) -> Grid<usize> {
        let mut grid = Grid::with_shape(self.width as usize, self.height as usize, 0usize);
        for r in &self.robots {
            grid.increment(&(r.pos.0.val, r.pos.1.val), 1usize);
        }
        grid
    }
    fn count_quads(&self) -> [u64; 4] {
        let mut counts = [0; 4];
        for r in &self.robots {
            if let Some(q) = r.quad((self.width, self.height)) {
                counts[q as usize] += 1
            }
        }
        counts
    }
    fn step(&mut self, count: i64) {
        for robot in &mut self.robots {
            robot.step(count)
        }
    }
}

impl Display for Robots {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let grid = self.as_grid();
        for row in 0..grid.height() {
            for col in 0..grid.width() {
                if *grid.get(&(col, row)).unwrap() != 0 {
                    "█".green().fmt(f)?;
                } else {
                    " ".color(colored::Color::Black).fmt(f)?;
                }
            }
            writeln!(f)?
        }
        Ok(())
    }
}

fn parse(input: &str, width: i64, height: i64) -> Vec<Robot> {
    input.lines().map(|l| Robot::from_str(l, (width, height))).collect()
}

fn part1_impl(input: &str, width: i64, height: i64) -> u64 {
    let mut robots = Robots::from_vec(parse(input, width, height), width, height);
    robots.step(100);
    let counts = robots.count_quads();
    counts.iter().product()
}

#[aoc(day14, part1)]
pub fn part1(input: &str) -> u64 {
    part1_impl(input, 101, 103)
}

#[aoc(day14, part2)]
pub fn part2(input: &str) -> u64 {
    let width = 101;
    let height = 103;
    let mut robots = Robots::from_vec(parse(input, width, height), width, height);
    for i in 1.. {
        robots.step(1);
        // collect into lines
        let g = robots.as_grid();
        if g.data
            .chunk_by(|a, b| *a != 0 && *b != 0)
            .filter(|c| !c.is_empty() && c[0] != 0)
            .any(|c| c.len() > width as usize / 10)
        {
            return i;
        }
    }
    unreachable!()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "p=0,4 v=3,-3
p=6,3 v=-1,-3
p=10,3 v=-1,2
p=2,0 v=2,-1
p=0,0 v=1,3
p=3,0 v=-2,-2
p=7,6 v=-1,-3
p=3,0 v=-1,-2
p=9,3 v=2,3
p=7,3 v=-1,2
p=2,4 v=2,-3
p=9,5 v=-3,-3";

    #[test]
    fn part1_example() {
        assert_eq!(part1_impl(EXAMPLE, 11, 7), 12);
    }

    // part 2 does not converge using the test vector
    // #[test]
    // fn part2_example() {
    //     // assert_eq!(part2(EXAMPLE), 0);
    // }
}
