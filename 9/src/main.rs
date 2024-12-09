use std::collections::LinkedList;
use std::fmt::{Display, Write};
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

#[derive(Debug, Eq, PartialEq, Clone, Copy)]
enum Unit {
    File(usize),
    Free,
}

struct DiskMap {
    map: Vec<Unit>,
}

impl<T: BufRead> From<Lines<T>> for DiskMap {
    fn from(mut input: Lines<T>) -> Self {
        let line_s = input.next().unwrap().unwrap();
        let line = line_s.as_bytes();
        let mut file_id = 0;
        let mut map = Vec::new();
        for i in 0..line.len() {
            if i % 2 == 0 {
                // file
                for _ in 0..line[i] - b'0' {
                    map.push(Unit::File(file_id))
                }
                file_id += 1;
            } else {
                // free
                for _ in 0..line[i] - b'0' {
                    map.push(Unit::Free)
                }
            }
        }
        Self { map }
    }
}

impl Display for DiskMap {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for unit in &self.map {
            match unit {
                Unit::File(i) => f.write_char((b'0' + (*i % 10) as u8) as char)?,
                Unit::Free => f.write_char('.')?,
            }
        }
        Ok(())
    }
}

impl DiskMap {
    fn checksum(&self) -> u64 {
        self.map
            .iter()
            .enumerate()
            .map(|(i, u)| match u {
                Unit::File(id) => i * id,
                Unit::Free => 0,
            })
            .sum::<usize>() as u64
    }
}

// PROBLEM 1 solution

fn problem1<T: BufRead>(input: Lines<T>) -> u64 {
    let mut map = DiskMap::from(input);
    println!("{}", map);
    println!();
    let mut i = map.map.len() - 1;
    while i != 0 {
        // find start index of 'file'
        if map.map[i] == Unit::Free {
            i -= 1;
            continue;
        };
        let mut len = 1;
        while i >= 1 && map.map[i] == map.map[i - 1] {
            i -= 1;
            len += 1;
        }
        let file_id = match map.map[i] {
            Unit::File(id) => id,
            _ => panic!(),
        };
        let frees = map
            .map
            .iter()
            .enumerate()
            .take(i + len)
            .filter(|(_i, u)| **u == Unit::Free || **u == Unit::File(file_id))
            .map(|(i, _u)| i)
            .take(len)
            .collect_vec();
        if frees[0] == i && i > 0 {
            i -= 1;
            continue;
        }
        for j in 0..len {
            map.map.swap(frees[j], i + j);
        }
        if i > 0 {
            i -= 1;
        }
    }
    println!("{}", map);
    map.checksum()
}

// PROBLEM 2 solution
fn problem2<T: BufRead>(input: Lines<T>) -> u64 {
    let mut map = DiskMap::from(input);
    let mut i = map.map.len() - 1;
    while i != 0 {
        // find start index of 'file'
        if map.map[i] == Unit::Free {
            i -= 1;
            continue;
        };
        let mut len = 1;
        while i >= 1 && map.map[i] == map.map[i - 1] {
            i -= 1;
            len += 1;
        }
        let free_pos = map
            .map
            .windows(len)
            .enumerate()
            .find(|(i, u)| {
                u.iter().all(|u| match u {
                    Unit::Free => true,
                    _ => false,
                })
            })
            .map(|(i, _)| i);
        if let Some(free_pos) = free_pos {
            for j in 0..len {
                map.map[free_pos + j] = map.map[i + j];
                map.map[i + j] = Unit::Free;
            }
        }
        if i > 0 {
            i -= 1;
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::io::Cursor;

    const EXAMPLE: &str = &"2333133121414131402";

    #[test]
    fn problem1_example() {
        let c = Cursor::new(EXAMPLE);
        assert_eq!(problem1(c.lines()), 1928);
    }

    #[test]
    fn problem2_example() {
        let c = Cursor::new(EXAMPLE);
        assert_eq!(problem2(c.lines()), 0);
    }
}
