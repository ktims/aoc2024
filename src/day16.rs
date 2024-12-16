use aoc_runner_derive::{aoc, aoc_generator};
use grid::{AsCoord2d, Coord2d, Grid};
use std::{
    collections::{BinaryHeap, HashMap},
    str::FromStr,
};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
enum FacingDirection {
    East,
    South,
    West,
    North,
}

impl FacingDirection {
    fn ofs(&self) -> (i64, i64) {
        match self {
            FacingDirection::East => (1, 0),
            FacingDirection::South => (0, 1),
            FacingDirection::West => (-1, 0),
            FacingDirection::North => (0, -1),
        }
    }
    fn reachable(&self) -> [FacingDirection; 3] {
        // Can move perpendicularly or the same direction, not backwards
        match self {
            FacingDirection::East | FacingDirection::West => [*self, FacingDirection::North, FacingDirection::South],
            FacingDirection::South | FacingDirection::North => [*self, FacingDirection::East, FacingDirection::West],
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct State {
    cost: usize,
    position: Coord2d,
    facing: FacingDirection,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
            .then_with(|| self.facing.cmp(&other.facing))
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

struct Maze {
    map: Grid<u8>,
}

impl FromStr for Maze {
    type Err = Box<dyn std::error::Error>;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let map: Grid<u8> = s.parse()?;

        Ok(Self { map })
    }
}

impl Maze {
    fn dijkstra(&mut self) -> usize {
        let start = self.map.find(&b'S').expect("can't find start").to_coord();
        let finish = self.map.find(&b'E').expect("can't find finish").to_coord();

        let mut distances = HashMap::new();
        let mut queue = BinaryHeap::with_capacity(self.map.data.len());

        distances.insert((start, FacingDirection::East), 0);
        queue.push(State {
            cost: 0,
            position: start,
            facing: FacingDirection::East,
        });

        while let Some(State { cost, position, facing }) = queue.pop() {
            if position == finish {
                return cost;
            }

            if distances.get(&(position, facing)).is_some_and(|v| cost > *v) {
                continue;
            }

            for (new_dir, new_position, new_cost) in facing
                .reachable()
                .iter()
                .map(|dir| (dir, &position + dir.ofs()))
                .filter(|(_, pos)| self.map.get(pos).is_some_and(|c| *c != b'#'))
                .map(|(dir, pos)| (dir, pos, if *dir == facing { cost + 1 } else { cost + 1001 }))
            {
                if distances
                    .get(&(new_position, *new_dir))
                    .is_none_or(|best_cost| new_cost < *best_cost)
                {
                    queue.push(State {
                        cost: new_cost,
                        position: new_position,
                        facing: *new_dir,
                    });
                    distances.insert((new_position, *new_dir), new_cost);
                }
            }
        }
        panic!("no path found");
    }
}

fn parse(input: &str) -> Maze {
    input.parse().unwrap()
}

#[aoc(day16, part1)]
pub fn part1(input: &str) -> usize {
    let mut maze = parse(input);
    maze.dijkstra()
}

#[aoc(day16, part2)]
pub fn part2(input: &str) -> i64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE1: &str = "###############
#.......#....E#
#.#.###.#.###.#
#.....#.#...#.#
#.###.#####.#.#
#.#.#.......#.#
#.#.#####.###.#
#...........#.#
###.#.#####.#.#
#...#.....#.#.#
#.#.#.###.#.#.#
#.....#...#.#.#
#.###.#.#.#.#.#
#S..#.....#...#
###############";

    const EXAMPLE2: &str = "#################
#...#...#...#..E#
#.#.#.#.#.#.#.#.#
#.#.#.#...#...#.#
#.#.#.#.###.#.#.#
#...#.#.#.....#.#
#.#.#.#.#.#####.#
#.#...#.#.#.....#
#.#.#####.#.###.#
#.#.#.......#...#
#.#.###.#####.###
#.#.#...#.....#.#
#.#.#.#####.###.#
#.#.#.........#.#
#.#.#.#########.#
#S#.............#
#################";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE1), 7036);
        assert_eq!(part1(EXAMPLE2), 11048);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE1), 0);
        assert_eq!(part2(EXAMPLE2), 0);
    }
}
