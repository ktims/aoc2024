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

struct WordSearch {
    rows: Vec<String>,
}

impl<T: BufRead> From<Lines<T>> for WordSearch {
    fn from(input: Lines<T>) -> Self {
        let mut rows = Vec::new();
        for line in input.map(|i| i.unwrap()) {
            rows.push(line);
        }
        Self { rows }
    }
}

impl WordSearch {
    fn count_occurences(haystack: &str, needle: &str) -> u64 {
        let mut count = 0;
        for start in 0..haystack.len() - needle.len() + 1 {
            if &haystack[start..start + needle.len()] == needle {
                count += 1;
            }
        }
        count
    }
    fn count_forward(&self, needle: &str) -> u64 {
        let mut count = 0;
        for row in &self.rows {
            count += Self::count_occurences(row, needle)
        }
        count
    }
    fn count_vertical(&self, needle: &str) -> u64 {
        let mut count = 0;
        for col in 0..self.rows[0].len() {
            let s: String = self.rows.iter().map(|row| row.as_bytes()[col] as char).collect();
            count += Self::count_occurences(&s, needle)
        }
        count
    }
    fn count_diagonal(&self, needle: &str) -> u64 {
        let width = self.rows[0].len();
        let height = self.rows.len();

        let mut count = 0;
        for x in 0..width {
            for y in 0..height {
                // check down-right
                if x <= width - needle.len() && y <= height - needle.len() {
                    if (0..needle.len())
                        .into_iter()
                        .all(|i| self.get(x + i, y + i) == needle.as_bytes()[i].into())
                    {
                        count += 1
                    }
                }
                // check down-left
                if x >= needle.len() - 1 && y <= height - needle.len() {
                    if (0..needle.len())
                        .into_iter()
                        .all(|i| self.get(x - i, y + i) == needle.as_bytes()[i].into())
                    {
                        count += 1
                    }
                }
            }
        }
        count
    }

    fn get(&self, x: usize, y: usize) -> char {
        self.rows[y].as_bytes()[x].into()
    }

    fn count_x_mas(&self) -> u64 {
        // M.M M.S S.M S.S
        // .A. .A. .A. .A.
        // S.S M.S S.M M.M
        let searches: [[char; 5]; 4] =
            ["MMASS", "MSAMS", "SMASM", "SSAMM"].map(|s| s.chars().collect::<Vec<char>>().try_into().unwrap());
        let width = self.rows[0].len();
        let height = self.rows.len();

        let mut count = 0;
        for x in 0..width - 2 {
            for y in 0..height - 2 {
                let s = [
                    self.get(x, y),
                    self.get(x + 2, y),
                    self.get(x + 1, y + 1),
                    self.get(x, y + 2),
                    self.get(x + 2, y + 2),
                ];
                for needle in &searches {
                    if needle == &s {
                        count += 1
                    }
                }
            }
        }
        count
    }
}

// PROBLEM 1 solution

fn problem1<T: BufRead>(input: Lines<T>) -> u64 {
    let needle = "XMAS";
    let needle_rev: String = needle.chars().rev().collect();
    let ws = WordSearch::from(input);
    ws.count_forward(needle)
        + ws.count_forward(&needle_rev)
        + ws.count_vertical(needle)
        + ws.count_vertical(&needle_rev)
        + ws.count_diagonal(needle)
        + ws.count_diagonal(&needle_rev)
}

// PROBLEM 2 solution
fn problem2<T: BufRead>(input: Lines<T>) -> u64 {
    let ws = WordSearch::from(input);
    ws.count_x_mas()
}

#[cfg(test)]
mod tests {
    use crate::*;
    use std::io::Cursor;

    const EXAMPLE: &str = &"MMMSXXMASM
MSAMXMSMSA
AMXSXMAAMM
MSAMASMSMX
XMASAMXAMM
XXAMMXXAMA
SMSMSASXSS
SAXAMASAAA
MAMMMXMMMM
MXMXAXMASX";

    #[test]
    fn problem1_example() {
        let c = Cursor::new(EXAMPLE);
        assert_eq!(problem1(c.lines()), 18);
    }

    #[test]
    fn problem2_example() {
        let c = Cursor::new(EXAMPLE);
        assert_eq!(problem2(c.lines()), 9);
    }
}
