use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::time::{Duration, Instant};

// BOILERPLATE
type InputIter = Lines<BufReader<File>>;

fn get_input() -> InputIter {
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

struct Reports {
    reports: Vec<Vec<u64>>,
}

impl<T: BufRead> From<Lines<T>> for Reports {
    fn from(lines: Lines<T>) -> Self {
        let mut reports = Vec::new();
        for line in lines.map(|i| i.unwrap()) {
            reports.push(
                line.split_ascii_whitespace()
                    .map(|record| record.parse::<u64>().unwrap())
                    .collect(),
            )
        }
        Reports { reports }
    }
}

impl Reports {
    fn is_safe(report: &Vec<u64>) -> bool {
        let mut ascending: bool = true;
        let mut descending: bool = true;
        for (a, b) in report.iter().zip(report.iter().skip(1)) {
            if a > b {
                ascending = false
            }
            if a < b {
                descending = false;
            }
            let ad = a.abs_diff(*b);
            if !(ad >= 1 && ad <= 3) || (!ascending && !descending) {
                return false;
            };
        }
        return true;
    }
    fn count_safe(&self) -> u64 {
        self.reports.iter().filter(|report| Self::is_safe(report)).count() as u64
    }
    fn is_dumb_dampened_safe(report: &Vec<u64>) -> bool {
        if Self::is_safe(report) {
            return true;
        }
        for i in 0..report.len() {
            let mut new_vec = report.clone();
            new_vec.remove(i);
            if Self::is_safe(&new_vec) {
                return true;
            }
        }
        false
    }
    fn dampened_count_safe(&self) -> u64 {
        self.reports
            .iter()
            .filter(|report| Self::is_dumb_dampened_safe(report))
            .count() as u64
    }
}

// PROBLEM 1 solution

fn problem1<T: BufRead>(input: Lines<T>) -> u64 {
    let reports = Reports::from(input);
    reports.count_safe()
}

// PROBLEM 2 solution
fn problem2<T: BufRead>(input: Lines<T>) -> u64 {
    let reports = Reports::from(input);
    reports.dampened_count_safe()
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::io::Cursor;

    const EXAMPLE: &str = &"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn problem1_example() {
        let c = Cursor::new(EXAMPLE);
        assert_eq!(problem1(c.lines()), 2);
    }

    #[test]
    fn problem2_example() {
        let c = Cursor::new(EXAMPLE);
        assert_eq!(problem2(c.lines()), 4);
    }
}
