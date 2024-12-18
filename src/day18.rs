use aoc_runner_derive::aoc;
use grid::Grid;
use itertools::Itertools;
use std::{
    cmp::Reverse,
    collections::{BinaryHeap, VecDeque},
};

#[derive(Clone)]
struct MemoryMap {
    map: Grid<bool>,
    byte_stream: Vec<(i64, i64)>,
}

trait PathTrack {
    const DOES_WORK: bool = true;
    fn new() -> Self;
    fn push(&mut self, pos: (i64, i64));
    fn finalize(&mut self) {}
}

struct LengthPath(usize);
impl PathTrack for LengthPath {
    fn new() -> Self {
        LengthPath(0)
    }
    fn push(&mut self, _: (i64, i64)) {
        self.0 += 1
    }
}

impl PathTrack for Vec<(i64, i64)> {
    fn new() -> Self {
        Vec::new()
    }
    fn push(&mut self, pos: (i64, i64)) {
        self.push(pos);
    }
    fn finalize(&mut self) {
        self.reverse();
    }
}

struct NoopTrack {}
impl PathTrack for NoopTrack {
    const DOES_WORK: bool = false;
    fn new() -> Self {
        Self {}
    }
    fn push(&mut self, _: (i64, i64)) {}
}

impl MemoryMap {
    fn from_str(input: &str, width: usize, height: usize) -> Self {
        let map = Grid::with_shape(width, height, true);
        let mut byte_stream = Vec::new();

        for line in input.lines() {
            if let Some((x, y)) = line.split_once(',') {
                let pos: (i64, i64) = (x.parse().unwrap(), y.parse().unwrap());
                byte_stream.push(pos);
            }
        }

        Self { map, byte_stream }
    }

    fn place_byte(&mut self, i: usize) {
        let pos = self.byte_stream[i];
        self.map.set(&pos, false);
    }

    fn place_bytes(&mut self, start: usize, end: usize) {
        for i in start..=end {
            self.place_byte(i);
        }
    }

    fn valid_moves<'a>(&'a self, pos: &'a (i64, i64)) -> impl Iterator<Item = (i64, i64)> + 'a {
        ([(0, 1), (1, 0), (0, -1), (-1, 0)])
            .iter()
            .filter(|ofs| self.map.get(&(pos.0 + ofs.0, pos.1 + ofs.1)).is_some_and(|v| *v))
            .map(|ofs| (pos.0 + ofs.0, pos.1 + ofs.1))
    }

    fn bfs<T: PathTrack>(&self, start: (i64, i64)) -> Option<T> {
        let goal = (self.map.width() as i64 - 1, self.map.height() as i64 - 1);

        let mut visited = self.map.same_shape(false);
        let mut prev = self.map.same_shape((i64::MAX, i64::MAX));
        let mut queue = VecDeque::new();

        visited.set(&start, true);
        queue.push_back((0, start));

        while let Some((depth, pos)) = queue.pop_front() {
            if pos == goal {
                if T::DOES_WORK {
                    let mut visited_pos = goal;
                    let mut path = T::new();
                    path.push(pos);
                    while let Some(next) = prev.get(&visited_pos) {
                        visited_pos = *next;
                        path.push(*next);
                        if *next == start {
                            path.finalize();
                            return Some(path);
                        }
                    }
                } else {
                    return Some(T::new());
                }
            }

            // if visited.get(&pos).is_some_and(|v| *v) {
            //     continue;
            // }

            let moves = self.valid_moves(&pos);
            for new_pos in moves {
                if visited.get(&new_pos).is_none_or(|v| !v) {
                    visited.set(&new_pos, true);
                    if T::DOES_WORK {
                        prev.set(&new_pos, pos);
                    }
                    queue.push_back((depth + 1, new_pos));
                }
            }
        }
        None
    }

