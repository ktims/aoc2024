use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
use std::collections::HashMap;
use std::iter::repeat;

type IntType = u64;
type CacheType = HashMap<Stone, IntType>;

#[derive(Clone, Debug, Hash, PartialEq, Eq)]
struct Stone(IntType);
struct Stones(Vec<Stone>);

enum BlinkResult {
    One(Stone),
    Two(Stone, Stone),
}

impl From<&str> for Stones {
    fn from(input: &str) -> Self {
        Stones(
            input
                .split_ascii_whitespace()
                .map(|v| Stone(v.parse().unwrap()))
                .collect_vec(),
        )
    }
}

#[aoc_generator(day11)]
fn parse(input: &str) -> Stones {
    Stones::from(input)
}

impl Stone {
    fn blink_once(&self) -> BlinkResult {
        let n_digits = if self.0 == 0 { 1 } else { self.0.ilog10() + 1 };
        if self.0 == 0 {
            BlinkResult::One(Stone(1))
        } else if n_digits % 2 == 0 {
            let split_factor = (10 as IntType).pow(n_digits / 2);
            let parts = (self.0 / split_factor, self.0 % split_factor);
            BlinkResult::Two(Stone(parts.0), Stone(parts.1))
        } else {
            BlinkResult::One(Stone(&self.0 * 2024))
        }
    }
}

fn count_blinks(stone: &Stone, blink: usize, cache: &mut Vec<CacheType>) -> IntType {
    if cache[blink].contains_key(&stone) {
        return cache[blink][&stone].clone();
    }
    let stones = stone.blink_once();
    let result = if blink == 0 {
        match stones {
            BlinkResult::One(_) => 1,
            BlinkResult::Two(_, _) => 2,
        }
    } else {
        match stones {
            BlinkResult::One(s) => count_blinks(&s, blink - 1, cache),
            BlinkResult::Two(s1, s2) => count_blinks(&s1, blink - 1, cache) + count_blinks(&s2, blink - 1, cache),
        }
    };
    cache[blink].insert(stone.clone(), result);
    cache[blink][&stone].clone()
}

fn blink_stones(stones: &Stones, blinks: usize) -> IntType {
    let mut cache = Vec::from_iter(repeat(CacheType::new()).take(blinks));
    stones
        .0
        .iter()
        .map(|stone| count_blinks(stone, blinks - 1, &mut cache))
        .sum()
}

#[aoc(day11, part1)]
fn part1(stones: &Stones) -> IntType {
    blink_stones(stones, 25)
}

#[aoc(day11, part2)]
fn part2(stones: &Stones) -> IntType {
    blink_stones(stones, 75)
}

#[cfg(test)]
mod tests {
    use super::*;
    pub const EXAMPLE: &str = &"125 17";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&parse(EXAMPLE)), 55312);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 65601038650482);
    }
}
