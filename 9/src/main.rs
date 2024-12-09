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
    frees: Vec<Inode>,
}

impl<T: BufRead> From<Lines<T>> for DiskMap {
    fn from(mut input: Lines<T>) -> Self {
        let line_s = input.next().unwrap().unwrap();
        let line = line_s.as_bytes();
        let mut file_id = 0;
        let mut map = Vec::new();
        let mut files = Vec::new();
        let mut frees = Vec::new();
        for (i, c) in line.iter().enumerate() {
            let len = c - b'0';
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
                frees.push(Inode {
                    id: 0,
                    pos: map.len(),
                    len,
                });
                for _ in 0..len {
                    map.push(Unit::Free)
                }
            }
        }
        Self { map, files, frees }
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
    let mut last_free = 0;
    for file in map.files.iter().rev() {
        let frees = map
            .map
            .iter()
            .enumerate()
            .skip(last_free) // we greedy fill, so no need to check for free space before the last one we used
            .take(file.pos + file.len as usize) // and we only need to search until the end of the current file
            .filter(|(_i, u)| **u == Unit::Free || **u == Unit::File(file.id)) // look for free space or our existing space
            .map(|(i, _u)| i)
            .take(file.len as usize) // get the first file.len free blocks
            .collect_vec();
        // Note: no need to test for too small frees list here, since we are guaranteed at worst to find our current position
        if frees[0] >= file.pos {
            // if the first available free is > file.pos, it's fully packed, job done
            break;
        }
        #[allow(clippy::needless_range_loop)]
        for j in 0..file.len as usize {
            map.map.swap(frees[j], file.pos + j);
        }
        last_free = frees[file.len as usize - 1]
    }
    map.checksum()
}

// PROBLEM 2 solution
fn problem2<T: BufRead>(input: Lines<T>) -> u64 {
    let mut map = DiskMap::from(input);
    for file in map.files.iter().rev() {
        let free = map.frees.iter_mut().find(|inode| inode.len >= file.len); // find the first entry in the free space map large enough
        if let Some(free) = free {
            if free.pos >= file.pos {
                // if it's past our position, continue, but can't break since there might be free space for future files
                continue;
            }
            for j in 0..file.len {
                map.map.swap(free.pos + j as usize, file.pos + j as usize);
            }
            // Note: It is slightly faster to keep these hanging around in the free map with size = 0 then to remove them from the vec
            free.len -= file.len;
            free.pos += file.len as usize;

            map.frees.push(Inode {
                id: 0,
                pos: file.pos,
                len: file.len,
            });
        }
    }
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
