use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::time::{Duration, Instant};

use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, IntoParallelRefIterator, ParallelIterator};
use thread_local::ThreadLocal;

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

#[derive(Debug, Clone)]
struct Calibration {
    result: u64,
    numbers: Vec<u64>,
}

impl From<&str> for Calibration {
    fn from(value: &str) -> Self {
        let (result, rest) = value.splitn(2, ':').next_tuple().unwrap();
        Self {
            result: result.parse().unwrap(),
            numbers: rest.split_ascii_whitespace().map(|s| s.parse().unwrap()).collect(),
        }
    }
}

#[derive(Debug)]
struct Calibrations {
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
            Operator::Concatenate => u64::pow(10, u64::ilog10(b) + 1) * a + b,
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
                std::iter::repeat_n(operators.iter().map(|v| *v), k)
                    .multi_cartesian_product()
                    .collect()
            })
            .collect()
    }
    fn check_oper_set(cal: &Calibration, oper_set: &Vec<Operator>) -> bool {
        let accum = oper_set
            .iter()
            .zip(cal.numbers.iter().skip(1))
            .fold(cal.numbers[0], |accum, (oper, val)| oper.exec(accum, *val));
        if accum == cal.result {
            true
        } else {
            false
        }
    }
    fn possible(&self, operators: &[Operator]) -> u64 {
        let operator_sets = Calibrations::make_operator_sets(operators, self.longest_cal);
        self.cals
            .par_iter()
            .map(|cal| {
                let n_opers = cal.numbers.len() - 1;
                let tl = ThreadLocal::new();
                if operator_sets[n_opers]
                    .par_iter()
                    .find_any(|oper_set| Self::check_oper_set(&cal, &oper_set))
                    .is_some()
                {
                    let cal_local = tl.get_or(|| cal.clone());
                    return cal_local.result;
                }
                0
            })
            .sum()
    }
}

// PROBLEM 1 solution

fn problem1<T: BufRead>(input: Lines<T>) -> u64 {
    let cals = Calibrations::from(input);
    let operators = [Operator::Add, Operator::Multiply];
    cals.possible(&operators)
}

// PROBLEM 2 solution
fn problem2<T: BufRead>(input: Lines<T>) -> u64 {
    let cals = Calibrations::from(input);
    let operators = [Operator::Add, Operator::Multiply, Operator::Concatenate];
    cals.possible(&operators)
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::io::Cursor;

    const EXAMPLE: &str = &"190: 10 19
3267: 81 40 27
83: 17 5
156: 15 6
7290: 6 8 6 15
161011: 16 10 13
192: 17 8 14
21037: 9 7 18 13
292: 11 6 16 20";

    #[test]
    fn problem1_example() {
        let c = Cursor::new(EXAMPLE);
        assert_eq!(problem1(c.lines()), 3749);
    }

    #[test]
    fn problem2_example() {
        let c = Cursor::new(EXAMPLE);
        assert_eq!(problem2(c.lines()), 11387);
    }
}
