use std::{ops::BitXor, str::FromStr};

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
    adv = 0,
    bxl = 1,
    bst = 2,
    jnz = 3,
    bxc = 4,
    out = 5,
    bdv = 6,
    cdv = 7,
}

impl From<usize> for Opcode {
    fn from(value: usize) -> Self {
        match value {
            0 => Opcode::adv,
            1 => Opcode::bxl,
            2 => Opcode::bst,
            3 => Opcode::jnz,
            4 => Opcode::bxc,
            5 => Opcode::out,
            6 => Opcode::bdv,
            7 => Opcode::cdv,
            v => panic!("invalid opcode {}", v),
        }
    }
}

impl Opcode {
    fn interp_operand(&self, value: i64) -> Operand {
        match self {
            Self::adv | Self::bst | Self::out | Self::bdv | Self::cdv => Operand::new_combo(value),
            Self::bxl | Self::jnz => Operand::Literal(value),
            Self::bxc => Operand::Ignore,
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
    fn is_literal(self) -> bool {
        match self {
            Self::Literal(_) => true,
            _ => false,
        }
    }
}

#[derive(Debug)]
struct RegisterFile<const SIZE: usize, T> {
    file: [T; SIZE],
}

impl<T, const SIZE: usize> RegisterFile<SIZE, T> {
    fn load(&self, reg: Register) -> &T {
        &self.file[reg as usize]
    }
    fn store(&mut self, reg: Register, val: T) {
        self.file[reg as usize] = val;
    }
}

#[derive(Debug, Copy, Clone)]
struct Instruction {
    opcode: Opcode,
    operand: Operand,
}

impl Instruction {
    fn new(opcode: Opcode, operand: Operand) -> Self {
        Self { opcode, operand }
    }
    fn exec(&self, m: &mut Machine) {
        match self.opcode {
            Opcode::adv => self.adv(m),
            Opcode::bxl => self.bxl(m),
            Opcode::bst => self.bst(m),
            Opcode::jnz => self.jnz(m),
            Opcode::bxc => self.bxc(m),
            Opcode::out => self.out(m),
            Opcode::bdv => self.bdv(m),
            Opcode::cdv => self.cdv(m),

            _ => unimplemented!(),
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
        m.registers.store(Register::B, lhs.bitxor(rhs));
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
        let a = m.registers.load(Register::B);
        let b = m.registers.load(Register::C);
        m.registers.store(Register::B, a.bitxor(b));
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
struct Machine {
    registers: RegisterFile<3, i64>,
    program: Vec<Instruction>,
    ip: usize,
    out_file: Vec<i64>,
}

impl Machine {
    fn run(&mut self) {
        let program = self.program.clone();
        loop {
            if let Some(inst) = program.get(self.ip) {
                inst.exec(self);
            } else {
                break;
            }
        }
    }
    fn advance(&mut self) {
        self.ip += 1;
    }
    fn jump(&mut self, addr: usize) {
        self.ip = addr;
    }
}

fn parse(input: &str) -> Machine {
    let reg_re = Regex::new(r"Register ([ABC]): (\d+)").unwrap();
    let prog_re = Regex::new(r"Program: ((\d+,)*\d+)").unwrap();

    let mut registers: RegisterFile<3, i64> = RegisterFile { file: [0; 3] };
    let mut program = Vec::new();
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
                let opcode: Opcode = inst.parse::<usize>().unwrap().into();
                let operand = operand.parse::<i64>().unwrap().into();
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

#[aoc(day17, part2)]
pub fn part2(input: &str) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE: &str = "Register A: 729
Register B: 0
Register C: 0

Program: 0,1,5,4,3,0";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), "4,6,3,5,6,3,5,2,1,0");
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 0);
    }
}
