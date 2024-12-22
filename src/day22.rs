use aoc_runner_derive::aoc;
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};
use rustc_hash::FxHashMap;

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord, Clone, Copy)]
struct Change {
    price: i8,
    delta: i8,
}

type Secret = u64;

fn evolve_secret(mut n: Secret) -> Secret {
    n = ((n * 64) ^ n) % 16777216;
    n = ((n / 32) ^ n) % 16777216;
    n = ((n * 2048) ^ n) % 16777216;
    n
}

fn rounds(mut secret: Secret, n: Secret) -> Secret {
    for _ in 0..n {
        secret = evolve_secret(secret)
    }
    secret
}

fn prices(mut secret: Secret, n: usize) -> Vec<i8> {
    let mut prices = vec![(secret % 10) as i8];
    for _ in 1..n {
        secret = evolve_secret(secret);
        prices.push((secret % 10) as i8);
    }
    prices
}

fn build_profit_map(prices: &[i8]) -> FxHashMap<[i8; 4], i8> {
    let mut profits = FxHashMap::default();
    let changes = prices
        .windows(2)
        .map(|a| Change {
            price: a[1],
            delta: a[1] - a[0],
        })
        .collect_vec();
    for i in 3..changes.len() {
        let seq: [i8; 4] = changes[i - 3..=i]
            .iter()
            .map(|c| c.delta)
            .collect_vec()
            .try_into()
            .unwrap();
        profits.entry(seq).or_insert(changes[i].price);
    }
    profits
}

fn profit_for_sequence(changes: &[FxHashMap<[i8; 4], i8>], seq: &[i8]) -> i64 {
    changes
        .iter()
        .filter_map(|inner| inner.get(seq).map(|v| *v as i64))
        .sum()
}

fn find_best_sequence(changes: &[FxHashMap<[i8; 4], i8>]) -> [i8; 4] {
    let possible_seqs = (0..4).map(|_| (-9..=9i8)).multi_cartesian_product().collect_vec();
    let (best_seq, _best_profit) = possible_seqs
        .par_iter()
        .map_with(changes, |changes, seq| (seq, profit_for_sequence(changes, seq)))
        .max_by(|(_, a), (_, b)| a.cmp(b))
        .unwrap();

    best_seq.as_slice().try_into().unwrap()
}

fn parse(input: &str) -> Vec<Secret> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day22, part1)]
pub fn part1(input: &str) -> Secret {
    let secrets = parse(input);

    secrets.iter().map(|s| rounds(*s, 2000)).sum::<Secret>()
}

#[aoc(day22, part2)]
pub fn part2(input: &str) -> i64 {
    let secrets = parse(input);

    let price_changes = secrets
        .iter()
        .map(|s| build_profit_map(&prices(*s, 2000)))
        .collect_vec();

    let seq = find_best_sequence(&price_changes);
    profit_for_sequence(&price_changes, &seq)
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "1
10
100
2024";

    const EXAMPLE2: &str = "1
2
3
2024";

    #[test]
    fn evolution() {
        assert_eq!(evolve_secret(123), 15887950);
        assert_eq!(evolve_secret(15887950), 16495136);
        assert_eq!(evolve_secret(16495136), 527345);
    }

    #[test]
    fn test_rounds() {
        assert_eq!(rounds(1, 2000), 8685429);
        assert_eq!(rounds(10, 2000), 4700978);
    }

    #[test]
    fn test_prices() {
        assert_eq!(prices(123, 10), vec![3, 0, 6, 5, 4, 4, 6, 4, 4, 2]);
    }

    #[test]
    fn test_profit() {
        assert_eq!(
            profit_for_sequence(&vec![build_profit_map(&prices(123, 10))], &[-1, -1, 0, 2]),
            6
        );
        let secrets = parse(EXAMPLE2);

        let price_changes = secrets
            .iter()
            .map(|s| build_profit_map(&prices(*s, 2000)))
            .collect_vec();
        assert_eq!(profit_for_sequence(&price_changes, &[-2, 1, -1, 3]), 23);
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 37327623);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE2), 23);
    }
}
