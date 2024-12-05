use regex::bytes::Regex;
use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::time::{Duration, Instant};

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

// PROBLEM 1 solution

fn problem1<T: BufRead>(input: Lines<T>) -> u64 {
    let mut sum = 0u64;
    let re = Regex::new(r"(?-u)mul\((\d+),(\d+)\)").unwrap();
    for line in input.map(|i| i.unwrap()) {
        let line = line.as_bytes();
        for m in re.captures_iter(line) {
            sum += std::str::from_utf8(&m[1]).unwrap().parse::<u64>().unwrap()
                * std::str::from_utf8(&m[2]).unwrap().parse::<u64>().unwrap();
        }
    }
    sum
}

// PROBLEM 2 solution
fn problem2<T: BufRead>(input: Lines<T>) -> u64 {
    let mut sum = 0u64;
    let mut do_mul = true;
    let re = Regex::new(r"(?-u)(do\(\)|don't\(\)|mul\((\d+),(\d+)\))").unwrap();
    for line in input.map(|i| i.unwrap()) {
        let line = line.as_bytes();
        for m in re.captures_iter(line) {
            match std::str::from_utf8(&m[1]).unwrap() {
                "do()" => do_mul = true,
                "don't()" => do_mul = false,
                _ if do_mul => {
                    sum += std::str::from_utf8(&m[2]).unwrap().parse::<u64>().unwrap()
                        * std::str::from_utf8(&m[3]).unwrap().parse::<u64>().unwrap()
                }
                _ => {}
            }
        }
    }
    sum
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::io::Cursor;

    const EXAMPLE1: &str = &"xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
    const EXAMPLE2: &str = &"xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";

    #[test]
    fn problem1_example() {
        let c = Cursor::new(EXAMPLE1);
        assert_eq!(problem1(c.lines()), 161);
    }

    #[test]
    fn problem2_example() {
        let c = Cursor::new(EXAMPLE2);
        assert_eq!(problem2(c.lines()), 48);
    }
}
