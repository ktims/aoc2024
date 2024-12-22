use aoc_runner_derive::{aoc, aoc_generator};

fn evolve_secret(mut n: i64) -> i64 {
    n = ((n * 64) ^ n) % 16777216;
    n = ((n / 32) ^ n) % 16777216;
    n = ((n * 2048) ^ n)  % 16777216;
    n
}

fn rounds(mut secret: i64, n: i64) -> i64 {
    for _ in 0..n {
        secret = evolve_secret(secret)
    }
    secret
}

fn parse(input: &str) -> Vec<i64> {
    input.lines().map(|l| l.parse().unwrap()).collect()
}

#[aoc(day22, part1)]
fn part1(input: &str) -> i64 {
    let secrets = parse(input);

    secrets.iter().map(|s| rounds(*s, 2000)).sum::<i64>()
}

#[aoc(day22, part2)]
fn part2(input: &str) -> i64 {
    todo!()
}


#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "1
10
100
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
    fn part1_example() {
        assert_eq!(part1(EXAMPLE),37327623);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 0);
    }
}