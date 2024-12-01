use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::iter::zip;
use std::time::Instant;

// BOILERPLATE
type InputIter = Lines<BufReader<File>>;

fn get_input() -> InputIter {
    let f = File::open("input").unwrap();
    let br = BufReader::new(f);
    br.lines()
}

fn main() {
    let start = Instant::now();
    let ans1 = problem1(get_input());
    let duration = start.elapsed();
    println!("Problem 1 solution: {} [{}s]", ans1, duration.as_secs_f64());

    let start = Instant::now();
    let ans2 = problem2(get_input());
    let duration = start.elapsed();
    println!("Problem 2 solution: {} [{}s]", ans2, duration.as_secs_f64());
}

struct Locations {
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
        let mut right_count: HashMap<u64, u64> = HashMap::new();
        for rval in &self.right {
            right_count.insert(*rval, *right_count.get(rval).unwrap_or(&0) + 1);
        }
        right_count
    }
}

// PROBLEM 1 solution

fn problem1<T: BufRead>(input: Lines<T>) -> u64 {
    let mut locations = Locations::from(input);
    locations.sort();

    zip(locations.left, locations.right)
        .map(|(l, r)| u64::abs_diff(l, r))
        .sum()
}

// PROBLEM 2 solution
fn problem2<T: BufRead>(input: Lines<T>) -> u64 {
    let locations = Locations::from(input);
    let right_count = locations.right_count();
    locations
        .left
        .iter()
        .map(|l| l * right_count.get(l).unwrap_or(&0))
        .sum::<u64>()
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::io::Cursor;

    const EXAMPLE: &str = &"3   4
4   3
2   5
1   3
3   9
3   3";

    #[test]
    fn problem1_example() {
        let c = Cursor::new(EXAMPLE);
        assert_eq!(problem1(c.lines()), 11);
    }

    #[test]
    fn problem2_example() {
        let c = Cursor::new(EXAMPLE);
        assert_eq!(problem2(c.lines()), 31);
    }
}
