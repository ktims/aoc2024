use grid::Grid;
use itertools::Itertools;
use std::collections::HashSet;
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

struct AntennaMap {
    map: Grid<u8>,
    antinodes: Grid<u8>,
}

impl<T: BufRead> From<Lines<T>> for AntennaMap {
    fn from(input: Lines<T>) -> Self {
        let map = Grid::from(input);
        let width = map.width();
        let height = map.height();
        Self {
            map,
            antinodes: Grid::with_shape(width, height, b'.'),
        }
    }
}

// PROBLEM 1 solution

fn problem1<T: BufRead>(input: Lines<T>) -> u64 {
    let mut map = AntennaMap::from(input);

    // find the unique frequencies in a dumb way
    let freq_set: HashSet<&u8> = HashSet::from_iter(map.map.data.iter().filter(|c| **c != b'.'));

    // for each unique frequency, get all the pairs' positions
    for freq in freq_set {
        let coords = map
            .map
            .data
            .iter()
            .enumerate()
            .filter(|(_, c)| *c == freq)
            .map(|(i, _)| map.map.coord(i as i64).unwrap())
            .collect_vec();

        for pair in coords.iter().permutations(2).collect_vec() {
            let (a, b) = (pair[0], pair[1]);
            let node_pos = (a.0 + a.0 - b.0, a.1 + a.1 - b.1);
            map.antinodes.set(node_pos.0, node_pos.1, b'#');
        }
    }
    map.antinodes.count(b'#') as u64
}

// PROBLEM 2 solution
fn problem2<T: BufRead>(input: Lines<T>) -> u64 {
    let mut map = AntennaMap::from(input);

    // find the unique frequencies in a dumb way
    let freq_set: HashSet<&u8> = HashSet::from_iter(map.map.data.iter().filter(|c| **c != b'.'));

    // for each unique frequency, get all the pairs' positions
    for freq in freq_set {
        let coords = map
            .map
            .data
            .iter()
            .enumerate()
            .filter(|(_, c)| *c == freq)
            .map(|(i, _)| map.map.coord(i as i64).unwrap())
            .collect_vec();

        for pair in coords.iter().permutations(2).collect_vec() {
            let (a, b) = (pair[0], pair[1]);
            let offset = (a.0 - b.0, a.1 - b.1);
            let mut i = 0;
            loop {
                let node_pos = (a.0 + i * offset.0, a.1 + i * offset.1);
                if !map.antinodes.set(node_pos.0, node_pos.1, b'#') {
                    break;
                }
                i += 1;
            }
        }
    }
    map.antinodes.count(b'#') as u64
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::io::Cursor;

    const EXAMPLE: &str = &"............
........0...
.....0......
.......0....
....0.......
......A.....
............
............
........A...
.........A..
............
............";

    #[test]
    fn problem1_example() {
        let c = Cursor::new(EXAMPLE);
        assert_eq!(problem1(c.lines()), 14);
    }

    #[test]
    fn problem2_example() {
        let c = Cursor::new(EXAMPLE);
        assert_eq!(problem2(c.lines()), 34);
    }
}
