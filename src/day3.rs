use aoc_runner_derive::{aoc, aoc_generator};
use atoi::FromRadix10;
use regex::bytes::Regex;
use std::io::BufRead;

#[aoc_generator(day3)]
pub fn get_input(input: &[u8]) -> Vec<String> {
    input.lines().map(|l| l.unwrap()).collect()
}

// PROBLEM 1 solution
#[aoc(day3, part1)]
pub fn part1(input: &Vec<String>) -> u64 {
    let re = Regex::new(r"(?-u)mul\((\d+),(\d+)\)").unwrap();
    input
        .iter()
        .map(|line| {
            re.captures_iter(line.as_bytes())
                .map(|m| u64::from_radix_10(&m[1]).0 * u64::from_radix_10(&m[2]).0)
                .sum::<u64>()
        })
        .sum()
}

// PROBLEM 2 solution
#[aoc(day3, part2)]
pub fn part2(input: &Vec<String>) -> u64 {
    let mut sum = 0u64;
    let mut do_mul: u64 = 1;
    let re = Regex::new(r"(?-u)(do\(\)|don't\(\)|mul\((\d+),(\d+)\))").unwrap();
    for line in input {
        let line = line.as_bytes();
        for m in re.captures_iter(line) {
            match &m[1] {
                b"do()" => do_mul = 1,
                b"don't()" => do_mul = 0,
                _ => {
                    sum += u64::from_radix_10(&m[2]).0 * u64::from_radix_10(&m[3]).0 * do_mul;
                }
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::day3::*;

    const EXAMPLE1: &[u8] = b"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const EXAMPLE2: &[u8] = b"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn part1_example() {
        let input = get_input(EXAMPLE1);
        assert_eq!(part1(&input), 161);
    }

    #[test]
    fn part2_example() {
        let input = get_input(EXAMPLE2);
        assert_eq!(part2(&input), 48);
    }
}
