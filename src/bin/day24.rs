use std::cell::OnceCell;
use std::collections::HashMap;
use std::ops::Shl;
use std::str::FromStr;

const INPUT: &str = "./input/day24.txt";

#[derive(Debug, Clone, PartialEq, Eq)]
enum Output<'output> {
    Gate(Gate<'output>),
    Wire(u64),
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GateType {
    And,
    Or,
    Xor,
}

impl FromStr for GateType {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "AND" => Ok(Self::And),
            "OR" => Ok(Self::Or),
            "XOR" => Ok(Self::Xor),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone, Eq)]
struct Gate<'gate> {
    gate_type: GateType,
    input_wires: (&'gate str, &'gate str),
    value: OnceCell<u64>,
}

impl Gate<'_> {
    fn get(&self, device: &Device) -> u64 {
        *self.value.get_or_init(|| {
            let a = device.get_value(self.input_wires.0);
            let b = device.get_value(self.input_wires.1);
            match self.gate_type {
                GateType::And => a & b,
                GateType::Or => a | b,
                GateType::Xor => a ^ b,
            }
        })
    }
}

impl PartialEq for Gate<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.gate_type == other.gate_type
            && ((self.input_wires.0 == other.input_wires.0
                && self.input_wires.1 == other.input_wires.1)
                || (self.input_wires.0 == other.input_wires.1
                    && self.input_wires.1 == other.input_wires.0))
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Device<'device> {
    outputs: HashMap<&'device str, Output<'device>>,
}

impl<'device> Device<'device> {
    fn new(s: &'device str) -> Self {
        let (initial, gates) = s.split_once("\n\n").unwrap();

        let input_wires = initial.lines().map(|line| {
            line.split_once(": ")
                .map(|(name, value)| (name, Output::Wire(value.parse().unwrap())))
                .unwrap()
        });
        let gates = gates.lines().map(|line| {
            let mut iter = line.split_whitespace();
            let a = iter.next().unwrap();
            let gate_type = iter.next().unwrap().parse::<GateType>().unwrap();
            let b = iter.next().unwrap();
            let wire = iter.nth(1).unwrap();

            (
                wire,
                Output::Gate(Gate {
                    gate_type,
                    input_wires: (a, b),
                    value: OnceCell::new(),
                }),
            )
        });

        Self {
            outputs: input_wires.chain(gates).collect(),
        }
    }

    fn get_value(&self, wire: &str) -> u64 {
        match self.outputs.get(wire).unwrap() {
            Output::Gate(gate) => gate.get(self),
            Output::Wire(w) => *w,
        }
    }

    fn output(&self) -> u64 {
        let mut keys: Vec<&&str> = self
            .outputs
            .keys()
            .filter(|key| key.starts_with("z"))
            .collect();
        keys.sort();

        keys.into_iter()
            .rev()
            .fold(0, |acc, key| acc.shl(1) + self.get_value(key))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT)?;
    let device = Device::new(&input);

    println!("The first answer is: {}", device.output());

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SMALL_EXAMPLE: &str = "\
        x00: 1\n\
        x01: 1\n\
        x02: 1\n\
        y00: 0\n\
        y01: 1\n\
        y02: 0\n\
        \n\
        x00 AND y00 -> z00\n\
        x01 XOR y01 -> z01\n\
        x02 OR y02 -> z02\n\
    ";

    const LARGE_EXAMPLE: &str = "\
        x00: 1\n\
        x01: 0\n\
        x02: 1\n\
        x03: 1\n\
        x04: 0\n\
        y00: 1\n\
        y01: 1\n\
        y02: 1\n\
        y03: 1\n\
        y04: 1\n\
        \n\
        ntg XOR fgs -> mjb\n\
        y02 OR x01 -> tnw\n\
        kwq OR kpj -> z05\n\
        x00 OR x03 -> fst\n\
        tgd XOR rvg -> z01\n\
        vdt OR tnw -> bfw\n\
        bfw AND frj -> z10\n\
        ffh OR nrd -> bqk\n\
        y00 AND y03 -> djm\n\
        y03 OR y00 -> psh\n\
        bqk OR frj -> z08\n\
        tnw OR fst -> frj\n\
        gnj AND tgd -> z11\n\
        bfw XOR mjb -> z00\n\
        x03 OR x00 -> vdt\n\
        gnj AND wpb -> z02\n\
        x04 AND y00 -> kjc\n\
        djm OR pbm -> qhw\n\
        nrd AND vdt -> hwm\n\
        kjc AND fst -> rvg\n\
        y04 OR y02 -> fgs\n\
        y01 AND x02 -> pbm\n\
        ntg OR kjc -> kwq\n\
        psh XOR fgs -> tgd\n\
        qhw XOR tgd -> z09\n\
        pbm OR djm -> kpj\n\
        x03 XOR y03 -> ffh\n\
        x00 XOR y04 -> ntg\n\
        bfw OR bqk -> z06\n\
        nrd XOR fgs -> wpb\n\
        frj XOR qhw -> z04\n\
        bqk OR frj -> z07\n\
        y03 OR x01 -> nrd\n\
        hwm AND bqk -> z03\n\
        tgd XOR rvg -> z12\n\
        tnw OR pbm -> gnj\n\
    ";

    #[test]
    fn test_part1_small_example() {
        let device = Device::new(SMALL_EXAMPLE);

        let actual = device.output();
        let expected = 4;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part1_large_example() {
        let device = Device::new(LARGE_EXAMPLE);

        let actual = device.output();
        let expected = 2024;

        assert_eq!(expected, actual);
    }
}
