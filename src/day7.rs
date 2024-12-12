use aoc_runner_derive::{aoc, aoc_generator};
use itertools::Itertools;
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
    longest_cal: usize,
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
        let mut cals = Vec::new();
        let mut longest_cal = 0;
        for line in input.map(|l| l.unwrap()) {
            let cal: Calibration = line.as_str().into();
            longest_cal = std::cmp::max(longest_cal, cal.numbers.len());
            cals.push(cal);
        }
        Self { cals, longest_cal }
    }
}

impl Calibrations {
    fn make_operator_sets(operators: &[Operator], n_opers: usize) -> Vec<Vec<Vec<Operator>>> {
        (0..n_opers)
            .map(|k| {
                std::iter::repeat_n(operators.iter().copied(), k)
                    .multi_cartesian_product()
                    .collect()
            })
            .collect()
    }
    fn check_oper_set(cal: &Calibration, oper_set: &[Operator]) -> bool {
        let accum = oper_set
            .iter()
            .zip(cal.numbers.iter().skip(1))
            .fold(cal.numbers[0], |accum, (oper, val)| oper.exec(accum, *val));
        accum == cal.result
    }
    fn possible(&self, operators: &[Operator]) -> u64 {
        let operator_sets = Calibrations::make_operator_sets(operators, self.longest_cal);
        self.cals
            .par_iter()
            .map(|cal| {
                let n_opers = cal.numbers.len() - 1;
                if operator_sets[n_opers]
                    .par_iter()
                    .find_any(|oper_set| Self::check_oper_set(cal, oper_set))
                    .is_some()
                {
                    return cal.result;
                }
                0
            })
            .sum()
    }
}

// PROBLEM 1 solution
#[aoc(day7, part1)]
pub fn part1(cals: &Calibrations) -> u64 {
    let operators = [Operator::Add, Operator::Multiply];
    cals.possible(&operators)
}

// PROBLEM 2 solution
#[aoc(day7, part2)]
pub fn part2(cals: &Calibrations) -> u64 {
    let operators = [Operator::Add, Operator::Multiply, Operator::Concatenate];
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
