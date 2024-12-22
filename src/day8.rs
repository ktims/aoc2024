use aoc_runner_derive::{aoc, aoc_generator};
use grid::Grid;
use itertools::Itertools;
use rustc_hash::FxHashSet;
use std::io::BufRead;

type HashSet<T> = FxHashSet<T>;

#[aoc_generator(day8)]
pub fn get_input(input: &[u8]) -> AntennaMap {
    AntennaMap::from(input)
}

pub struct AntennaMap {
    map: Grid<u8>,
}

impl<T: BufRead> From<T> for AntennaMap {
    fn from(input: T) -> Self {
        Self { map: Grid::from(input) }
    }
}

impl AntennaMap {
    fn find_antinodes(&self, start: usize, reps: Option<usize>) -> Grid<bool> {
        let mut antinodes = Grid::with_shape(self.map.width(), self.map.height(), false);
        // find the unique frequencies in a dumb way
        // NOTE: The dumb way is faster than the slightly-smarter ways I tried
        let freq_set: HashSet<&u8> = HashSet::from_iter(self.map.data.iter().filter(|c| **c != b'.'));

        // for each unique frequency, get all the pairs' positions
        for freq in freq_set {
            for pair in self
                .map
                .data
                .iter()
                .enumerate()
                .filter(|(_, c)| *c == freq)
                .map(|(i, _)| self.map.coord(i as i64).unwrap())
                .permutations(2)
            {
                // permutations generates both pairs, ie. ((1,2),(2,1)) and ((2,1),(1,2)) so we don't need
                // to consider the 'negative' side of the line, which will be generated by the other pair
                let (a, b) = (pair[0], pair[1]);
                let offset = (a.x - b.x, a.y - b.y);
                for i in (start..).map_while(|i| if Some(i - start) != reps { Some(i as i64) } else { None }) {
                    let node_pos = (a.x + i * offset.0, a.y + i * offset.1);
                    if antinodes.set(&node_pos, true).is_none() {
                        // left the grid
                        break;
                    }
                }
            }
        }
        antinodes
    }
}

// PROBLEM 1 solution
#[aoc(day8, part1)]
pub fn part1(map: &AntennaMap) -> u64 {
    let antinodes = map.find_antinodes(1, Some(1));
    antinodes.count(&true) as u64
}

// PROBLEM 2 solution
#[aoc(day8, part2)]
pub fn part2(map: &AntennaMap) -> u64 {
    let antinodes = map.find_antinodes(0, None);
    antinodes.count(&true) as u64
}

#[cfg(test)]
mod tests {
    use crate::day8::*;

    const EXAMPLE: &[u8] = b"............
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
    fn part1_example() {
        assert_eq!(part1(&get_input(EXAMPLE)), 14);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&get_input(EXAMPLE)), 34);
    }
}
