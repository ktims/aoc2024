use aoc_runner_derive::aoc;
use itertools::Itertools;
use nom::And;
use regex::Regex;
use rustc_hash::FxHashMap;

#[derive(Copy, Clone, Debug)]
enum Op {
    And,
    Or,
    Xor,
    Constant,
}

impl From<&str> for Op {
    fn from(value: &str) -> Self {
        match value {
            "AND" => Self::And,
            "OR" => Self::Or,
            "XOR" => Self::Xor,
            s => panic!("invalid operation {}", s),
        }
    }
}

#[derive(Clone, Debug)]
struct Gate {
    op: Op,
    value: Option<bool>,
    arguments: [String; 2],
}

impl Gate {
    fn eval(&self, machine: &GateMachine) -> bool {
        match self.op {
            Op::And => machine.val_of(&self.arguments[0]) && machine.val_of(&self.arguments[1]),
            Op::Or => machine.val_of(&self.arguments[0]) || machine.val_of(&self.arguments[1]),
            Op::Xor => machine.val_of(&self.arguments[0]) ^ machine.val_of(&self.arguments[1]),
            Op::Constant => self.value.unwrap(),
        }
    }
}

#[derive(Debug)]
struct GateMachine {
    gates: FxHashMap<String, Gate>,
}

impl GateMachine {
    fn val_of(&self, gate: &str) -> bool {
        println!("gate: {}", gate);
        if let Some(val) = self.gates[gate].value {
            val
        } else {
            self.gates[gate].eval(self)
        }
    }
}

fn parse(input: &str) -> GateMachine {
    let mut gates = FxHashMap::default();
    for line in input.lines() {
        println!("{line}");
        let const_re = Regex::new(r"^([xyz][0-9]{2}): ([01])$").unwrap();
        let gate_re = Regex::new(r"^([a-z0-9]{3}) (AND|XOR|OR) ([a-z0-9]{3}) -> ([a-z0-9]{3})$").unwrap();

        if let Some(caps) = const_re.captures(line) {
            println!(" is const: {:?}", caps);
            gates.insert(
                caps[1].to_string(),
                Gate {
                    op: Op::Constant,
                    value: if &caps[2] == "1" { Some(true) } else { Some(false) },
                    arguments: [String::new(), String::new()],
                },
            );
        } else if let Some(caps) = gate_re.captures(line) {
            println!(" is gate: {:?}", caps);
            gates.insert(
                caps[4].to_string(),
                Gate {
                    op: Op::from(&caps[2]),
                    value: None,
                    arguments: [caps[1].to_string(), caps[3].to_string()],
                },
            );
        }
    }
    GateMachine { gates }
}

#[aoc(day24, part1)]
pub fn part1(input: &str) -> i64 {
    let machine = parse(input);
    let z_gates = machine
        .gates
        .keys()
        .filter(|k| k.starts_with('z'))
        .map(|s| (s, s.split_at(1).1.parse::<usize>().unwrap()));
    let bit_vals = z_gates
        .map(|(name, bit)| if machine.val_of(name) { 1 << bit } else { 0 })
        .fold(0, |accum, val| accum | val);
    bit_vals
}

#[aoc(day24, part2)]
pub fn part2(input: &str) -> i64 {
    0
}

#[cfg(test)]
mod tests {
    use super::*;
    const EXAMPLE1: &str = "x00: 1
x01: 1
x02: 1
y00: 0
y01: 1
y02: 0

x00 AND y00 -> z00
x01 XOR y01 -> z01x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj
x02 OR y02 -> z02";

    const EXAMPLE2: &str = "x00: 1
x01: 0
x02: 1
x03: 1
x04: 0
y00: 1
y01: 1
y02: 1
y03: 1
y04: 1

ntg XOR fgs -> mjb
y02 OR x01 -> tnw
kwq OR kpj -> z05
x00 OR x03 -> fst
tgd XOR rvg -> z01
vdt OR tnw -> bfw
bfw AND frj -> z10
ffh OR nrd -> bqk
y00 AND y03 -> djm
y03 OR y00 -> psh
bqk OR frj -> z08
tnw OR fst -> frj
gnj AND tgd -> z11
bfw XOR mjb -> z00
x03 OR x00 -> vdt
gnj AND wpb -> z02
x04 AND y00 -> kjc
djm OR pbm -> qhw
nrd AND vdt -> hwm
kjc AND fst -> rvg
y04 OR y02 -> fgs
y01 AND x02 -> pbm
ntg OR kjc -> kwq
psh XOR fgs -> tgd
qhw XOR tgd -> z09
pbm OR djm -> kpj
x03 XOR y03 -> ffh
x00 XOR y04 -> ntg
bfw OR bqk -> z06
nrd XOR fgs -> wpb
frj XOR qhw -> z04
bqk OR frj -> z07
y03 OR x01 -> nrd
hwm AND bqk -> z03
tgd XOR rvg -> z12
tnw OR pbm -> gnj";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE1), 4);
        assert_eq!(part1(EXAMPLE2), 2024);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE1), 0);
    }
}
