use aoc_runner_derive::aoc;
use rustc_hash::FxHashMap;
use std::iter::repeat_n;

#[derive(Clone, Debug)]
enum KeypadRobot {
    Number(NumberKeypadRobot),
    Direction(DirectionKeypadRobot),
}

impl KeypadRobot {
    fn press(&mut self, target: u8) -> Vec<FxHashMap<Vec<u8>, usize>> {
        match self {
            Self::Number(r) => r.press(target),
            Self::Direction(r) => r.press(target),
        }
    }
}

#[derive(Clone, Debug)]
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
impl NumberKeypadRobot {
    fn new() -> Self {
        Self { pointing_at: b'A' }
    }
    fn press(&mut self, target: u8) -> Vec<FxHashMap<Vec<u8>, usize>> {
        let cur_pos = Self::pos_of(self.pointing_at);
        let goal_pos = Self::pos_of(target);
        let x_ofs = goal_pos.0 - cur_pos.0;
        let y_ofs = goal_pos.1 - cur_pos.1;

        let mut paths = Vec::new();
        // NOTE: no need to consider zig-zags since those paths will always require more button presses going back and forth
        if (cur_pos.0 + x_ofs, cur_pos.1) != Self::pos_of(b'X') {
            let mut x_first = Vec::new();
            x_first.extend(repeat_n(if x_ofs > 0 { b'>' } else { b'<' }, x_ofs.abs() as usize));
            x_first.extend(repeat_n(if y_ofs > 0 { b'v' } else { b'^' }, y_ofs.abs() as usize));
            x_first.push(b'A');
            paths.push({
                let mut h = FxHashMap::default();
                h.insert(x_first, 1);
                h
            });
        }
        if (cur_pos.0, cur_pos.1 + y_ofs) != Self::pos_of(b'X') {
            let mut y_first = Vec::new();
            y_first.extend(repeat_n(if y_ofs > 0 { b'v' } else { b'^' }, y_ofs.abs() as usize));
            y_first.extend(repeat_n(if x_ofs > 0 { b'>' } else { b'<' }, x_ofs.abs() as usize));
            y_first.push(b'A');
            paths.push({
                let mut h = FxHashMap::default();
                h.insert(y_first, 1);
                h
            });
        }
        if paths.is_empty() {
            panic!("all paths lead to the void");
        }
        paths.dedup();
        self.pointing_at = target;
        paths
    }
}

#[derive(Clone, Debug)]
struct DirectionKeypadRobot {
    pointing_at: u8,
    child: Option<Box<KeypadRobot>>,
    id: usize,
}

impl DirectionKeypadRobot {
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

    fn new(id: usize, child: Option<Box<KeypadRobot>>) -> Self {
        Self {
            id,
            pointing_at: b'A',
            child,
        }
    }
    fn press(&mut self, target: u8) -> Vec<FxHashMap<Vec<u8>, usize>> {
        let child_frequencies = self.child.as_mut().unwrap().press(target);
        let mut my_frequencies = Vec::new();

        for freq_set in child_frequencies {
            let mut local_freqs = FxHashMap::default();
            for (moves, count) in freq_set {
                assert_eq!(self.pointing_at, b'A');
                let path = self.path_to(&moves);
                for path_move in path.split_inclusive(|m| *m == b'A') {
                    let entry = local_freqs.entry(path_move.to_vec()).or_insert(0);
                    *entry = *entry + count;
                }
            }
            my_frequencies.push(local_freqs);
        }
        my_frequencies
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

fn run_robots(code: &Code, n: usize) -> i64 {
    let numpad = Box::new(KeypadRobot::Number(NumberKeypadRobot::new()));

    let mut robot = Box::new(KeypadRobot::Direction(DirectionKeypadRobot::new(0, Some(numpad))));
    for i in 1..n {
        let new_robot = Box::new(KeypadRobot::Direction(DirectionKeypadRobot::new(i, Some(robot))));
        robot = new_robot
    }

    // let mut sol_freqs = Vec::new();
    let mut sum = 0;
    for button in &code.0 {
        let paths = robot.press(*button).to_vec();
        let best = paths
            .iter()
            .map(|bp| bp.iter().map(|(k, v)| k.len() * v).sum::<usize>())
            .min()
            .unwrap();
        sum += best;
    }
    return sum as i64 * code.num_val();
}

#[aoc(day21, part1)]
fn part1(input: &str) -> i64 {
    let codes = parse(input);
    codes.iter().map(|c| run_robots(c, 2)).sum::<i64>() as i64
}

#[aoc(day21, part2)]
fn part2(input: &str) -> i64 {
    let codes = parse(input);
    for i in 0..26 {
        let res = codes.iter().map(|c| run_robots(c, i)).sum::<i64>() as i64;
        println!("{i}: {res}");
    }
    0
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
