use aoc_runner_derive::{aoc, aoc_generator};
use grid::Grid;
use itertools::Itertools;
use std::io::{BufRead, Lines};

#[aoc_generator(day10)]
pub fn get_input(input: &[u8]) -> TrailMap {
    TrailMap::from(input.lines())
}

pub struct TrailMap {
    map: Grid<u8>,
}

impl<T: BufRead> From<Lines<T>> for TrailMap {
    fn from(input: Lines<T>) -> Self {
        Self { map: input.into() }
    }
}

impl TrailMap {
    fn trailheads(&self) -> Vec<(i64, i64)> {
        self.map
            .data
            .iter()
            .enumerate()
            .filter(|(_, v)| **v == b'0')
            .map(|(i, _v)| self.map.coord(i as i64).unwrap())
            .collect_vec()
    }
    fn count_reachable_from(&self, pos: &(i64, i64), needle: u8, visited: &mut Grid<bool>) -> u64 {
        if visited.get(pos) == Some(true) {
            return 0;
        } else {
            visited.set(pos, true);
        }
        let our_val = self.map.get(pos).unwrap();
        if our_val == needle {
            return 1;
        }
        // adjacents that are +1
        [(-1, 0), (1, 0), (0, -1), (0, 1)] // left, right, up, down
            .iter()
            .map(|(x_ofs, y_ofs)| (pos.0 + x_ofs, pos.1 + y_ofs)) // get target position
            .map(|target_pos| (target_pos, self.map.get(&target_pos))) // get value at that position
            .filter(|(_, val)| *val == Some(our_val + 1)) // only interested if it's our value + 1
            .map(|(pos, _)| pos) // discard the value
            .map(|pos| self.count_reachable_from(&pos, needle, visited))
            .sum()
    }

    fn count_paths_to(&self, pos: &(i64, i64), needle: u8) -> u64 {
        let our_val = self.map.get(pos).unwrap();
        if our_val == needle {
            return 1;
        }
        [(-1, 0), (1, 0), (0, -1), (0, 1)] // left, right, up, down
            .iter()
            .map(|(x_ofs, y_ofs)| (pos.0 + x_ofs, pos.1 + y_ofs)) // get target position
            .map(|target_pos| (target_pos, self.map.get(&target_pos))) // get value at that position
            .filter(|(_, val)| *val == Some(our_val + 1)) // only interested if it's our value + 1
            .map(|(pos, _)| pos) // discard the value
            .map(|mov| self.count_paths_to(&mov, needle))
            .sum::<u64>()
    }
}

// PROBLEM 1 solution
#[aoc(day10, part1)]
pub fn part1(map: &TrailMap) -> u64 {
    map.trailheads()
        .iter()
        .map(|pos| {
            let mut visited = Grid::with_shape(map.map.width(), map.map.height(), false);
            map.count_reachable_from(pos, b'9', &mut visited)
        })
        .sum()
}

// PROBLEM 2 solution
#[aoc(day10, part2)]
pub fn part2(map: &TrailMap) -> u64 {
    map.trailheads()
        .iter()
        .map(|pos| map.count_paths_to(pos, b'9'))
        .sum::<u64>()
}

#[cfg(test)]
mod tests {
    use crate::day10::*;

    const EXAMPLE: &[u8] = b"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&get_input(EXAMPLE)), 36);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&get_input(EXAMPLE)), 81);
    }
}
