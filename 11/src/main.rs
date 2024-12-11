use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::iter::repeat;
use std::time::{Duration, Instant};

use itertools::Itertools;

// BOILERPLATE
type InputIter = Lines<BufReader<File>>;

pub fn get_input() -> InputIter {
    let f = File::open("input").unwrap();
    let br = BufReader::new(f);
    br.lines()
}

fn duration_format(duration: Duration) -> String {
    match duration.as_secs_f64() {
        x if x > 1.0 => format!("{:.3}s", x),
        x if x > 0.010 => format!("{:.3}ms", x * 1e3),
        x => format!("{:.3}us", x * 1e6),
    }
}

fn main() {
    let input = get_input();
    let start = Instant::now();
    let ans1 = problem1(input);
    let duration1 = start.elapsed();
    println!("Problem 1 solution: {} [{}]", ans1, duration_format(duration1));

    let input = get_input();
    let start = Instant::now();
    let ans2 = problem2(input);
    let duration2 = start.elapsed();
    println!("Problem 2 solution: {} [{}]", ans2, duration_format(duration2));
    println!("Total duration: {}", duration_format(duration1 + duration2));
}

#[derive(Clone, Copy, Debug, Hash, PartialEq, Eq)]
struct Stone {
    value: u64,
}

enum BlinkResult {
    One(Stone),
    Two(Stone, Stone),
}

impl Stone {
    fn blink_once(self) -> BlinkResult {
        let n_digits = if self.value == 0 { 1 } else { self.value.ilog10() + 1 };
        if self.value == 0 {
            BlinkResult::One(Stone { value: 1 })
        } else if n_digits % 2 == 0 {
            let parts = (
                self.value / 10u64.pow(n_digits / 2),
                self.value % 10u64.pow(n_digits / 2),
            );
            BlinkResult::Two(Stone { value: parts.0 }, Stone { value: parts.1 })
        } else {
            BlinkResult::One(Stone {
                value: self.value * 2024,
            })
        }
    }
    // #[allow(dead_code)]
    // fn blink(self, times: usize) -> Vec<Stone> {
    //     // Used in submitted part 1 solution
    //     let mut stones = vec![self];
    //     for _ in 0..times {
    //         stones = stones.iter().flat_map(|stone| stone.blink_once()).collect();
    //     }
    //     stones
    // }
}

fn count_blinks(stone: Stone, blink: usize, cache: &mut Vec<HashMap<Stone, u64>>) -> u64 {
    if cache[blink].contains_key(&stone) {
        return cache[blink][&stone];
    }
    let stones = stone.blink_once();
    let result = if blink == 0 {
        match stones {
            BlinkResult::One(_) => 1,
            BlinkResult::Two(_, _) => 2,
        }
    } else {
        match stones {
            BlinkResult::One(s) => count_blinks(s, blink - 1, cache),
            BlinkResult::Two(s1, s2) => count_blinks(s1, blink - 1, cache) + count_blinks(s2, blink - 1, cache),
        }
    };
    cache[blink].insert(stone, result);
    result
}

fn blink_stones(stones: &[Stone], blinks: usize) -> u64 {
    let mut cache = Vec::from_iter(repeat(HashMap::new()).take(blinks));
    stones
        .iter()
        .map(|stone| count_blinks(*stone, blinks - 1, &mut cache))
        .sum()
}

// PROBLEM 1 solution

fn problem1<T: BufRead>(mut input: Lines<T>) -> u64 {
    let stones = input
        .next()
        .unwrap()
        .unwrap()
        .split_ascii_whitespace()
        .map(|v| Stone {
            value: v.parse().unwrap(),
        })
        .collect_vec();
    blink_stones(&stones, 25)
}

// PROBLEM 2 solution
fn problem2<T: BufRead>(mut input: Lines<T>) -> u64 {
    let stones = input
        .next()
        .unwrap()
        .unwrap()
        .split_ascii_whitespace()
        .map(|v| Stone {
            value: v.parse().unwrap(),
        })
        .collect_vec();
    blink_stones(&stones, 75)
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::io::Cursor;

    const EXAMPLE: &str = &"125 17";

    #[test]
    fn problem1_example() {
        let c = Cursor::new(EXAMPLE);
        assert_eq!(problem1(c.lines()), 55312);
    }

    #[test]
    fn problem2_example() {
        let c = Cursor::new(EXAMPLE);
        assert_eq!(problem2(c.lines()), 65601038650482);
    }
}
