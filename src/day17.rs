use aoc_runner_derive::aoc;
use itertools::Itertools;
use regex::Regex;

#[derive(Debug, Clone, Copy)]
enum Register {
    A = 0,
    B = 1,
    C = 2,
}
impl From<usize> for Register {
    fn from(value: usize) -> Self {
        match value {
            0 => Register::A,
            1 => Register::B,
            2 => Register::C,
            v => panic!("invalid register address {}", v),
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Opcode {
    Adv = 0,
    Bxl = 1,
    Bst = 2,
    Jnz = 3,
    Bxc = 4,
    Out = 5,
    Bdv = 6,
    Cdv = 7,
}

impl From<i64> for Opcode {
    fn from(value: i64) -> Self {
        match value {
            0 => Opcode::Adv,
            1 => Opcode::Bxl,
            2 => Opcode::Bst,
            3 => Opcode::Jnz,
            4 => Opcode::Bxc,
            5 => Opcode::Out,
            6 => Opcode::Bdv,
            7 => Opcode::Cdv,
            v => panic!("invalid opcode {}", v),
        }
    }
}

impl Opcode {
    fn interp_operand(&self, value: i64) -> Operand {
        match self {
            Self::Adv | Self::Bst | Self::Out | Self::Bdv | Self::Cdv => Operand::new_combo(value),
            Self::Bxl | Self::Jnz => Operand::Literal(value),
            Self::Bxc => Operand::Ignore,
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Operand {
    Literal(i64),
    Load(Register),
    Ignore,
}

impl Operand {
    fn new_combo(value: i64) -> Self {
        match value {
            0..=3 => Operand::Literal(value),
            4 => Operand::Load(Register::A),
            5 => Operand::Load(Register::B),
            6 => Operand::Load(Register::C),
            7 => panic!("reserved combo operand 7"),
            i => panic!("invalid combo operand {}", i),
        }
    }
    fn value(self, m: &Machine) -> i64 {
        match self {
            Self::Literal(i) => i,
            Self::Load(reg) => *m.registers.load(reg),
            Self::Ignore => panic!("can't read ignored operand"),
        }
    }
}

#[derive(Debug)]
struct RegisterFile<const SIZE: usize, T> {
    file: [T; SIZE],
}

impl<T: Clone + From<i64>, const SIZE: usize> RegisterFile<SIZE, T> {
    fn load(&self, reg: Register) -> &T {
        &self.file[reg as usize]
    }
    fn store(&mut self, reg: Register, val: T) {
        self.file[reg as usize] = val;
    }
    fn reset(&mut self) {
        self.file.fill(0.into());
    }
}

#[derive(Debug, Copy, Clone)]
struct Instruction {
    opcode: Opcode,
    operand: Operand,
}

impl Instruction {
    fn exec(&self, m: &mut Machine) {
        match self.opcode {
            Opcode::Adv => self.adv(m),
            Opcode::Bxl => self.bxl(m),
            Opcode::Bst => self.bst(m),
            Opcode::Jnz => self.jnz(m),
            Opcode::Bxc => self.bxc(m),
            Opcode::Out => self.out(m),
            Opcode::Bdv => self.bdv(m),
            Opcode::Cdv => self.cdv(m),
        }
    }
    fn adv(&self, m: &mut Machine) {
        let num = m.registers.load(Register::A);
        let denom = 1 << self.operand.value(m);
        m.registers.store(Register::A, num / denom);
        m.advance();
    }
    fn bxl(&self, m: &mut Machine) {
        let lhs = self.operand.value(m);
        let rhs = m.registers.load(Register::B);
        m.registers.store(Register::B, lhs ^ rhs);
        m.advance();
    }
    fn bst(&self, m: &mut Machine) {
        m.registers.store(Register::B, self.operand.value(m) % 8);
        m.advance();
    }
    fn jnz(&self, m: &mut Machine) {
        if *m.registers.load(Register::A) == 0 {
            m.advance();
        } else {
            m.jump(self.operand.value(m) as usize);
        }
    }
    fn bxc(&self, m: &mut Machine) {
        let lhs = m.registers.load(Register::B);
        let rhs = m.registers.load(Register::C);
        m.registers.store(Register::B, lhs ^ rhs);
        m.advance();
    }
    fn out(&self, m: &mut Machine) {
        m.out_file.push(self.operand.value(m) % 8);
        m.advance();
    }
    fn bdv(&self, m: &mut Machine) {
        let num = m.registers.load(Register::A);
        let denom = 1 << self.operand.value(m);
        m.registers.store(Register::B, num / denom);
        m.advance();
    }
    fn cdv(&self, m: &mut Machine) {
        let num = m.registers.load(Register::A);
        let denom = 1 << self.operand.value(m);
        m.registers.store(Register::C, num / denom);
        m.advance();
    }
}

#[derive(Debug)]
pub struct Machine {
    registers: RegisterFile<3, i64>,
    program: Vec<Instruction>,
    program_raw: Vec<i64>,
    ip: usize,
    out_file: Vec<i64>,
}

impl Machine {
    fn run(&mut self) {
        let program = self.program.clone();
        while let Some(inst) = program.get(self.ip) {
            inst.exec(self);
        }
    }
    fn advance(&mut self) {
        self.ip += 1;
    }
    fn jump(&mut self, addr: usize) {
        self.ip = addr;
    }
    fn reset(&mut self) {
        self.registers.reset();
        self.ip = 0;
        self.out_file.clear();
    }
}

fn parse(input: &str) -> Machine {
    let reg_re = Regex::new(r"Register ([ABC]): (\d+)").unwrap();
    let prog_re = Regex::new(r"Program: ((\d+,)*\d+)").unwrap();

    let mut registers: RegisterFile<3, i64> = RegisterFile { file: [0; 3] };
    let mut program = Vec::new();
    let mut program_raw = Vec::new();
    for line in input.lines() {
        if let Some(caps) = reg_re.captures(line) {
            let address = (caps[1].as_bytes()[0] - b'A') as usize;
            let value = caps[2].parse().unwrap();
            registers.store(address.into(), value);
            continue;
        }
        if let Some(caps) = prog_re.captures(line) {
            let instructions = caps[1].split(',');
            for (inst, operand) in instructions.tuples() {
                let opcode = inst.parse::<i64>().unwrap();
                let operand = operand.parse::<i64>().unwrap();
                program_raw.push(opcode);
                program_raw.push(operand);
                let opcode: Opcode = opcode.into();
                program.push(Instruction {
                    operand: opcode.interp_operand(operand),
                    opcode,
                });
            }
        }
    }

    Machine {
        registers,
        program,
        program_raw,
        out_file: Vec::new(),
        ip: 0,
    }
}

#[aoc(day17, part1)]
pub fn part1(input: &str) -> String {
    let mut machine = parse(input);
    machine.run();
    machine.out_file.iter().map(|n| n.to_string()).join(",")
}

// The program must be of the correct form to be solvable
pub fn solve(m: &mut Machine, guess: i64, i: usize) -> Option<i64> {
    if i == m.program_raw.len() {
        return Some(guess);
    }
    let program_pos = m.program_raw.len() - 1 - i;
    let goal_digit = m.program_raw[program_pos];

    for digit in 0..8 {
        let local_guess = (digit << (program_pos * 3)) + guess;
        m.reset();
        m.registers.store(Register::A, local_guess);
        m.run();
        if m.out_file.len() == m.program_raw.len() && m.out_file[program_pos] == goal_digit {
            if let Some(sol) = solve(m, local_guess, i + 1) {
                return Some(sol);
            }
        }
    }
    None
}

#[aoc(day17, part2)]
pub fn part2(input: &str) -> i64 {
    let mut machine = parse(input);

    solve(&mut machine, 0, 0).expect("expected a solution")
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE1: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    const EXAMPLE2: &str = "Register A: 2024
Register B: 0
Register C: 0

Program: 0,3,5,4,3,0";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE1), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE2), 117440);
    }
}
