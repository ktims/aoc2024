use std::{cmp::Reverse, collections::BinaryHeap};

use aoc_runner_derive::aoc;
use grid::Grid;

#[derive(Clone)]
struct MemoryMap {
    map: Grid<bool>,
    byte_stream: Vec<(i64, i64)>,
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
        if self.map.set(&pos, false).is_none() {
            panic!("corruption outside memory bounds");
        }
    }
    fn place_bytes(&mut self, n: usize) {
        assert!(n < self.byte_stream.len());
        for i in 0..n {
            self.place_byte(i);
        }
    }

    fn valid_moves(&self, pos: &(i64, i64)) -> Vec<(i64, i64)> {
        [(0, 1), (1, 0), (0, -1), (-1, 0)]
            .iter()
            .filter(|ofs| self.map.get(&(pos.0 + ofs.0, pos.1 + ofs.1)).is_some_and(|v| *v))
            .map(|ofs| (pos.0 + ofs.0, pos.1 + ofs.1))
            .collect()
    }

    fn dijkstra(&self) -> Option<Vec<(i64, i64)>> {
        let start = (0i64, 0i64);
        let goal = (self.map.width() as i64 - 1, self.map.height() as i64 - 1);

        let mut distances = self.map.same_shape(i64::MAX);
        let mut prev = self.map.same_shape((i64::MAX, i64::MAX));
        let mut queue = BinaryHeap::new();

        distances.set(&start, 0);
        queue.push((Reverse(0), start));

        while let Some((cost, pos)) = queue.pop() {
            if pos == goal {
                let mut visited = Vec::new();
                let mut visited_pos = goal;

                visited.push(pos);
                while let Some(next) = prev.get(&visited_pos) {
                    visited_pos = *next;
                    visited.push(*next);
                }
                return Some(visited);
            }

            if distances.get(&pos).is_some_and(|v| cost.0 > *v) {
                continue;
            }

            for new_pos in self.valid_moves(&pos) {
                if distances.get(&new_pos).is_none_or(|best_cost| cost.0 + 1 < *best_cost) {
                    distances.set(&new_pos, cost.0 + 1);
                    prev.set(&new_pos, pos);
                    queue.push((Reverse(cost.0 + 1), new_pos));
                }
            }
        }
        None
    }
}

pub fn part1_impl(input: &str, width: usize, height: usize, n: usize) -> usize {
    let mut map = MemoryMap::from_str(input, width, height);
    map.place_bytes(n);
    let path = map.dijkstra().expect("no path found");
    let mut sol_map = map.map.same_shape(b'.');
    sol_map.data = map
        .map
        .data
        .iter()
        .map(|clear| if *clear { b'.' } else { b'#' })
        .collect();
    for visited in &path {
        sol_map.set(visited, b'O');
    }

    path.len() - 2 // count vertexes, not visited nodes
}

pub fn part2_impl(input: &str, width: usize, height: usize, n: usize) -> (i64, i64) {
    let mut input_map = MemoryMap::from_str(input, width, height);

    input_map.place_bytes(n);
    let mut path = input_map.dijkstra().expect("no path found");

    for byte in n..input_map.byte_stream.len() {
        input_map.place_byte(byte);
        if path.contains(&input_map.byte_stream[byte]) {
            if let Some(new_path) = input_map.dijkstra() {
                path = new_path;
            } else {
                println!("obstruction found on trial {}", byte);
                return input_map.byte_stream[byte];
            }
        }
    }
    panic!("no bytes block route");
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
}