    #[allow(dead_code)] // will be moved to Grid at some point
    fn dijkstra<T: PathTrack>(&self, start: (i64, i64)) -> Option<T> {
        let goal = (self.map.width() as i64 - 1, self.map.height() as i64 - 1);

        let mut costs = self.map.same_shape(i64::MAX);
        let mut prev = self.map.same_shape((i64::MAX, i64::MAX));
        let mut queue = BinaryHeap::new();

        costs.set(&start, 0);
        queue.push((Reverse(0), start));

        while let Some((cost, pos)) = queue.pop() {
            if pos == goal {
                if T::DOES_WORK {
                    let mut visited_pos = goal;
                    let mut path = T::new();
                    path.push(pos);
                    while let Some(next) = prev.get(&visited_pos) {
                        visited_pos = *next;
                        path.push(*next);
                        if *next == start {
                            path.finalize();
                            return Some(path);
                        }
                    }
                } else {
                    return Some(T::new());
                }
            }

            if costs.get(&pos).is_some_and(|v| cost.0 > *v) {
                continue;
            }

            let moves = self.valid_moves(&pos);
            for new_pos in moves {
                if costs.get(&new_pos).is_none_or(|best_cost| cost.0 + 1 < *best_cost) {
                    costs.set(&new_pos, cost.0 + 1);
                    if T::DOES_WORK {
                        prev.set(&new_pos, pos);
                    }
                    queue.push((Reverse(cost.0 + 1), new_pos));
                }
            }
        }
        None
    }
}

pub fn part1_impl(input: &str, width: usize, height: usize, initial_safe_byte_count: usize) -> usize {
    let mut map = MemoryMap::from_str(input, width, height);
    map.place_bytes(0, initial_safe_byte_count - 1);
    let path = map.bfs::<LengthPath>((0, 0)).expect("no path found");

    path.0 - 1 // count edges, not visited nodes (start doesn't count)
}

// My original devised solution
pub fn part2_impl_brute(input: &str, width: usize, height: usize, initial_safe_byte_count: usize) -> (i64, i64) {
    let mut input_map = MemoryMap::from_str(input, width, height);
    input_map.place_bytes(0, initial_safe_byte_count - 1);

    let mut path = input_map.bfs::<Vec<(i64, i64)>>((0, 0)).expect("no path found");

    for byte in initial_safe_byte_count..input_map.byte_stream.len() {
        input_map.place_byte(byte);
        // If it obstructs our best path, we need to do a new path search
        if let Some((obs_at, _)) = path.iter().find_position(|v| *v == &input_map.byte_stream[byte]) {
            let (before, _) = path.split_at(obs_at);

            if let Some(new_path) = input_map.bfs::<Vec<(i64, i64)>>(path[obs_at - 1]) {
                path = [before, &new_path].concat();
            } else {
                return input_map.byte_stream[byte];
            }
        }
    }
    panic!("no bytes block route");
}

// Optimized based on others' ideas
pub fn part2_impl(input: &str, width: usize, height: usize, initial_safe_byte_count: usize) -> (i64, i64) {
    let mut input_map = MemoryMap::from_str(input, width, height);

    input_map.place_bytes(0, initial_safe_byte_count - 1);

    // for the unplaced bytes, binary search for the partition point, given the predicate that a path is reachable
    // when all bytes up to that n have been placed
    let possible_problems = (initial_safe_byte_count..input_map.byte_stream.len()).collect_vec();
    let solution = possible_problems.partition_point(|byte| {
        // avoiding this clone by rolling back the byte placements instead is slower
        let mut local_map = input_map.clone();
        local_map.place_bytes(initial_safe_byte_count, *byte);
        local_map.bfs::<NoopTrack>((0, 0)).is_some()
    }) + initial_safe_byte_count;

    input_map.byte_stream[solution]
}

#[aoc(day18, part1)]
pub fn part1(input: &str) -> usize {
    part1_impl(input, 71, 71, 1024)
}

#[aoc(day18, part2)]
pub fn part2(input: &str) -> String {
    let sol = part2_impl(input, 71, 71, 1024);
    format!("{},{}", sol.0, sol.1)
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "5,4
4,2
4,5
3,0
2,1
6,3
2,4
1,5
0,6
3,3
2,6
5,1
1,2
5,5
2,5
6,5
1,4
0,4
6,4
1,1
6,1
1,0
0,5
1,6
2,0";

    #[test]
    fn part1_example() {
        assert_eq!(part1_impl(EXAMPLE, 7, 7, 12), 22);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2_impl(EXAMPLE, 7, 7, 12), (6, 1));
    }

    #[test]
    fn part2_example_brute() {
        assert_eq!(part2_impl_brute(EXAMPLE, 7, 7, 12,), (6, 1));
    }
}
