use aoc_runner_derive::aoc;
use itertools::Itertools;

enum LockKey {
    Lock,
    Key,
}

#[derive(Clone, Debug)]
struct LockPile {
    keys: Vec<Vec<usize>>,
    locks: Vec<Vec<usize>>,
}

fn parse_grid(lines: &Vec<&str>) -> (LockKey, Vec<usize>) {
    assert_eq!(lines.len(), 7);
    if lines[0].chars().all(|c| c == '#') {
        // lock
        let mut pins = vec![0; 5];
        for row in 1..lines.len() {
            let row_s = lines[row];
            for i in 0..row_s.len() {
                if row_s.chars().nth(i) == Some('#') {
                    pins[i] = row
                }
            }
        }
        (LockKey::Lock, pins)
    } else if lines[6].chars().all(|c| c == '#') {
        // key
        let mut pins = vec![5; 5];
        for row in (1..lines.len()).rev() {
            let row_s = lines[row];
            for i in 0..row_s.len() {
                if row_s.chars().nth(i) == Some('#') {
                    pins[i] = 6 - row
                }
            }
        }
        (LockKey::Key, pins)
    } else {
        panic!("not a lock or a key: {:?}", lines);
    }
}

fn parse(input: &str) -> LockPile {
    let mut locks = Vec::new();
    let mut keys = Vec::new();
    let mut accum: Vec<&str> = Vec::new();
    for line in input.lines() {
        if line == "" {
            let (lk, pins) = parse_grid(&accum);
            match lk {
                LockKey::Lock => locks.push(pins),
                LockKey::Key => keys.push(pins),
            }
            accum.clear();
        } else {
            accum.push(line);
        }
    }
    if accum.len() != 0 {
        let (lk, pins) = parse_grid(&accum);
        match lk {
            LockKey::Lock => locks.push(pins),
            LockKey::Key => keys.push(pins),
        }
    }
    LockPile { keys, locks }
}

fn test_lock_key(lock: &Vec<usize>, key: &Vec<usize>) -> bool {
    !lock.iter().zip(key.iter()).any(|(lp, kp)| lp + kp > 5)
}

#[aoc(day25, part1)]
pub fn part1(input: &str) -> i64 {
    let lockpile = parse(input);

    lockpile
        .locks
        .iter()
        .cartesian_product(lockpile.keys.iter())
        .filter(|(l, k)| test_lock_key(l, k))
        .count() as i64
}

#[aoc(day25, part2)]
pub fn part2(_input: &str) -> String {
    "run the other solutions for day 25 part 2!".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "#####
.####
.####
.####
.#.#.
.#...
.....

#####
##.##
.#.##
...##
...#.
...#.
.....

.....
#....
#....
#...#
#.#.#
#.###
#####

.....
.....
#.#..
###..
###.#
###.#
#####

.....
.....
.....
#....
#.#..
#.#.#
#####";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 3);
    }

    #[test]
    fn part2_example() {}
}
