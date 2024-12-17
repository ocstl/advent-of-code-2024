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
    a: u64,
    b: u64,
    c: u64,
    instruction_pointer: usize,
    program: Program,
}

impl Computer {
    fn new(a: u64, b: u64, c: u64, program: Program) -> Self {
        Self {
            a,
            b,
            c,
            instruction_pointer: 0,
            program,
        }
    }

    fn initialize(&mut self, a: u64) -> &mut Self {
        self.a = a;

        self
    }

    fn operand(&self, operand: Operand) -> u64 {
        match operand {
            Operand::Value(value) => u64::from(value),
            Operand::A => self.a,
            Operand::B => self.b,
            Operand::C => self.c,
            Operand::Reserved => unreachable!("Reserved."),
        }
    }
}

impl Iterator for Computer {
    type Item = u8;

    fn next(&mut self) -> Option<Self::Item> {
        while let (Some(&opcode), Some(&operand)) = (
            self.program.get(self.instruction_pointer),
            self.program.get(self.instruction_pointer + 1),
        ) {
            self.instruction_pointer += 2;
            let combo_operand = Operand::from(operand);

            match Instructions::from(opcode) {
                Instructions::Adv => self.a /= 2_u64.pow(self.operand(combo_operand) as u32),
                Instructions::Bxl => self.b ^= u64::from(operand),
                Instructions::Bst => self.b = self.operand(combo_operand) % 8,
                Instructions::Jnz => {
                    if self.a != 0 {
                        self.instruction_pointer = usize::from(operand);
                    }
                }
                Instructions::Bxc => self.b ^= self.c,
                Instructions::Out => return Some(self.operand(combo_operand) as u8 % 8),
                Instructions::Bdv => {
                    self.b = self.a / 2_u64.pow(self.operand(combo_operand) as u32)
                }
                Instructions::Cdv => {
                    self.c = self.a / 2_u64.pow(self.operand(combo_operand) as u32)
                }
            }
        }

        None
    }
}

fn parse_input(input: &str) -> Computer {
    let (computer, program) = input.split_once("\n\n").unwrap();
    let mut registers = computer.lines().map(|line| {
        line.split_whitespace()
            .last()
            .unwrap()
            .parse::<u64>()
            .unwrap()
    });
    let a = registers.next().unwrap();
    let b = registers.next().unwrap();
    let c = registers.next().unwrap();

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

    Computer::new(a, b, c, program)
}

fn part2(computer: &Computer) -> u64 {
    // Decoding the program:
    // - the loop depends on register A, so its final value needs to be 0.
    // - the output occurs right before the jump, and only depends on register A (B and C are
    //   filled from operations with A).
    // - for each loop, the value in register A is divided by 8.
    // So, we will try to reconstruct the initial value of register A by iterating backwards.

    let mut possibles_values = vec![0];
    for &output in computer.program.iter().rev() {
        possibles_values = possibles_values
            .into_iter()
            .flat_map(|previous| (0..8).map(move |new| previous * 8 + new))
            .filter(|&new| computer.clone().initialize(new).next().unwrap() == output)
            .collect();
    }

    possibles_values.into_iter().min().unwrap()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT)?;
    let computer = parse_input(&input);

    println!(
        "The first answer is: {}",
        computer
            .clone()
            .flat_map(|value| [char::from(value + b'0'), ','])
            .collect::<String>()
    );

    println!("The second answer is: {}", part2(&computer));

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

    const EXAMPLE_2: &str = "\
        Register A: 2024\n\
        Register B: 0\n\
        Register C: 0\n\
        \n\
        Program: 0,3,5,4,3,0\n\
    ";

    #[test]
    fn test_part1() {
        let computer = parse_input(EXAMPLE);
        let actual: Vec<u8> = computer.collect();
        let expected = vec![4, 6, 3, 5, 6, 3, 5, 2, 1, 0];

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2() {
        let computer = parse_input(EXAMPLE_2);

        let actual = part2(&computer);
        let expected = 117440;

        assert_eq!(expected, actual);
    }
}
