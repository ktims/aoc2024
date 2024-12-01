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

// PROBLEM 1 solution

#[derive(Debug)]
struct Entry {
    pos: usize,
    val: u32,
}

fn problem1<T: BufRead>(input: Lines<T>) -> u64 {
    let mut left = Vec::new();
    let mut right = Vec::new();
    for (pos, line) in input.enumerate() {
        if line.is_err() {
            panic!("can't read line");
        }
        let line = line.unwrap();
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();
        left.push(Entry {
            pos,
            val: parts[0].parse::<u32>().unwrap(),
        });
        right.push(Entry {
            pos,
            val: parts[1].parse::<u32>().unwrap(),
        });
    }
    left.sort_by_key(|entry| entry.val);
    right.sort_by_key(|entry| entry.val);

    println!("{:?}", right);

    zip(left, right)
        .map(|(left, right)| {
            println!("{:?} {:?}", left, right);
            u64::abs_diff(left.val as u64, right.val as u64)
        })
        .sum()
}

// PROBLEM 2 solution
fn problem2<T: BufRead>(input: Lines<T>) -> u64 {
    let mut left = Vec::new();
    let mut right_count = HashMap::new();
    for (pos, line) in input.enumerate() {
        if line.is_err() {
            panic!("can't read line");
        }
        let line = line.unwrap();
        let parts: Vec<&str> = line.split_ascii_whitespace().collect();
        left.push(parts[0].parse::<u32>().unwrap());
        let right = parts[1].parse::<u32>().unwrap();
        right_count.insert(right, *right_count.get(&right).unwrap_or(&0) + 1);
    }
    left.iter().map(|l| l * right_count.get(l).unwrap_or(&0)).sum::<u32>() as u64
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
