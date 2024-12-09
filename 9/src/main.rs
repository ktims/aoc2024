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

struct Inode {
    id: usize,
    pos: usize,
    len: u8,
}

struct DiskMap {
    map: Vec<Unit>,
    files: Vec<Inode>,
}

impl<T: BufRead> From<Lines<T>> for DiskMap {
    fn from(mut input: Lines<T>) -> Self {
        let line_s = input.next().unwrap().unwrap();
        let line = line_s.as_bytes();
        let mut file_id = 0;
        let mut map = Vec::new();
        let mut files = Vec::new();
        for i in 0..line.len() {
            let len = line[i] - b'0';
            if i % 2 == 0 {
                // file
                files.push(Inode {
                    id: file_id,
                    pos: map.len(),
                    len,
                });
                for _ in 0..len {
                    map.push(Unit::File(file_id))
                }
                file_id += 1;
            } else {
                // free
                for _ in 0..len {
                    map.push(Unit::Free)
                }
            }
        }
        Self { map, files }
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
    for file in map.files.iter().rev() {
        let frees = map
            .map
            .iter()
            .enumerate()
            .take(file.pos + file.len as usize + 1)
            .filter(|(_i, u)| **u == Unit::Free || **u == Unit::File(file.id))
            .map(|(i, _u)| i)
            .take(file.len as usize)
            .collect_vec();
        if frees[0] >= file.pos {
            continue;
        }
        for j in 0..file.len as usize {
            map.map.swap(frees[j], file.pos + j);
        }
    }
    map.checksum()
}

// PROBLEM 2 solution
fn problem2<T: BufRead>(input: Lines<T>) -> u64 {
    let mut map = DiskMap::from(input);
    // println!("before: {}", map);
    for file in map.files.iter().rev() {
        let free_pos = map
            .map
            .windows(file.len as usize)
            .take(file.pos)
            .enumerate()
            .find(|(_i, u)| {
                u.iter().all(|u| match u {
                    Unit::Free => true,
                    _ => false,
                })
            })
            .map(|(i, _)| i);
        if let Some(free_pos) = free_pos {
            // println!("moving {}@{:?} to {}", file.id, file.pos, free_pos);
            for j in 0..file.len {
                map.map[free_pos + j as usize] = map.map[file.pos + j as usize];
                map.map[file.pos + j as usize] = Unit::Free;
            }
            // println!("after:  {}", map);
        }
    }
    // println!("after:  {}", map);
    map.checksum()
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
        assert_eq!(problem2(c.lines()), 2858);
    }
}
