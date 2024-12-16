use aoc_runner_derive::aoc;
use grid::{AsCoord2d, Coord2d, Grid};
use std::{
    collections::{BinaryHeap, HashMap},
    str::FromStr,
    usize,
};

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
enum FacingDirection {
    East,
    South,
    West,
    North,
}

impl FacingDirection {
    fn ofs(&self) -> (i32, i32) {
        match self {
            FacingDirection::East => (1, 0),
            FacingDirection::South => (0, 1),
            FacingDirection::West => (-1, 0),
            FacingDirection::North => (0, -1),
        }
    }
    fn reachable(&self) -> [FacingDirection; 3] {
        // Can move perpendicularly or the same direction, backwards would always increase path cost
        match self {
            FacingDirection::East | FacingDirection::West => [*self, FacingDirection::North, FacingDirection::South],
            FacingDirection::South | FacingDirection::North => [*self, FacingDirection::East, FacingDirection::West],
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct State {
    cost: usize,
    position: (i32, i32),
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

#[derive(Clone, Eq, PartialEq, Debug)]
struct PathState {
    state: State,
    path: Vec<(i32, i32)>,
}

impl Ord for PathState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.state.cmp(&other.state)
    }
}
impl PartialOrd for PathState {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        self.state.partial_cmp(&other.state)
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
    fn dijkstra(&self) -> usize {
        let (start_x, start_y) = self.map.find(&b'S').expect("can't find start");
        let start = (start_x as i32, start_y as i32);

        let (finish_x, finish_y) = self.map.find(&b'E').expect("can't find finish");
        let finish = (finish_x as i32, finish_y as i32);

        let mut distances = HashMap::new();
        let mut queue = BinaryHeap::new();

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
                .map(|dir| (dir, (position.0 + dir.ofs().0, position.1 + dir.ofs().1)))
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
        usize::MAX
    }
    fn path_dijkstra(&mut self) -> (usize, Vec<Vec<(i32, i32)>>) {
        let (start_x, start_y) = self.map.find(&b'S').expect("can't find start");
        let start = (start_x.try_into().unwrap(), start_y.try_into().unwrap());
        let (finish_x, finish_y) = self.map.find(&b'E').expect("can't find finish");
        let finish = (finish_x.try_into().unwrap(), finish_y.try_into().unwrap());

        let mut distances = HashMap::new();
        let mut queue = BinaryHeap::with_capacity(self.map.data.len());
        let mut best_paths = Vec::new();
        let mut best_cost = usize::MAX;

        distances.insert((start, FacingDirection::East), 0);
        queue.push(PathState {
            state: State {
                cost: 0,
                position: start,
                facing: FacingDirection::East,
            },
            path: Vec::with_capacity(100),
        });

        while let Some(PathState { state, path }) = queue.pop() {
            let mut new_path = path.clone();
            new_path.push(state.position);

            if state.position == finish {
                if state.cost < best_cost {
                    best_paths.clear();
                    best_paths.push(new_path);
                    best_cost = state.cost
                } else if state.cost == best_cost {
                    best_paths.push(new_path);
                }
                continue;
            }

            if distances
                .get(&(state.position, state.facing))
                .is_some_and(|v| state.cost > *v)
            {
                continue;
            }

            for (new_dir, new_position, new_cost) in state
                .facing
                .reachable()
                .iter()
                .map(|dir| (dir, (state.position.0 + dir.ofs().0, state.position.1 + dir.ofs().1)))
                .filter(|(_, pos)| self.map.get(pos).is_some_and(|c| *c != b'#'))
                .map(|(dir, pos)| {
                    (
                        dir,
                        pos,
                        if *dir == state.facing {
                            state.cost + 1
                        } else {
                            state.cost + 1001
                        },
                    )
                })
            {
                if distances
                    .get(&(new_position, *new_dir))
                    .is_none_or(|best_cost| new_cost <= *best_cost)
                {
                    queue.push(PathState {
                        state: State {
                            cost: new_cost,
                            position: new_position,
                            facing: *new_dir,
                        },
                        path: new_path.clone(),
                    });
                    distances.insert((new_position, *new_dir), new_cost);
                }
            }
        }
        return (best_cost, best_paths);
    }
}

fn parse(input: &str) -> Maze {
    input.parse().unwrap()
}

#[aoc(day16, part1)]
pub fn part1(input: &str) -> usize {
    let maze = parse(input);
    maze.dijkstra()
}

#[aoc(day16, part2)]
pub fn part2(input: &str) -> usize {
    let mut maze = parse(input);
    let best_paths = maze.path_dijkstra();

    let mut path_map = maze.map.clone();
    for tile in best_paths.1.into_iter().flatten() {
        path_map.set(&tile, b'O');
    }
    path_map.count(&b'O')
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
    fn part1_example1() {
        assert_eq!(part1(EXAMPLE1), 7036);
    }

    #[test]
    fn part1_example2() {
        assert_eq!(part1(EXAMPLE2), 11048);
    }

    #[test]
    fn part2_example1() {
        assert_eq!(part2(EXAMPLE1), 45);
    }

    #[test]
    fn part2_example2() {
        assert_eq!(part2(EXAMPLE2), 64);
    }
}
