use aoc_runner_derive::aoc;
use cached::proc_macro::cached;
use itertools::Itertools;
use rustc_hash::{FxHashMap, FxHasher};
use std::{
    collections::VecDeque,
    fmt::{Display, Write},
};

#[repr(u8)]
#[derive(Clone, Copy, Debug, Eq, PartialEq, Hash)]
enum Stripe {
    White = b'w',
    Blue = b'u',
    Black = b'b',
    Red = b'r',
    Green = b'g',
}

impl From<&u8> for Stripe {
    fn from(value: &u8) -> Self {
        match value {
            b'w' => Self::White,
            b'u' => Self::Blue,
            b'b' => Self::Black,
            b'r' => Self::Red,
            b'g' => Self::Green,
            _ => unimplemented!(),
        }
    }
}

impl Into<char> for &Stripe {
    fn into(self) -> char {
        let v = *self as u8;
        v.into()
    }
}

impl Display for Stripe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_char(self.into())
    }
}

#[derive(Debug)]
struct Design {
    stripes: Vec<Stripe>,
}

impl From<&[u8]> for Design {
    fn from(input: &[u8]) -> Self {
        let stripes = input.iter().map(|c| c.into()).collect();
        Self { stripes }
    }
}

impl Display for Design {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for stripe in &self.stripes {
            f.write_char(stripe.into())?;
        }
        Ok(())
    }
}

#[derive(Debug)]
struct Onsen {
    towels: Vec<Design>,
    designs: Vec<Design>,
    // ways_cache: FxHashMap<&'a [Stripe], i64>
}

impl Display for Onsen {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.towels.iter().join(", "))?;
        writeln!(f)?;
        writeln!(f)?;
        for d in &self.designs {
            d.fmt(f)?;
            writeln!(f)?;
        }
        Ok(())
    }
}

impl Onsen {
    fn possible(&self, d: &[Stripe]) -> bool {
        if d.len() == 0 {
            return true;
        }
        for t in &self.towels {
            if d.starts_with(&t.stripes) {
                if self.possible(d.split_at(t.stripes.len()).1) {
                    return true;
                }
            }
        }
        false
    }
    // Count the ways to construct a given substring
    fn ways<'a>(&'a self, d: &'a [Stripe], mut cache: FxHashMap<&'a [Stripe], i64>) -> (FxHashMap<&'a [Stripe], i64>, i64) {
        if d.len() == 0 {
            // println!("ways to make {} - 0", d.iter().join(""));
            return (cache, 1);
        }
        if cache.contains_key(d) {
            let val = cache[d];
            return (cache, val);
        }

        let mut count = 0;
        for t in &self.towels {
            if d.starts_with(&t.stripes) {
                let res_count;
                (cache, res_count) = self.ways(&d[t.stripes.len()..d.len()], cache);
                count += res_count;
            }
        }
        // println!("ways to make {} - {}", d.iter().join(""), count);
        cache.insert(d, count);
        (cache, count)
    }
    fn count_possible(&self) -> i64 {
        self.designs
            .iter()
            .map(|d| self.possible(&d.stripes))
            .filter(|p| *p)
            .count() as i64
    }
    fn count_ways(&self) -> i64 {
        self.designs.iter().map(|d| self.ways(&d.stripes, FxHashMap::default()).1).sum::<i64>()
    }
}

fn parse(input: &str) -> Onsen {
    let mut lines = input.lines();

    let towels = lines
        .next()
        .unwrap()
        .split(&[',', ' '])
        .filter(|s| !s.is_empty())
        .map(|s| s.as_bytes().into())
        .collect();
    lines.next().unwrap(); // discard empty line

    let designs = lines.map(|l| l.as_bytes().into()).collect();

    Onsen { towels, designs }
}

#[aoc(day19, part1)]
fn part1(input: &str) -> i64 {
    let onsen = parse(input);

    onsen.count_possible()
}

#[aoc(day19, part2)]
fn part2(input: &str) -> i64 {
    let onsen = parse(input);

    onsen.count_ways()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "r, wr, b, g, bwu, rb, gb, br

brwrr
bggr
gbbr
rrbgbr
ubwu
bwurrg
brgr
bbrgwb";
const EXAMPLE2: &str = "r, wr, b, g, bwu, rb, gb, br

rrbgbr";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 6);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 16);
    }
}
