use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
};

use aoc_runner_derive::aoc;
use grid::Grid;
use rustc_hash::{FxHashMap, FxHashSet};

trait PathTrack {
    const DOES_WORK: bool = true;
    fn new() -> Self;
    fn push(&mut self, pos: (i64, i64));
    fn finalize(&mut self) {}
}

struct RaceTrack {
    map: Grid<u8>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
struct State {
    pos: (i64, i64),
    cost: usize,
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct CheatState {
    s: State,
    p: Vec<(i64, i64)>,
}

const DIRECTIONS: [(i64, i64); 4] = [(-1, 0), (1, 0), (0, -1), (0, 1)];

impl RaceTrack {
    fn valid_moves<'a>(&'a self, CheatState { s: state, p }: &'a CheatState) -> impl Iterator<Item = CheatState> + 'a {
        let mut new_path = p.clone();
        new_path.push(state.pos);
        DIRECTIONS
            .iter()
            .map(|dir| (state.pos.0 + dir.0, state.pos.1 + dir.1))
            .filter_map(move |pos| match &self.map.get(&pos) {
                Some(b'.') | Some(b'S') | Some(b'E') => Some(CheatState {
                    p: new_path.clone(),
                    s: State {
                        pos,
                        cost: state.cost + 1,
                    },
                }),
                _ => None,
            })
    }
    fn path_costs(&self, start: (i64, i64), goal: (i64, i64)) -> (Vec<(i64, i64)>, Grid<Option<usize>>) {
        let mut queue = VecDeque::new();
        let mut visited = self.map.same_shape(None);

        let start_state = CheatState {
            s: State {
                pos: start,
                cost: 0usize,
            },
            p: Vec::new(),
        };
        visited.set(&start, Some(0));
        queue.push_back(start_state);

        while let Some(state) = queue.pop_front() {
            if state.s.pos == goal {
                let mut final_path = state.p;
                final_path.push(goal);
                return (final_path, visited);
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

    fn find_cheats(
        &self,
        path: &Vec<(i64, i64)>,
        costs: &Grid<Option<usize>>,
        min: usize,
    ) -> Vec<((i64, i64), (i64, i64), usize)> {
        let mut cheats = Vec::new();
        for pos in path {
            let local_cost = costs.get(pos).unwrap().unwrap();
            for ofs in DIRECTIONS {
                let cheat_start = (pos.0 + ofs.0, pos.1 + ofs.1);
                let cheat_exit = (pos.0 + ofs.0 * 2, pos.1 + ofs.1 * 2);
                if let Some(Some(cheat_cost)) = costs.get(&cheat_exit) {
                    if *cheat_cost > local_cost + 2 {
                        let cheat_savings = cheat_cost - local_cost - 2;
                        if cheat_savings >= min {
                            cheats.push((cheat_start, cheat_exit, cheat_savings));
                        }
                    }
                }
            }
        }
        cheats
    }
}

fn parse(input: &str) -> RaceTrack {
    let map = input.as_bytes().into();
    RaceTrack { map }
}

fn part1_impl(input: &str, cheat_min: usize) -> i64 {
    let track = parse(input);
    let start = track.map.find(&b'S').unwrap();
    let goal = track.map.find(&b'E').unwrap();
    let (best_path, costs) = track.path_costs(start.into(), goal.into());
    let cheats = track.find_cheats(&best_path, &costs, cheat_min);

    cheats.len() as i64
}

#[aoc(day20, part1)]
pub fn part1(input: &str) -> i64 {
    part1_impl(input, 100)
}

#[aoc(day20, part2)]
pub fn part2(input: &str) -> i64 {
    todo!()
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
        assert_eq!(part2(EXAMPLE), 0);
    }
}
