use aoc_runner_derive::aoc;
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelIterator};

#[derive(Debug, Eq, PartialEq, PartialOrd, Ord)]
struct Change {
    price: i8,
    delta: i8,
}

fn evolve_secret(mut n: i64) -> i64 {
    n = ((n * 64) ^ n) % 16777216;
    n = ((n / 32) ^ n) % 16777216;
    n = ((n * 2048) ^ n) % 16777216;
    n
}

fn rounds(mut secret: i64, n: i64) -> i64 {
    for _ in 0..n {
        secret = evolve_secret(secret)
    }
    secret
}

fn prices(mut secret: i64, n: i64) -> Vec<i8> {
    let mut prices = vec![(secret % 10) as i8];
    for _ in 1..n {
        secret = evolve_secret(secret);
        prices.push((secret % 10) as i8);
    }
    prices
}

fn changes(prices: &[i8]) -> Vec<Change> {
    prices
        .windows(2)
        .map(|a| Change {
            price: a[1],
            delta: a[1] - a[0],
        })
        .collect()
}

fn profit_for_sequence(changes: &Vec<Vec<Change>>, seq: &[i8]) -> i64 {
    changes
        .par_iter()
        .filter_map(|inner| {
            inner
                .windows(seq.len())
                .find(|window| window.iter().zip(seq).all(|(w, z)| w.delta == *z))
                .map(|buy| buy[seq.len() - 1].price as i64)
        })
        .sum()
}

fn find_best_sequence(changes: &Vec<Vec<Change>>) -> [i8; 4] {
    let mut best_seq = [0, 0, 0, 0];
    let mut best_profit = 0;
    for seq in (0..4).map(|_| (-9..=9i8)).multi_cartesian_product() {
        let profit = profit_for_sequence(changes, &seq);
        if profit > best_profit {
            best_seq = seq.try_into().unwrap();
            best_profit = profit;
        }
    }
    best_seq
}

fn parse(input: &str) -> Vec<i64> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day22, part1)]
pub fn part1(input: &str) -> i64 {
    let secrets = parse(input);

    secrets.iter().map(|s| rounds(*s, 2000)).sum::<i64>()
}

#[aoc(day22, part2)]
pub fn part2(input: &str) -> i64 {
    let secrets = parse(input);

    let price_changes = secrets.iter().map(|s| changes(&prices(*s, 2000))).collect_vec();

    let seq = find_best_sequence(&price_changes);
    println!("found best seq: {:?}", seq);
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
            profit_for_sequence(&vec![changes(&prices(123, 10))], &[-1, -1, 0, 2]),
            6
        );
        let secrets = parse(EXAMPLE2);

        let price_changes = secrets.iter().map(|s| changes(&prices(*s, 2000))).collect_vec();
        assert_eq!(profit_for_sequence(&price_changes, &[-2, 1, -1, 3]), 23);
    }

    #[test]
    fn test_changes() {
        let changes = changes(&prices(123, 10));
        assert_eq!(
            changes.iter().map(|c| c.delta).collect_vec(),
            vec![-3, 6, -1, -1, 0, 2, -2, 0, -2]
        );
        assert_eq!(
            changes.iter().map(|c| c.price).collect_vec(),
            vec![0, 6, 5, 4, 4, 6, 4, 4, 2]
        );
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
