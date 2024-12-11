use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::time::{Duration, Instant};

use grid::Grid;
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

struct TrailMap {
    map: Grid<u8>,
}

impl<T: BufRead> From<Lines<T>> for TrailMap {
    fn from(input: Lines<T>) -> Self {
        Self { map: input.into() }
    }
}

impl TrailMap {
    fn trailheads(&self) -> Vec<(i64, i64)> {
        self.map
            .data
            .iter()
            .enumerate()
            .filter(|(_, v)| **v == b'0')
            .map(|(i, _v)| self.map.coord(i as i64).unwrap())
            .collect_vec()
    }
    fn count_reachable_from(&self, pos: &(i64, i64), needle: u8, visited: &mut Grid<bool>) -> u64 {
        let our_val = self.map.get(pos.0, pos.1).unwrap();
        if our_val == needle && visited.get(pos.0, pos.1) == Some(false) {
            visited.set(pos.0, pos.1, true);
            return 1;
        } else if our_val == needle {
            return 0;
        }
        // adjacents that are +1
        [(-1, 0), (1, 0), (0, -1), (0, 1)] // left, right, up, down
            .iter()
            .map(|(x_ofs, y_ofs)| (pos.0 + x_ofs, pos.1 + y_ofs)) // get target position
            .map(|(x, y)| ((x, y), self.map.get(x, y))) // get value at that position
            .filter(|(_, val)| *val == Some(our_val + 1)) // only interested if it's our value + 1
            .map(|(pos, _)| pos) // discard the value
            .map(|pos| self.count_reachable_from(&pos, needle, visited))
            .sum()
    }

    fn count_paths_to(&self, pos: &(i64, i64), needle: u8) -> u64 {
        let our_val = self.map.get(pos.0, pos.1).unwrap();
        if our_val == needle {
            return 1;
        }
        [(-1, 0), (1, 0), (0, -1), (0, 1)] // left, right, up, down
            .iter()
            .map(|(x_ofs, y_ofs)| (pos.0 + x_ofs, pos.1 + y_ofs)) // get target position
            .map(|(x, y)| ((x, y), self.map.get(x, y))) // get value at that position
            .filter(|(_, val)| *val == Some(our_val + 1)) // only interested if it's our value + 1
            .map(|(pos, _)| pos) // discard the value
            .map(|mov| self.count_paths_to(&mov, needle))
            .sum::<u64>() as u64
    }
}

// PROBLEM 1 solution

fn problem1<T: BufRead>(input: Lines<T>) -> u64 {
    let map: TrailMap = input.into();

    map.trailheads()
        .iter()
        .map(|pos| {
            let mut visited = Grid::with_shape(map.map.width(), map.map.height(), false);
            map.count_reachable_from(pos, b'9', &mut visited)
        })
        .sum()
}

// PROBLEM 2 solution
fn problem2<T: BufRead>(input: Lines<T>) -> u64 {
    let map: TrailMap = input.into();

    map.trailheads()
        .iter()
        .map(|pos| map.count_paths_to(pos, b'9'))
        .sum::<u64>() as u64
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::io::Cursor;

    const EXAMPLE: &str = &"89010123
78121874
87430965
96549874
45678903
32019012
01329801
10456732";

    #[test]
    fn problem1_example() {
        let c = Cursor::new(EXAMPLE);
        assert_eq!(problem1(c.lines()), 36);
    }

    #[test]
    fn problem2_example() {
        let c = Cursor::new(EXAMPLE);
        assert_eq!(problem2(c.lines()), 81);
    }
}
