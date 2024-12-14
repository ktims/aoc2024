use aoc_runner_derive::aoc;
use itertools::Itertools;
use regex::Regex;
use std::str::Lines;

#[derive(Debug, Clone)]
pub struct MachineAction(i64, i64);

#[derive(Debug, Clone)]
struct ClawMachine {
    button_a: MachineAction,
    button_b: MachineAction,
    prize: (i64, i64),
}

impl ClawMachine {
    fn consume_from_input(input: &mut Lines) -> Option<Self> {
        // consume any empty lines at the front
        let ofs_re = Regex::new(r"X([+-]\d+), Y([+-]\d+)").unwrap();
        let prize_re = Regex::new(r"X=(\d+), Y=(\d+)").unwrap();
        // consume 3 lines - a, b, prize
        if let Some((a_line, b_line, prize_line)) = input.filter(|l| !l.is_empty()).take(3).collect_tuple() {
            let a_caps = ofs_re.captures(&a_line).unwrap();
            let b_caps = ofs_re.captures(&b_line).unwrap();
            let prize_caps = prize_re.captures(&prize_line).unwrap();
            let button_a = MachineAction(
                a_caps.get(1).unwrap().as_str().parse().unwrap(),
                a_caps.get(2).unwrap().as_str().parse().unwrap(),
            );
            let button_b = MachineAction(
                b_caps.get(1).unwrap().as_str().parse().unwrap(),
                b_caps.get(2).unwrap().as_str().parse().unwrap(),
            );
            let prize = (
                prize_caps.get(1).unwrap().as_str().parse().unwrap(),
                prize_caps.get(2).unwrap().as_str().parse().unwrap(),
            );
            Some(Self {
                button_a,
                button_b,
                prize,
            })
        } else {
            None
        }
    }
    fn cost(moves: (i64, i64)) -> i64 {
        moves.0 * 3 + moves.1
    }
    fn cheapest_prize(&self) -> Option<i64> {
        let remainder_a = (self.button_b.0 * self.prize.1 - self.button_b.1 * self.prize.0)
            % (self.button_b.0 * self.button_a.1 - self.button_a.0 * self.button_b.1);
        let a = (self.button_b.0 * self.prize.1 - self.button_b.1 * self.prize.0)
            / (self.button_b.0 * self.button_a.1 - self.button_a.0 * self.button_b.1);
        let remainder_b = (self.prize.0 - (self.button_a.0 * a)) % self.button_b.0;
        let b = (self.prize.0 - (self.button_a.0 * a)) / self.button_b.0;
        if remainder_a == 0 && remainder_b == 0 {
            Some(Self::cost((a, b)))
        } else {
            None
        }
    }
    fn offset(&mut self, offset: i64) {
        self.prize = (self.prize.0 + offset, self.prize.1 + offset)
    }
}

#[derive(Debug, Clone)]
struct ClawMachines {
    machines: Vec<ClawMachine>,
}

impl From<&str> for ClawMachines {
    fn from(input: &str) -> Self {
        let mut machines = Vec::new();
        let mut lines = input.lines();
        while let Some(machine) = ClawMachine::consume_from_input(&mut lines) {
            machines.push(machine);
        }
        Self { machines }
    }
}

fn parse(input: &str) -> ClawMachines {
    ClawMachines::from(input)
}

#[aoc(day13, part1)]
fn part1(input: &str) -> i64 {
    let machines = parse(input);
    machines.machines.iter().filter_map(|m| m.cheapest_prize()).sum()
}

#[aoc(day13, part2)]
fn part2(input: &str) -> i64 {
    let mut machines = parse(input);
    machines
        .machines
        .iter_mut()
        .filter_map(|m| {
            m.offset(10000000000000);
            m.cheapest_prize()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "Button A: X+94, Y+34
Button B: X+22, Y+67
Prize: X=8400, Y=5400

Button A: X+26, Y+66
Button B: X+67, Y+21
Prize: X=12748, Y=12176

Button A: X+17, Y+86
Button B: X+84, Y+37
Prize: X=7870, Y=6450

Button A: X+69, Y+23
Button B: X+27, Y+71
Prize: X=18641, Y=10279";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 480);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 875318608908);
    }
}
