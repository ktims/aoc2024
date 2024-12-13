use aoc_runner_derive::{aoc, aoc_generator};
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use std::io::{BufRead, Lines};

#[aoc_generator(day7)]
pub fn get_input(input: &[u8]) -> Calibrations {
    Calibrations::from(input.lines())
}

#[derive(Debug, Clone)]
struct Calibration {
    result: u64,
    numbers: Vec<u64>,
}

impl From<&str> for Calibration {
    fn from(value: &str) -> Self {
        let (result, rest) = value.split_once(':').unwrap();
        Self {
            result: result.parse().unwrap(),
            numbers: rest.split_ascii_whitespace().map(|s| s.parse().unwrap()).collect(),
        }
    }
}

#[derive(Debug)]
pub struct Calibrations {
    cals: Vec<Calibration>,
}

#[derive(Debug, Copy, Clone)]
enum Operator {
    Add,
    Multiply,
    Concatenate,
}

impl Operator {
    fn exec(&self, a: u64, b: u64) -> u64 {
        match self {
            Operator::Add => a + b,
            Operator::Multiply => a * b,
            Operator::Concatenate => u64::pow(10, b.ilog10() + 1) * a + b,
        }
    }
}

impl<T: BufRead> From<Lines<T>> for Calibrations {
    fn from(input: Lines<T>) -> Self {
        let cals = input.map(|l| l.unwrap().as_str().into()).collect();
        Self { cals }
    }
}

impl Calibrations {
    fn possible(&self, operators: &[Operator]) -> u64 {
        self.cals
            .par_iter()
            .map(|cal| eval_calibration(operators, cal.result, cal.numbers[0], &cal.numbers[1..]))
            .map(|result| result.unwrap_or(0))
            .sum()
    }
}

fn eval_calibration(operators: &[Operator], expect: u64, left: u64, right: &[u64]) -> Option<u64> {
    if left > expect {
        // all operations make the number larger, so this branch is hopeless, early exit
        return None;
    }
    if right.is_empty() {
        // base case - no further operations
        if left == expect {
            return Some(left);
        } else {
            return None;
        }
    }
    operators
        .iter()
        .map(|oper| eval_calibration(operators, expect, oper.exec(left, right[0]), &right[1..]))
        .find_map(|result| result)
}

// PROBLEM 1 solution
#[aoc(day7, part1)]
pub fn part1(cals: &Calibrations) -> u64 {
    let operators = [Operator::Multiply, Operator::Add];
    cals.possible(&operators)
}

// PROBLEM 2 solution
#[aoc(day7, part2)]
pub fn part2(cals: &Calibrations) -> u64 {
    let operators = [Operator::Multiply, Operator::Add, Operator::Concatenate];
    cals.possible(&operators)
}

#[cfg(test)]
mod tests {
    use crate::day7::*;

    const EXAMPLE: &[u8] = b"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&get_input(EXAMPLE)), 3749);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&get_input(EXAMPLE)), 11387);
    }
}
