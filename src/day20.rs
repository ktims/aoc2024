use aoc_runner_derive::aoc;
use grid::{AsCoord2d, Coord2d, Grid};
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::collections::VecDeque;

struct RaceTrack {
    map: Grid<u8>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct State {
    pos: Coord2d,
    cost: u64,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct CheatState {
    s: State,
}

const DIRECTIONS: [(i64, i64); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

impl RaceTrack {
    fn valid_moves<'a>(&'a self, CheatState { s: state }: &'a CheatState) -> impl Iterator<Item = CheatState> + 'a {
        DIRECTIONS
            .iter()
            .map(|dir| state.pos + dir)
            .filter_map(move |pos| match &self.map.get(&pos) {
                Some(b'.') | Some(b'S') | Some(b'E') => Some(CheatState {
                    s: State {
                        pos,
                        cost: state.cost + 1,
                    },
                }),
                _ => None,
            })
    }
    fn path_costs(&self, start: Coord2d, goal: Coord2d) -> Grid<Option<u64>> {
        let mut queue = VecDeque::new();
        let mut visited = self.map.same_shape(None);

        let start_state = CheatState {
            s: State { pos: start, cost: 0 },
        };
        visited.set(&start, Some(0));
        queue.push_back(start_state);

        while let Some(state) = queue.pop_front() {
            if state.s.pos == goal {
                return visited;
            }

            let moves = self.valid_moves(&state);
            for new_state in moves {
                if visited.get(&new_state.s.pos).unwrap().is_some() {
                    continue;
                }
                visited.set(&new_state.s.pos, Some(new_state.s.cost));
                queue.push_back(new_state);
            }
        }
        panic!("no path");
    }

    fn find_cheats(&self, path: &Vec<Coord2d>, costs: &Grid<Option<u64>>, min: u64) -> i64 {
        let mut n = 0;
        for pos in path {
            let local_cost = costs.get(pos).unwrap().unwrap();
            for ofs in DIRECTIONS {
                let cheat_exit = (pos.x() + ofs.0 * 2, pos.y() + ofs.1 * 2);
                if let Some(Some(cheat_cost)) = costs.get(&cheat_exit) {
                    if *cheat_cost > local_cost + 2 {
                        let cheat_savings = cheat_cost - local_cost - 2;
                        if cheat_savings >= min {
                            n += 1;
                        }
                    }
                }
            }
        }
        n
    }

    fn taxi_dist<A: AsCoord2d, B: AsCoord2d>(from: &A, to: &B) -> u64 {
        from.x().abs_diff(to.x()) + from.y().abs_diff(to.y())
    }

    fn find_cheats_n(&self, path: &Vec<Coord2d>, costs: &Grid<Option<u64>>, max_length: u64, min: u64) -> i64 {
        path.par_iter()
            .map_with(costs, |costs, pos| {
                let from_cost = costs.get(pos).unwrap().unwrap();
                let mut n = 0;
                for x in pos.x - max_length as i64 - 1..=pos.x + max_length as i64 {
                    for y in pos.y - max_length as i64 - 1..=pos.y + max_length as i64 {
                        let dist = Self::taxi_dist(pos, &(x, y));
                        if dist <= max_length && dist >= 2 {
                            if let Some(Some(to_cost)) = costs.get(&(x, y)) {
                                if *to_cost > (from_cost + dist) && (to_cost - (from_cost + dist) >= min) {
                                    n += 1;
                                }
                            }
                        }
                    }
                }
                n
            })
            .sum()
    }
}

fn parse(input: &str) -> RaceTrack {
    let map = input.as_bytes().into();
    RaceTrack { map }
}

fn part1_impl(input: &str, cheat_min: u64) -> i64 {
    let track = parse(input);
    let start = track.map.find(&b'S').unwrap();
    let goal = track.map.find(&b'E').unwrap();
    let costs = track.path_costs(start, goal);
    let path_squares = costs
        .data
        .iter()
        .enumerate()
        .filter(|(_i, c)| c.is_some())
        .filter_map(|(i, _)| track.map.coord(i as i64))
        .collect_vec();
    track.find_cheats(&path_squares, &costs, cheat_min)
}

fn part2_impl(input: &str, max_length: u64, cheat_min: u64) -> i64 {
    let track = parse(input);
    let start = track.map.find(&b'S').unwrap();
    let goal = track.map.find(&b'E').unwrap();
    let costs = track.path_costs(start, goal);
    let path_squares = costs
        .data
        .iter()
        .enumerate()
        .filter(|(_i, c)| c.is_some())
        .filter_map(|(i, _)| track.map.coord(i as i64))
        .collect_vec();
    track.find_cheats_n(&path_squares, &costs, max_length, cheat_min)
}

#[aoc(day20, part1)]
pub fn part1(input: &str) -> i64 {
    part1_impl(input, 100)
}

#[aoc(day20, part2)]
pub fn part2(input: &str) -> i64 {
    part2_impl(input, 20, 100)
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "###############
#...#...#.....#
#.#.#.#.#.###.#
#S#...#.#.#...#
#######.#.#.###
#######.#.#...#
#######.#.###.#
###..E#...#...#
###.#######.###
#...###...#...#
#.#####.#.###.#
#.#...#.#.#...#
#.#.#.#.#.#.###
#...#...#...###
###############";

    #[test]
    fn part1_example() {
        assert_eq!(part1_impl(EXAMPLE, 0), 44);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2_impl(EXAMPLE, 2, 0), 44);
        assert_eq!(part2_impl(EXAMPLE, 20, 50), 285);
    }
}
