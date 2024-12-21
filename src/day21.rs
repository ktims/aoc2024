use aoc_runner_derive::aoc;
use itertools::Itertools;
use std::iter::repeat_n;

trait KeypadRobot {
    fn new() -> Self;
    fn press(&mut self, target: u8) -> Vec<Vec<u8>>;
}

#[derive(Clone, Copy, Debug)]
struct NumberKeypadRobot {
    pointing_at: u8,
}

impl NumberKeypadRobot {
    fn pos_of(button: u8) -> (i8, i8) {
        match button {
            b'7' => (0, 0),
            b'8' => (1, 0),
            b'9' => (2, 0),
            b'4' => (0, 1),
            b'5' => (1, 1),
            b'6' => (2, 1),
            b'1' => (0, 2),
            b'2' => (1, 2),
            b'3' => (2, 2),
            b'X' => (0, 3),
            b'0' => (1, 3),
            b'A' => (2, 3),
            c => unimplemented!("unexpected character {}", c),
        }
    }
}
impl KeypadRobot for NumberKeypadRobot {
    fn new() -> Self {
        Self { pointing_at: b'A' }
    }
    fn press(&mut self, target: u8) -> Vec<Vec<u8>> {
        let cur_pos = Self::pos_of(self.pointing_at);
        let goal_pos = Self::pos_of(target);
        let x_ofs = goal_pos.0 - cur_pos.0;
        let y_ofs = goal_pos.1 - cur_pos.1;

        let mut paths = Vec::new();

        if (cur_pos.0 + x_ofs, cur_pos.1) != Self::pos_of(b'X') {
            let mut x_first = Vec::new();
            x_first.extend(repeat_n(if x_ofs > 0 { b'>' } else { b'<' }, x_ofs.abs() as usize));
            x_first.extend(repeat_n(if y_ofs > 0 { b'v' } else { b'^' }, y_ofs.abs() as usize));
            x_first.push(b'A');
            paths.push(x_first);
        }
        if (cur_pos.0, cur_pos.1 + y_ofs) != Self::pos_of(b'X') {
            let mut y_first = Vec::new();
            y_first.extend(repeat_n(if y_ofs > 0 { b'v' } else { b'^' }, y_ofs.abs() as usize));
            y_first.extend(repeat_n(if x_ofs > 0 { b'>' } else { b'<' }, x_ofs.abs() as usize));
            y_first.push(b'A');
            paths.push(y_first);
        }
        if paths.is_empty() {
            panic!("all paths lead to the void");
        }
        paths.dedup();
        self.pointing_at = target;
        paths
    }
}

#[derive(Clone, Copy, Debug)]
struct DirectionKeypadRobot<T: KeypadRobot> {
    pointing_at: u8,
    child: Option<T>,
}

impl<T: KeypadRobot> DirectionKeypadRobot<T> {
    fn pos_of(target: u8) -> (i8, i8) {
        match target {
            b'X' => (0, 0),
            b'^' => (1, 0),
            b'A' => (2, 0),
            b'<' => (0, 1),
            b'v' => (1, 1),
            b'>' => (2, 1),
            c => unimplemented!("unexpected char {}", c),
        }
    }
    fn move_to(&mut self, target: u8) -> Vec<u8> {
        let cur_pos = Self::pos_of(self.pointing_at);
        let goal_pos = Self::pos_of(target);
        let x_ofs = goal_pos.0 - cur_pos.0;
        let y_ofs = goal_pos.1 - cur_pos.1;

        self.pointing_at = target;

        if (cur_pos.0 + x_ofs, cur_pos.1) != Self::pos_of(b'X') {
            let mut x_first = Vec::new();
            x_first.extend(repeat_n(if x_ofs > 0 { b'>' } else { b'<' }, x_ofs.abs() as usize));
            x_first.extend(repeat_n(if y_ofs > 0 { b'v' } else { b'^' }, y_ofs.abs() as usize));
            x_first.push(b'A');
            return x_first;
        }
        if (cur_pos.0, cur_pos.1 + y_ofs) != Self::pos_of(b'X') {
            let mut y_first = Vec::new();
            y_first.extend(repeat_n(if y_ofs > 0 { b'v' } else { b'^' }, y_ofs.abs() as usize));
            y_first.extend(repeat_n(if x_ofs > 0 { b'>' } else { b'<' }, x_ofs.abs() as usize));
            y_first.push(b'A');
            return y_first;
        }
        panic!("all routes lead to the void");
    }
    fn path_to(&mut self, moves: &Vec<u8>) -> Vec<u8> {
        let prev_point = self.pointing_at;
        let mut path = Vec::new();
        for m in moves {
            path.append(&mut self.move_to(*m));
        }
        self.pointing_at = prev_point;
        path
    }
}
impl<T: KeypadRobot> KeypadRobot for DirectionKeypadRobot<T> {
    fn new() -> Self {
        Self {
            pointing_at: b'A',
            child: None,
        }
    }
    fn press(&mut self, target: u8) -> Vec<Vec<u8>> {
        let path_options = self.child.as_mut().unwrap().press(target);
        // for each path option, find our shortest route
        let mut candidate_paths = Vec::new();
        for child_path in path_options {
            let candidate_path = self.path_to(&child_path);
            candidate_paths.push(candidate_path);
        }
        candidate_paths
    }
}

struct Code(Vec<u8>);

impl Code {
    fn num_val(&self) -> i64 {
        String::from_utf8_lossy(&self.0.as_slice()[0..3]).parse().unwrap()
    }
}

fn parse(input: &str) -> Vec<Code> {
    let mut codes = Vec::new();
    for code in input.lines() {
        codes.push(Code(code.as_bytes().to_vec()))
    }
    codes
}

#[aoc(day21, part1)]
fn part1(input: &str) -> i64 {
    let codes = parse(input);
    let mut sum = 0;
    for code in &codes {
        let numpad = NumberKeypadRobot::new();
        let mut robot1 = DirectionKeypadRobot::new();
        robot1.child = Some(numpad);

        let mut robot2 = DirectionKeypadRobot::new();
        robot2.child = Some(robot1);

        let mut path = Vec::new();
        for button in &code.0 {
            let paths = robot2.press(*button);
            path.push(paths);
        }
        let paths = path.clone().into_iter()
            .multi_cartesian_product()
            .map(|c| c.iter().flatten().map(|c| *c as char).join("")).collect_vec();
        let best = paths.iter().map(|p| p.len()).min().unwrap() as i64;
        let score = code.num_val() * best;
        sum += score;
    }
    sum
}

#[aoc(day21, part2)]
fn part2(input: &str) -> i64 {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "029A
980A
179A
456A
379A";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 126384);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 0);
    }
}
