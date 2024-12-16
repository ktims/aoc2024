use aoc_runner_derive::aoc;
use colored::Colorize;
use grid::{AsCoord2d, Coord2d, Grid};
use regex::Regex;
use std::str::FromStr;

struct Robot {
    pos: Coord2d,
    vel: Coord2d,
}

#[derive(Debug, Eq, PartialEq)]
enum Quadrant {
    NW = 0,
    NE = 1,
    SW = 2,
    SE = 3,
}

impl FromStr for Robot {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let re = Regex::new(r"p=(\d+),(\d+) v=([+-]?\d+),([+-]?\d+)").unwrap();
        match re.captures(s) {
            Some(c) => Ok(Self {
                pos: (
                    c.get(1).unwrap().as_str().parse::<i64>().unwrap(),
                    c.get(2).unwrap().as_str().parse().unwrap(),
                )
                    .to_coord(),
                vel: (
                    c.get(3).unwrap().as_str().parse::<i64>().unwrap(),
                    c.get(4).unwrap().as_str().parse().unwrap(),
                )
                    .to_coord(),
            }),
            None => panic!(),
        }
    }
}

impl Robot {
    fn step(&mut self, bounds: (i64, i64)) {
        let mut candidate_new_pos = ((self.pos.x() + self.vel.x()), (self.pos.y() + self.vel.y()));
        if candidate_new_pos.0 < 0 {
            // if pos goes negative, add the upper bound
            candidate_new_pos.0 += bounds.0;
        }
        if candidate_new_pos.1 < 0 {
            candidate_new_pos.1 += bounds.1;
        }
        candidate_new_pos.0 %= bounds.0;
        candidate_new_pos.1 %= bounds.1;

        self.pos = candidate_new_pos.to_coord();
    }
    fn quad(&self, bounds: (i64, i64)) -> Option<Quadrant> {
        let splits = (bounds.0 / 2, bounds.1 / 2);
        if self.pos.x() < splits.0 && self.pos.y() < splits.1 {
            Some(Quadrant::NW)
        } else if self.pos.x() > splits.0 && self.pos.y() < splits.1 {
            Some(Quadrant::NE)
        } else if self.pos.x() < splits.0 && self.pos.y() > splits.1 {
            Some(Quadrant::SW)
        } else if self.pos.x() > splits.0 && self.pos.y() > splits.1 {
            Some(Quadrant::SE)
        } else {
            None
        }
    }
}

#[allow(dead_code)]
fn display(robots: &Vec<Robot>, bounds: (i64, i64)) {
    let grid = as_grid(robots, bounds);
    for row in 0..grid.height() {
        for col in 0..grid.width() {
            print!(
                "{}",
                if *grid.get(&(col, row)).unwrap() != 0 {
                    "█".green()
                } else {
                    " ".color(colored::Color::Black)
                }
            );
        }
        println!();
    }
}

fn as_grid(robots: &Vec<Robot>, bounds: (i64, i64)) -> Grid<usize> {
    let mut grid = Grid::with_shape(bounds.0 as usize, bounds.1 as usize, 0);
    for r in robots {
        grid.increment(&r.pos, 1usize);
    }
    grid
}

fn parse(input: &str) -> Vec<Robot> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

fn part1_impl(input: &str, width: i64, height: i64) -> u64 {
    let mut robots = parse(input);
    for _ in 0..100 {
        for r in &mut robots {
            r.step((width, height))
        }
    }
    let mut counts = [0; 4];
    for r in robots {
        if let Some(q) = r.quad((width, height)) {
            counts[q as usize] += 1
        }
    }
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
    let mut robots = parse(input);
    for i in 1.. {
        for r in &mut robots {
            r.step((width, height))
        }
        // collect into lines
        let g = as_grid(&robots, (width, height));
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
