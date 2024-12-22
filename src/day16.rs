use aoc_runner_derive::aoc;
use grid::{Coord2d, Grid};
use std::{
    cmp::Ordering,
    collections::{BinaryHeap, HashMap},
    str::FromStr,
};

type CoordType = i16;
type Coord = (CoordType, CoordType);

#[derive(Copy, Clone, Eq, PartialEq, Hash, Debug, Ord, PartialOrd)]
enum FacingDirection {
    East,
    South,
    West,
    North,
}

impl FacingDirection {
    fn ofs(&self) -> (CoordType, CoordType) {
        match self {
            FacingDirection::East => (1, 0),
            FacingDirection::South => (0, 1),
            FacingDirection::West => (-1, 0),
            FacingDirection::North => (0, -1),
        }
    }
    fn reachable(&self) -> &[FacingDirection; 3] {
        // Can move perpendicularly or the same direction, backwards would always increase path cost
        match self {
            FacingDirection::East => &[FacingDirection::East, FacingDirection::North, FacingDirection::South],
            FacingDirection::West => &[FacingDirection::West, FacingDirection::North, FacingDirection::South],
            FacingDirection::South => &[FacingDirection::South, FacingDirection::East, FacingDirection::West],
            FacingDirection::North => &[FacingDirection::North, FacingDirection::East, FacingDirection::West],
        }
    }
}

#[derive(Clone, Eq, PartialEq, Debug)]
struct State {
    cost: usize,
    position: Coord,
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
    path: Vec<Coord>,
}

impl Ord for PathState {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.state.cmp(&other.state)
    }
}
impl PartialOrd for PathState {
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
    fn valid_moves<'a>(&'a self, state: &'a State) -> impl Iterator<Item = State> + use<'a> {
        let reachable = state.facing.reachable();
        reachable
            .iter()
            .map(|dir| (dir, (state.position.0 + dir.ofs().0, state.position.1 + dir.ofs().1)))
            .filter(|(_, pos)| self.map.get(pos).is_some_and(|c| *c != b'#'))
            .map(|(dir, pos)| State {
                facing: *dir,
                position: pos,
                cost: if *dir == state.facing {
                    state.cost + 1
                } else {
                    state.cost + 1001
                },
            })
    }
    fn dijkstra(&self) -> usize {
        let Coord2d {x: start_x, y: start_y} = self.map.find(&b'S').expect("can't find start");
        let start = (start_x as CoordType, start_y as CoordType);

        let Coord2d {x: finish_x, y: finish_y} = self.map.find(&b'E').expect("can't find finish");
        let finish = (finish_x as CoordType, finish_y as CoordType);

        let mut distances = HashMap::new();
        let mut queue = BinaryHeap::new();

        distances.insert((start, FacingDirection::East), 0);
        queue.push(State {
            cost: 0,
            position: start,
            facing: FacingDirection::East,
        });

        while let Some(state) = queue.pop() {
            if state.position == finish {
                return state.cost;
            }

            if distances
                .get(&(state.position, state.facing))
                .is_some_and(|v| state.cost > *v)
            {
                continue;
            }

            for new_state in self.valid_moves(&state) {
                if distances
                    .get(&(new_state.position, new_state.facing))
                    .is_none_or(|best_cost| new_state.cost < *best_cost)
                {
                    distances.insert((new_state.position, new_state.facing), new_state.cost);
                    queue.push(new_state);
                }
            }
        }
        usize::MAX
    }
    fn path_dijkstra(&mut self) -> (usize, Vec<Vec<Coord>>) {
        let Coord2d {x: start_x, y: start_y} = self.map.find(&b'S').expect("can't find start");
        let start = (start_x as CoordType, start_y as CoordType);

        let Coord2d {x: finish_x, y: finish_y} = self.map.find(&b'E').expect("can't find finish");
        let finish = (finish_x as CoordType, finish_y as CoordType);

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
            path: Vec::new(),
        });

        while let Some(PathState { state, mut path }) = queue.pop() {
            if distances
                .get(&(state.position, state.facing))
                .is_some_and(|v| state.cost > *v)
            {
                continue;
            }
            if state.position == finish {
                match state.cost.cmp(&best_cost) {
                    Ordering::Less => {
                        path.push(state.position);
                        best_paths.clear();
                        best_paths.push(path);
                        best_cost = state.cost
                    }
                    Ordering::Equal => {
                        path.push(state.position);
                        best_paths.push(path);
                    }
                    _ => {}
                }
                continue;
            }

            for new_state in self.valid_moves(&state) {
                if distances
                    .get(&(new_state.position, new_state.facing))
                    .is_none_or(|best_cost| new_state.cost <= *best_cost)
                {
                    let mut new_path = path.clone();
                    new_path.push(state.position);
                    distances.insert((new_state.position, new_state.facing), new_state.cost);
                    queue.push(PathState {
                        state: new_state,
                        path: new_path,
                    });
                }
            }
        }
        (best_cost, best_paths)
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

    let mut path_map = maze.map.same_shape(false);
    for tile in best_paths.1.iter().flatten() {
        path_map.set(tile, true);
    }
    path_map.count(&true)
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
