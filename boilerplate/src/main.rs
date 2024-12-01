use std::fs::File;
use std::io::{BufRead, BufReader, Lines};
use std::time::Instant;

// BOILERPLATE
type InputIter = Lines<BufReader<File>>;

fn get_input() -> InputIter {
    let f = File::open("input").unwrap();
    let br = BufReader::new(f);
    br.lines()
}

fn main() {
    let start = Instant::now();
    let ans1 = problem1(get_input());
    let duration = start.elapsed();
    println!("Problem 1 solution: {} [{}s]", ans1, duration.as_secs_f64());

    let start = Instant::now();
    let ans2 = problem2(get_input());
    let duration = start.elapsed();
    println!("Problem 2 solution: {} [{}s]", ans2, duration.as_secs_f64());
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
