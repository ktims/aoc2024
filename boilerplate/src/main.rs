use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::time::{Duration, Instant};

// BOILERPLATE
type InputIter = Lines<BufReader<File>>;

fn duration_format(duration: &Duration) -> String {
    match duration.as_secs_f64() {
        x if x > 1.0 => format!("{:.3}s", x),
        x if x > 0.010 => format!("{:.3}ms", x * 1e3),
        x => format!("{:.3}us", x * 1e6),
    }
}

fn main() {
    let start = Instant::now();
    let ans1 = problem1(get_input());
    let duration = start.elapsed();
    println!("Problem 1 solution: {} [{}]", ans1, duration_format(&duration));

    let start = Instant::now();
    let ans2 = problem2(get_input());
    let duration = start.elapsed();
    println!("Problem 2 solution: {} [{}]", ans2, duration_format(&duration));
}

// PROBLEM 1 solution

fn problem1<T: BufRead>(input: Lines<T>) -> u64 {
    0
}

// PROBLEM 2 solution
fn problem2<T: BufRead>(input: Lines<T>) -> u64 {
    0
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::io::Cursor;

    const EXAMPLE: &str = &"";

    #[test]
    fn problem1_example() {
        let c = Cursor::new(EXAMPLE);
        assert_eq!(problem1(c.lines()), 0);
    }

    #[test]
    fn problem2_example() {
        let c = Cursor::new(EXAMPLE);
        assert_eq!(problem2(c.lines()), 0);
    }
}
