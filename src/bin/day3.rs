use regex::Regex;

const INPUT: &str = "./input/day3.txt";

fn part1(input: &str) -> u32 {
    // We want to match only perfect "mul" instructions.
    Regex::new(r"mul\((\d+),(\d+)\)")
        .unwrap()
        .captures_iter(input)
        .map(|capture| {
            capture.get(1).unwrap().as_str().parse::<u32>().unwrap()
                * capture.get(2).unwrap().as_str().parse::<u32>().unwrap()
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    // We now include "do" and "don't" instructions, to enable/disable the accumulation.
    let re = Regex::new(r"mul\((\d+),(\d+)\)|do\(\)|don't\(\)").unwrap();

    let mut enabled = true;
    let mut total: u32 = 0;
    for capture in re.captures_iter(input) {
        match capture.get(0).unwrap().as_str() {
            "do()" => enabled = true,
            "don't()" => enabled = false,
            _ => {
                if enabled {
                    total += capture.get(1).unwrap().as_str().parse::<u32>().unwrap()
                        * capture.get(2).unwrap().as_str().parse::<u32>().unwrap()
                }
            }
        }
    }

    total
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT)?;

    println!("The first answer is: {}", part1(&input));
    println!("The second answer is: {}", part2(&input));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        const SAMPLE: &str =
            "xmul(2,4)%&mul[3,7]!@^do_not_mul(5,5)+mul(32,64]then(mul(11,8)mul(8,5))";
        let actual = part1(SAMPLE);
        let expected = 161;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2() {
        const SAMPLE: &str =
            "xmul(2,4)&mul[3,7]!^don't()_mul(5,5)+mul(32,64](mul(11,8)undo()?mul(8,5))";
        let actual = part2(SAMPLE);
        let expected = 48;
        assert_eq!(expected, actual);
    }
}
