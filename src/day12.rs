use std::io::BufRead;

use aoc_runner_derive::{aoc, aoc_generator};
use grid::{Coord2d, Grid};

pub struct Farm {
    map: Grid<u8>,
}

impl From<&[u8]> for Farm {
    fn from(input: &[u8]) -> Self {
        Self {
            map: Grid::from(input.lines()),
        }
    }
}

impl Farm {
    fn compute_region(&self, pos: &Coord2d, visited: &mut Grid<bool>) -> (u64, u64) {
        let our_plant = self.map.get(pos).unwrap();
        let mut perimeter = 0;
        let mut area = 1;

        visited.set(pos, true);

        for adj in [(-1i64, 0i64), (1, 0), (0, -1), (0, 1)].map(|ofs| pos + ofs) {
            match self.map.get(&adj) {
                Some(plant) if plant == our_plant => {
                    if visited.get(&adj) == Some(false) {
                        // add the perimeter of the growth from there if not visited yet
                        let (n_area, n_perimeter) = self.compute_region(&adj, visited);
                        area += n_area;
                        perimeter += n_perimeter;
                    }
                }
                Some(_) | None => perimeter += 1,
            }
        }
        (area, perimeter)
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
        //  A
        // AAA has 4 inside corners (pos at centre). check that  AA A's exist and B doesn't for each rotation
        //  A                                                    AB
        let our_plant = self.map.get(pos);
        let inside_corners = [(1i64, 1i64), (-1, 1), (1, -1), (-1, -1)]
            .iter()
            .filter(|inside_corner| {
                self.map.get(&(pos + **inside_corner)) != our_plant
                    && self.map.get(&(pos + (inside_corner.0, 0))) == our_plant
                    && self.map.get(&(pos + (0, inside_corner.1))) == our_plant
            })
            .count();
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
        let mut area = 1;
        let mut corners = self.count_corners(pos);

        visited.set(pos, true);

        for adj in [(-1i64, 0i64), (1, 0), (0, -1), (0, 1)].map(|ofs| pos + ofs) {
            match self.map.get(&adj) {
                Some(plant) if plant == our_plant => {
                    if visited.get(&adj) == Some(false) {
                        // add the perimeter of the growth from there if not visited yet
                        let (n_area, n_corners) = self.region_corners(&adj, visited);
                        area += n_area;
                        corners += n_corners;
                    }
                }
                Some(_) | None => {}
            }
        }
        (area, corners)
    }
    fn regions_discount_cost(&self) -> u64 {
        let mut visited = Grid::with_shape(self.map.width(), self.map.height(), false);
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

#[aoc_generator(day12)]
fn parse(input: &[u8]) -> Farm {
    input.into()
}

#[aoc(day12, part1)]
pub fn part1(farm: &Farm) -> u64 {
    farm.regions_cost()
}

#[aoc(day12, part2)]
pub fn part2(farm: &Farm) -> u64 {
    farm.regions_discount_cost()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &[u8] = b"RRRRIICCFF
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
        assert_eq!(part1(&parse(EXAMPLE)), 1930);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&parse(EXAMPLE)), 1206);
    }
}
