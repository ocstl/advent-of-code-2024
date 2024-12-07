use radixal::IntoDigits;

const INPUT: &str = "./input/day7.txt";

type Value = u64;
type Equation = (Value, Vec<Value>);

fn parse_input(input: &str) -> Vec<Equation> {
    input
        .lines()
        .filter_map(|line| {
            line.split_once(':').map(|(total, equation)| {
                let total = total.parse::<Value>().unwrap();
                let equation = equation
                    .split_whitespace()
                    .map(|value| value.parse::<Value>().unwrap())
                    .collect();
                (total, equation)
            })
        })
        .collect()
}

fn part1(equations: &[Equation]) -> Value {
    equations
        .iter()
        .filter_map(|(total, values)| {
            let mut possibilities = vec![*total];

            // We'll iterate backwards, eliminating negative values and divisions with a remainder.
            // Skip the first one, as we'll want to check its presence.
            for &value in values.iter().skip(1).rev() {
                let mut new_possibilities = Vec::new();
                for p in possibilities {
                    if let Some(sub) = p.checked_sub(value) {
                        new_possibilities.push(sub);
                    }
                    if p % value == 0 {
                        new_possibilities.push(p / value)
                    }
                }
                possibilities = new_possibilities;
            }
            if possibilities.contains(&values[0]) {
                Some(total)
            } else {
                None
            }
        })
        .sum()
}

fn part2(equations: &[Equation]) -> Value {
    equations
        .iter()
        .filter_map(|(total, values)| {
            let mut possibilities = vec![*total];

            // We'll iterate backwards, eliminating negative values and divisions with a remainder.
            // Skip the first one, as we'll want to check its presence.
            for &value in values.iter().skip(1).rev() {
                let mut new_possibilities = Vec::new();
                for p in possibilities {
                    if let Some(sub) = p.checked_sub(value) {
                        new_possibilities.push(sub);
                    }
                    if p % value == 0 {
                        new_possibilities.push(p / value)
                    }
                    let separator = 10_u64.pow(value.nbr_decimal_digits() as u32);
                    if p % separator == value {
                        new_possibilities.push(p / separator);
                    }
                }
                possibilities = new_possibilities;
            }
            if possibilities.contains(&values[0]) {
                Some(total)
            } else {
                None
            }
        })
        .sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT)?;
    let equations = parse_input(&input);

    println!("The first answer is: {}", part1(&equations));
    println!("The second answer is: {}", part2(&equations));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = "\
        190: 10 19\n\
        3267: 81 40 27\n\
        83: 17 5\n\
        156: 15 6\n\
        7290: 6 8 6 15\n\
        161011: 16 10 13\n\
        192: 17 8 14\n\
        21037: 9 7 18 13\n\
        292: 11 6 16 20\n\
    ";

    #[test]
    fn test_part1() {
        let actual = part1(&parse_input(&SAMPLE));
        let expected = 3749;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2() {
        let actual = part2(&parse_input(&SAMPLE));
        let expected = 11387;
        assert_eq!(expected, actual);
    }
}
