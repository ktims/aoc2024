use aoc_runner_derive::{aoc, aoc_generator};
use rustc_hash::FxHashMap;
use std::io::{BufRead, Lines};

type HashMap<K, V> = FxHashMap<K, V>;

#[aoc_generator(day1)]
pub fn get_input(input: &[u8]) -> Locations {
    Locations::from(input.lines())
}

#[derive(Clone)]
pub struct Locations {
    left: Vec<u64>,
    right: Vec<u64>,
}

impl<T: BufRead> From<Lines<T>> for Locations {
    fn from(input: Lines<T>) -> Self {
        let mut left = Vec::new();
        let mut right = Vec::new();
        for line in input.map(|i| i.unwrap()) {
            let parts: Vec<&str> = line.split_ascii_whitespace().collect();
            left.push(parts[0].parse::<u64>().unwrap());
            right.push(parts[1].parse::<u64>().unwrap());
        }
        Self { left, right }
    }
}

impl Locations {
    fn sort(&mut self) {
        self.left.sort();
        self.right.sort();
    }
    fn right_count(&self) -> HashMap<u64, u64> {
        let mut right_count: HashMap<u64, u64> = HashMap::default();
        for rval in &self.right {
            right_count.insert(*rval, *right_count.get(rval).unwrap_or(&0) + 1);
        }
        right_count
    }
}

// PROBLEM 1 solution
#[aoc(day1, part1)]
pub fn part1(locations: &Locations) -> u64 {
    let mut locations = locations.clone();
    locations.sort();

    locations
        .left
        .iter()
        .zip(locations.right)
        .map(|(l, r)| u64::abs_diff(*l, r))
        .sum()
}

// PROBLEM 2 solution
#[aoc(day1, part2)]
pub fn part2(locations: &Locations) -> u64 {
    let right_count = locations.right_count();
    locations
        .left
        .iter()
        .map(|l| l * right_count.get(l).unwrap_or(&0))
        .sum::<u64>()
}

#[cfg(test)]
mod tests {
    use crate::day1::*;

    const EXAMPLE: &[u8] = b"3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn part1_example() {
        let input = get_input(EXAMPLE);
        assert_eq!(part1(&input), 11);
    }

    #[test]
    fn part2_example() {
        let input = get_input(EXAMPLE);
        assert_eq!(part2(&input), 31);
    }
}
