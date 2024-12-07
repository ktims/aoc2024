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

#[derive(Debug)]
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
}

#[derive(Debug)]
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
            Operator::Concatenate => u64::pow(10, b.to_string().len() as u32) * a + b,
        }
    }
}

impl<T: BufRead> From<Lines<T>> for Calibrations {
    fn from(input: Lines<T>) -> Self {
        let mut cals = Vec::new();
        for line in input.map(|l| l.unwrap()) {
            cals.push(line.as_str().into());
        }
        Self { cals }
    }
}

// PROBLEM 1 solution

fn problem1<T: BufRead>(input: Lines<T>) -> u64 {
    let cals = Calibrations::from(input);
    // println!("{:?}", cals);
    let mut sum = 0;

    let operators = [Operator::Add, Operator::Multiply];

    for cal in &cals.cals {
        let n_opers = cal.numbers.len() - 1;
        // println!("CAL: {:?} (opers: {})", cal, n_opers);
        for oper_set in std::iter::repeat_n(operators.iter(), n_opers).multi_cartesian_product() {
            // println!("operator set: {:?}", oper_set);
            let mut accum = cal.numbers[0];
            for (i, oper) in oper_set.iter().enumerate() {
                // println!("Testing {} {:?} {}", accum, oper, cal.numbers[i+1]);
                accum = oper.exec(accum, cal.numbers[i + 1]);
            }
            if accum == cal.result {
                sum += cal.result;
                // println!("Matched!");
                break;
            }
        }
        // println!("NO MATCHES");
    }
    sum
}

// PROBLEM 2 solution
fn problem2<T: BufRead>(input: Lines<T>) -> u64 {
    let cals = Calibrations::from(input);
    let mut sum = 0;
    let operators = [Operator::Add, Operator::Multiply, Operator::Concatenate];
    for cal in &cals.cals {
        let n_opers = cal.numbers.len() - 1;
        // println!("CAL: {:?} (opers: {})", cal, n_opers);
        for oper_set in std::iter::repeat_n(operators.iter(), n_opers).multi_cartesian_product() {
            // println!("operator set: {:?}", oper_set);
            let mut accum = cal.numbers[0];
            for (i, oper) in oper_set.iter().enumerate() {
                // println!("Testing {} {:?} {}", accum, oper, cal.numbers[i+1]);
                accum = oper.exec(accum, cal.numbers[i + 1]);
            }
            if accum == cal.result {
                sum += cal.result;
                // println!("Matched!");
                break;
            }
        }
        // println!("NO MATCHES");
    }
    sum
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
