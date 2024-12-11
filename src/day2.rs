use aoc_runner_derive::{aoc, aoc_generator};
use std::io::{BufRead, Lines};

#[aoc_generator(day2)]
pub fn get_input(input: &[u8]) -> Reports {
    Reports::from(input.lines())
}

#[derive(Debug)]
pub struct Reports {
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
    fn is_safe(report: &[u64]) -> bool {
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
            if !(1..=3).contains(&ad) || (!ascending && !descending) {
                return false;
            };
        }
        true
    }
    fn count_safe(&self) -> u64 {
        self.reports.iter().filter(|report| Self::is_safe(report)).count() as u64
    }
    fn is_dumb_dampened_safe(report: &[u64]) -> bool {
        if Self::is_safe(report) {
            return true;
        }
        for i in 0..report.len() {
            let mut new_vec = report.to_owned();
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
#[aoc(day2, part1)]
pub fn part1(input: &Reports) -> u64 {
    input.count_safe()
}

// PROBLEM 2 solution
#[aoc(day2, part2)]
pub fn part2(input: &Reports) -> u64 {
    input.dampened_count_safe()
}

#[cfg(test)]
mod tests {
    use crate::day2::*;

    const EXAMPLE: &[u8] = b"7 6 4 2 1
1 2 7 8 9
9 7 6 2 1
1 3 2 4 5
8 6 4 4 1
1 3 6 7 9";

    #[test]
    fn part1_example() {
        let input = get_input(EXAMPLE);
        println!("{:?}", input);
        assert_eq!(part1(&input), 2);
    }

    #[test]
    fn part2_example() {
        let input = get_input(EXAMPLE);
        assert_eq!(part2(&input), 4);
    }
}
