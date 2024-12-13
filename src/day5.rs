use aoc_runner_derive::{aoc, aoc_generator};
use rustc_hash::FxHashMap;
use std::cmp::Ordering;
use std::fmt::Debug;
use std::io::BufRead;

type HashMap<K, V> = FxHashMap<K, V>;

#[aoc_generator(day5)]
pub fn get_input(input: &[u8]) -> (OrderingRules, Vec<Vec<u64>>) {
    let mut lines = input.lines();

    let pairs = HashMap::from_iter(
        lines
            .by_ref()
            .map_while(|l| match l {
                Ok(line) if !line.is_empty() => {
                    let rule = BeforeRule::from(line);
                    Some(vec![
                        ((rule.a, rule.b), Ordering::Less),
                        ((rule.b, rule.a), Ordering::Greater),
                    ])
                }
                _ => None,
            })
            .flatten(),
    );
    let updates: Vec<Vec<u64>> = lines
        .by_ref()
        .map(|l| l.unwrap().split(',').map(|n| n.parse::<u64>().unwrap()).collect())
        .collect();
    (OrderingRules { pairs }, updates)
}

#[derive(Debug)]
struct BeforeRule {
    a: u64,
    b: u64,
}

impl From<String> for BeforeRule {
    fn from(line: String) -> BeforeRule {
        let nums = line.split_once('|').unwrap();
        BeforeRule {
            a: nums.0.parse().unwrap(),
            b: nums.1.parse().unwrap(),
        }
    }
}

#[derive(Debug)]
pub struct OrderingRules {
    pairs: HashMap<(u64, u64), Ordering>,
}

impl OrderingRules {
    fn check(&self, pages: &[u64]) -> bool {
        pages.is_sorted_by(|a, b| self.is_sorted(*a, *b))
    }
    fn cmp(&self, a: u64, b: u64) -> Ordering {
        if let Some(ord) = self.pairs.get(&(a, b)) {
            *ord
        } else {
            Ordering::Equal
        }
    }
    fn is_sorted(&self, a: u64, b: u64) -> bool {
        matches!(self.pairs.get(&(a, b)), Some(Ordering::Less) | Some(Ordering::Equal))
    }
}

// impl<'a, T: Iterator<Item = &'a str>> From<&mut T> for OrderingRules {
//     fn from(input: &mut T) -> Self {
//         let mut rules = Vec::new();
//         for line in input {
//             rules.push(line.into())
//         }
//         Self { rules }
//     }
// }

// PROBLEM 1 solution
#[aoc(day5, part1)]
pub fn part1((rules, updates): &(OrderingRules, Vec<Vec<u64>>)) -> u64 {
    updates
        .iter()
        .filter(|update| rules.check(update))
        .map(|update| update[update.len() / 2])
        .sum()
}

// PROBLEM 2 solution
#[aoc(day5, part2)]
pub fn part2((rules, updates): &(OrderingRules, Vec<Vec<u64>>)) -> u64 {
    let mut updates = updates.clone();
    updates
        .iter_mut()
        .filter(|update| !rules.check(update))
        .map(|update| {
            update.sort_by(|a, b| rules.cmp(*a, *b));
            update[update.len() / 2]
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use crate::day5::*;

    const EXAMPLE: &[u8] = b"47|53
97|13
97|61
97|47
75|29
61|13
75|53
29|13
97|29
53|29
61|53
97|53
61|29
47|13
75|47
97|75
47|61
75|61
47|29
75|13
53|13

75,47,61,53,29
97,61,53,29,13
75,29,13
75,97,47,61,53
61,13,29
97,13,75,29,47";

    #[test]
    fn part1_example() {
        assert_eq!(part1(&get_input(EXAMPLE)), 143);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(&get_input(EXAMPLE)), 123);
    }
}
