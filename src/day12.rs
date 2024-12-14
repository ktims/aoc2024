use std::str::FromStr;

use aoc_runner_derive::{aoc, aoc_generator};
use grid::{Coord2d, Grid};

pub struct Farm {
    map: Grid<u8>,
}

impl FromStr for Farm {
    type Err = <Grid<u8> as FromStr>::Err;
    fn from_str(input: &str) -> Result<Self, Self::Err> {
        Ok(Self { map: input.parse()? })
    }
}

impl Farm {
    fn compute_region(&self, pos: &Coord2d, visited: &mut Grid<bool>) -> (u64, u64) {
        let our_plant = self.map.get(pos).unwrap();

        visited.set(pos, true);

        [(-1i64, 0i64), (1, 0), (0, -1), (0, 1)]
            .map(|ofs| pos + ofs)
            .iter()
            .fold((1, 0), |(area, perimeter), adj| {
                match self.map.get(adj) {
                    Some(plant) if plant == our_plant => {
                        if visited.get(adj) == Some(&false) {
                            // add the perimeter of the growth from there if not visited yet
                            let (add_area, add_perimeter) = self.compute_region(adj, visited);
                            (area + add_area, perimeter + add_perimeter)
                        } else {
                            (area, perimeter)
                        }
                    }
                    Some(_) | None => (area, perimeter + 1),
                }
            })
    }

    fn regions_cost(&self) -> u64 {
        let mut visited = Grid::with_shape(self.map.width(), self.map.height(), false);
        let mut cost = 0;
        for y in 0..self.map.height() {
            for x in 0..self.map.width() {
                cost += match visited.get(&(x, y)) {
                    Some(false) => {
                        let (area, perim) = self.compute_region(
                            &Coord2d {
                                x: x as i64,
                                y: y as i64,
                            },
                            &mut visited,
                        );
                        area * perim
                    }
                    Some(_) | None => 0,
                }
            }
        }
        cost
    }
    fn count_corners(&self, pos: &Coord2d) -> u64 {
        // NOTE: Iterating twice is faster than combining conditions in one pass
        // BAB
        // AAA has 4 inside corners (pos at centre). check that for AA A's exist and B doesn't for each rotation
        // BAB                                                      AB
        let our_plant = self.map.get(pos);
        let inside_corners = [(1i64, 1i64), (-1, 1), (1, -1), (-1, -1)]
            .iter()
            .filter(|inside_corner| {
                self.map.get(&(pos + **inside_corner)) != our_plant
                    && self.map.get(&(pos + (inside_corner.0, 0))) == our_plant
                    && self.map.get(&(pos + (0, inside_corner.1))) == our_plant
            })
            .count();
        // BBB
        // BAB has 4 outside corners (pos at centre). check that for AB the  B are both not equal to A for each rot
        // BBB                                                       BB     B
        let outside_corners = [(1i64, 1i64), (-1, 1), (1, -1), (-1, -1)]
            .iter()
            .filter(|outside_corner| {
                self.map.get(&(pos + (outside_corner.0, 0))) != our_plant
                    && self.map.get(&(pos + (0, outside_corner.1))) != our_plant
            })
            .count();
        (inside_corners + outside_corners) as u64
    }
    fn region_corners(&self, pos: &Coord2d, visited: &mut Grid<bool>) -> (u64, u64) {
        let our_plant = self.map.get(pos).unwrap();

        visited.set(pos, true);

        [(-1i64, 0i64), (1, 0), (0, -1), (0, 1)]
            .map(|ofs| pos + ofs)
            .iter()
            .fold((1, self.count_corners(pos)), |(area, corners), adj| {
                match self.map.get(adj) {
                    Some(plant) if plant == our_plant => {
                        if visited.get(adj) == Some(&false) {
                            // add the perimeter of the growth from there if not visited yet
                            let (n_area, n_corners) = self.region_corners(adj, visited);
                            (area + n_area, corners + n_corners)
                        } else {
                            (area, corners)
                        }
                    }
                    Some(_) | None => (area, corners),
                }
            })
    }
    fn regions_discount_cost(&self) -> u64 {
        let mut visited = self.map.same_shape(false);
        let mut cost = 0;
        for y in 0..self.map.height() {
            for x in 0..self.map.width() {
                cost += match visited.get(&(x, y)) {
                    Some(false) => {
                        let (area, corners) = self.region_corners(
                            &Coord2d {
                                x: x as i64,
                                y: y as i64,
                            },
                            &mut visited,
                        );
                        area * corners
                    }
                    Some(_) | None => 0,
                }
            }
        }
        cost
    }
}

fn parse(input: &str) -> Farm {
    input.parse().unwrap()
}

#[aoc(day12, part1)]
pub fn part1(input: &str) -> u64 {
    let farm = parse(input);
    farm.regions_cost()
}

#[aoc(day12, part2)]
pub fn part2(input: &str) -> u64 {
    let farm = parse(input);
    farm.regions_discount_cost()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "RRRRIICCFF
RRRRIICCCF
VVRRRCCFFF
VVRCCCJFFF
VVVVCJJCFE
VVIVCCJJEE
VVIIICJJEE
MIIIIIJJEE
MIIISIJEEE
MMMISSJEEE";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 1930);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 1206);
    }
}
