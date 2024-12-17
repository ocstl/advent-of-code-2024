const INPUT: &str = "./input/day17.txt";

type Program = Vec<u8>;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Instructions {
    Adv,
    Bxl,
    Bst,
    Jnz,
    Bxc,
    Out,
    Bdv,
    Cdv,
}

impl From<u8> for Instructions {
    fn from(value: u8) -> Self {
        match value {
            0 => Instructions::Adv,
            1 => Instructions::Bxl,
            2 => Instructions::Bst,
            3 => Instructions::Jnz,
            4 => Instructions::Bxc,
            5 => Instructions::Out,
            6 => Instructions::Bdv,
            7 => Instructions::Cdv,
            _ => unreachable!("Invalid instruction value: {}", value),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Operand {
    Value(u8),
    A,
    B,
    C,
    Reserved,
}

impl From<u8> for Operand {
    fn from(value: u8) -> Self {
        match value {
            0..=3 => Operand::Value(value),
            4 => Operand::A,
            5 => Operand::B,
            6 => Operand::C,
            7 => Operand::Reserved,
            _ => unreachable!("Invalid operand value: {}", value),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Computer {
    a: u32,
    b: u32,
    c: u32,
    instruction_pointer: usize,
}

impl Computer {
    fn initialize(a: u32, b: u32, c: u32) -> Self {
        Self {
            a,
            b,
            c,
            instruction_pointer: 0,
        }
    }

    fn operand(&self, operand: Operand) -> u32 {
        match operand {
            Operand::Value(value) => u32::from(value),
            Operand::A => self.a,
            Operand::B => self.b,
            Operand::C => self.c,
            Operand::Reserved => unreachable!("Reserved."),
        }
    }

    fn run_program(&mut self, program: &Program) -> Vec<u32> {
        let mut output = Vec::new();

        while let (Some(&opcode), Some(&operand)) = (
            program.get(self.instruction_pointer),
            program.get(self.instruction_pointer + 1),
        ) {
            self.instruction_pointer += 2;
            let combo_operand = Operand::from(operand);

            match Instructions::from(opcode) {
                Instructions::Adv => self.a /= 2_u32.pow(self.operand(combo_operand)),
                Instructions::Bxl => self.b ^= u32::from(operand),
                Instructions::Bst => self.b = self.operand(combo_operand) % 8,
                Instructions::Jnz => {
                    if self.a != 0 {
                        self.instruction_pointer = usize::from(operand);
                    }
                }
                Instructions::Bxc => self.b ^= self.c,
                Instructions::Out => output.push(self.operand(combo_operand) % 8),
                Instructions::Bdv => self.b = self.a / 2_u32.pow(self.operand(combo_operand)),
                Instructions::Cdv => self.c = self.a / 2_u32.pow(self.operand(combo_operand)),
            }
        }

        output
    }
}

fn parse_input(input: &str) -> (Computer, Program) {
    let (computer, program) = input.split_once("\n\n").unwrap();
    let mut registers = computer.lines().map(|line| {
        line.split_whitespace()
            .last()
            .unwrap()
            .parse::<u32>()
            .unwrap()
    });
    let a = registers.next().unwrap();
    let b = registers.next().unwrap();
    let c = registers.next().unwrap();

    let computer = Computer::initialize(a, b, c);
    let program = program
        .split_whitespace()
        .last()
        .unwrap()
        .bytes()
        .filter_map(|value| {
            if value != b',' {
                Some(value - b'0')
            } else {
                None
            }
        })
        .collect();

    (computer, program)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT)?;
    let (computer, program) = parse_input(&input);

    println!(
        "The first answer is: {}",
        computer
            .clone()
            .run_program(&program)
            .into_iter()
            .flat_map(|value| [char::from_digit(value, 10).unwrap(), ','])
            .collect::<String>()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        Register A: 729\n\
        Register B: 0\n\
        Register C: 0\n\
        \n\
        Program: 0,1,5,4,3,0\n\
    ";

    #[test]
    fn part1() {
        let (mut computer, program) = parse_input(EXAMPLE);
        let actual = computer.run_program(&program);
        let expected = vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0];

        assert_eq!(expected, actual);
    }
}
