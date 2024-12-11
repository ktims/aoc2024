use aoc_runner_derive::{aoc, aoc_generator};
use std::fmt::Debug;
use std::io::BufRead;

#[aoc_generator(day5)]
pub fn get_input(input: &[u8]) -> (OrderingRules, Vec<Vec<u64>>) {
    let mut lines = input.lines();
    let rules = OrderingRules {
        rules: lines
            .by_ref()
            .map_while(|l| match l {
                Ok(line) if !line.is_empty() => Some(Box::new(BeforeRule::from(line)) as _),
                _ => None,
            })
            .collect(),
    };
    let updates: Vec<Vec<u64>> = lines
        .by_ref()
        .map(|l| l.unwrap().split(',').map(|n| n.parse::<u64>().unwrap()).collect())
        .collect();
    (rules, updates)
}

trait Rule: Debug {
    fn check(&self, pages: &[u64]) -> bool;
    fn fail_pos(&self, pages: &[u64]) -> Option<(usize, usize)>;
}

#[derive(Debug)]
struct BeforeRule {
    a: u64,
    b: u64,
}

impl Rule for BeforeRule {
    fn check(&self, pages: &[u64]) -> bool {
        let mut seen_a = false;
        let mut seen_b = false;
        for page in pages {
            if *page == self.a {
                if seen_b {
                    return false;
                }
                seen_a = true;
            } else if *page == self.b {
                if seen_a {
                    return true;
                }
                seen_b = true
            }
        }
        true
    }
    fn fail_pos(&self, pages: &[u64]) -> Option<(usize, usize)> {
        let mut a_pos = None;
        let mut b_pos = None;
        for (pos, page) in pages.iter().enumerate() {
            if *page == self.a {
                if let Some(b_pos) = b_pos {
                    return Some((b_pos, pos));
                }
                a_pos = Some(pos);
            } else if *page == self.b {
                if a_pos.is_some() {
                    return None;
                }
                b_pos = Some(pos);
            }
        }
        None
    }
}

impl From<String> for BeforeRule {
    fn from(line: String) -> BeforeRule {
        let nums: Vec<_> = line.splitn(2, '|').map(|s| s.parse::<u64>().unwrap()).collect();
        BeforeRule { a: nums[0], b: nums[1] }
    }
}

#[derive(Debug)]
pub struct OrderingRules {
    rules: Vec<Box<dyn Rule>>,
}

impl OrderingRules {
    fn check(&self, pages: &[u64]) -> bool {
        self.rules.iter().all(|p| p.check(pages))
    }
    fn fail_pos(&self, pages: &[u64]) -> Option<(usize, usize)> {
        for rule in &self.rules {
            let fail_pos = rule.fail_pos(pages);
            if fail_pos.is_some() {
                return fail_pos;
            }
        }
        None
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
    let mut res = 0;
    for update in updates {
        if rules.check(update) {
            res += update[update.len() / 2]
        }
    }
    res
}

// PROBLEM 2 solution
#[aoc(day5, part2)]
pub fn part2((rules, updates): &(OrderingRules, Vec<Vec<u64>>)) -> u64 {
    let mut updates = updates.clone();
    let mut res = 0;
    for update in updates.as_mut_slice() {
        let mut did_swaps = false;
        while let Some((a, b)) = rules.fail_pos(update) {
            did_swaps = true;
            update.swap(a, b);
        }
        if did_swaps {
            if !rules.check(update) {
                panic!("update still fails after swaps")
            }

            res += update[update.len() / 2];
        }
    }
    res
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
