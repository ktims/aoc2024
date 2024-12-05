use std::fmt::Debug;
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

trait Rule: Debug {
    fn check(&self, pages: &Vec<u64>) -> bool;
    fn fail_pos(&self, pages: &Vec<u64>) -> Option<(usize, usize)>;
}

#[derive(Debug)]
struct BeforeRule {
    a: u64,
    b: u64,
}

impl Rule for BeforeRule {
    fn check(&self, pages: &Vec<u64>) -> bool {
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
        return true;
    }
    fn fail_pos(&self, pages: &Vec<u64>) -> Option<(usize, usize)> {
        let mut a_pos = None;
        let mut b_pos = None;
        for (pos, page) in pages.iter().enumerate() {
            if *page == self.a {
                if b_pos.is_some() {
                    return Some((b_pos.unwrap(), pos));
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
struct OrderingRules {
    rules: Vec<Box<dyn Rule>>,
}

impl OrderingRules {
    fn check(&self, pages: &Vec<u64>) -> bool {
        self.rules.iter().all(|p| p.check(pages))
    }
    fn fail_pos(&self, pages: &Vec<u64>) -> Option<(usize, usize)> {
        for rule in &self.rules {
            let fail_pos = rule.fail_pos(pages);
            if fail_pos.is_some() {
                return fail_pos
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

fn problem1<T: BufRead>(mut input: Lines<T>) -> u64 {
    let rules = OrderingRules {
        rules: input
            .by_ref()
            .map_while(|l| match l {
                Ok(line) if line != "" => Some(Box::new(BeforeRule::from(line)) as _),
                _ => None,
            })
            .collect(),
    };
    let updates: Vec<Vec<u64>> = input
        .by_ref()
        .map(|l| l.unwrap().split(',').map(|n| n.parse::<u64>().unwrap()).collect())
        .collect();

    let mut res = 0;
    for update in updates {
        if rules.check(&update) {
            res += update[update.len() / 2]
        }
    }
    res
}

// PROBLEM 2 solution
fn problem2<T: BufRead>(mut input: Lines<T>) -> u64 {
    let rules = OrderingRules {
        rules: input
            .by_ref()
            .map_while(|l| match l {
                Ok(line) if line != "" => Some(Box::new(BeforeRule::from(line)) as _),
                _ => None,
            })
            .collect(),
    };
    let updates: Vec<Vec<u64>> = input
        .by_ref()
        .map(|l| l.unwrap().split(',').map(|n| n.parse::<u64>().unwrap()).collect())
        .collect();

    let mut res = 0;
    for mut update in updates {
        let mut did_swaps = false;
        while let Some((a,b)) = rules.fail_pos(&update) {
            did_swaps = true;
            update.swap(a, b);
        }
        if did_swaps {
            if !rules.check(&update) {
                panic!("update still fails after swaps")
            }

            res += update[update.len() / 2];
        }
    }
    res
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::io::Cursor;

    const EXAMPLE: &str = &"47|53
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
    fn problem1_example() {
        let c = Cursor::new(EXAMPLE);
        assert_eq!(problem1(c.lines()), 143);
    }

    #[test]
    fn problem2_example() {
        let c = Cursor::new(EXAMPLE);
        assert_eq!(problem2(c.lines()), 123);
    }
}
