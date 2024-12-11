use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
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

impl Stone {
    fn blink_once(self) -> Vec<Stone> {
        let n_digits = if self.value == 0 { 1 } else { self.value.ilog10() + 1 };
        if self.value == 0 {
            vec![Stone { value: 1 }]
        } else if n_digits % 2 == 0 {
            let parts = (
                self.value / 10u64.pow(n_digits / 2),
                self.value % 10u64.pow(n_digits / 2),
            );
            vec![Stone { value: parts.0 }, Stone { value: parts.1 }]
        } else {
            vec![Stone {
                value: self.value * 2024,
            }]
        }
    }
    fn blink(self, times: usize) -> Vec<Stone> {
        let mut stones = vec![self];
        for _ in 0..times {
            stones = stones.iter().flat_map(|stone| stone.blink_once()).collect();
        }
        stones
    }
}

fn count_blinks(stone: Stone, blinks: u64, cache: &mut HashMap<(u64, Stone), u64>) -> u64 {
    if cache.contains_key(&(blinks, stone)) {
        return cache[&(blinks, stone)];
    }
    let stones = stone.blink_once();
    let result = if blinks == 1 {
        stones.len() as u64
    } else {
        stones.iter().map(|s| count_blinks(*s, blinks - 1, cache)).sum()
    };
    cache.insert((blinks, stone), result);
    result
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
    stones.iter().flat_map(|stone| stone.blink(25)).count() as u64
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
    let mut cache = HashMap::new();
    stones.iter().map(|s| count_blinks(*s, 75, &mut cache)).sum()
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
