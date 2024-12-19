use std::collections::HashMap;

const INPUT: &str = "./input/day19.txt";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum StripeColor {
    White,
    Blue,
    Black,
    Red,
    Green,
}

impl TryFrom<char> for StripeColor {
    type Error = char;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            'w' => Ok(StripeColor::White),
            'u' => Ok(StripeColor::Blue),
            'b' => Ok(StripeColor::Black),
            'r' => Ok(StripeColor::Red),
            'g' => Ok(StripeColor::Green),
            _ => Err(c),
        }
    }
}

type Pattern = Vec<StripeColor>;

fn parse_input(input: &str) -> (Vec<Pattern>, Vec<Pattern>) {
    let (towels, patterns) = input.split_once("\n\n").unwrap();
    let towels = towels
        .split(',')
        .map(|t| {
            t.trim()
                .chars()
                .map(StripeColor::try_from)
                .collect::<Result<Pattern, _>>()
                .unwrap()
        })
        .collect();
    let patterns = patterns
        .lines()
        .map(|l| {
            l.chars()
                .map(StripeColor::try_from)
                .collect::<Result<Pattern, _>>()
                .unwrap()
        })
        .collect();

    (towels, patterns)
}

fn possible_pattern(towels: &[Pattern], pattern: &[StripeColor]) -> bool {
    if pattern.is_empty() {
        true
    } else {
        towels.iter().any(|towel| {
            let l = towel.len();
            l <= pattern.len() && towel == &pattern[0..l] && possible_pattern(towels, &pattern[l..])
        })
    }
}

fn part1(towels: &[Pattern], patterns: &[Pattern]) -> usize {
    // To cut down on possibilities, retain only the towels whose pattern can't be reproduced with
    // simpler towels.
    let mut simpler_towels = towels.to_vec();
    simpler_towels.sort_by_key(|t| t.len());
    let simpler_towels = simpler_towels
        .into_iter()
        .fold(Vec::new(), |mut acc, towel| {
            if !possible_pattern(&acc, &towel) {
                acc.push(towel);
            }

            acc
        });

    patterns
        .iter()
        .filter(|pattern| possible_pattern(&simpler_towels, pattern))
        .count()
}

fn part2(towels: &[Pattern], patterns: &[Pattern]) -> usize {
    // We can use memoization to store previously seen patterns, building it as we go along.
    let mut memoizer: HashMap<Pattern, usize> = HashMap::new();
    fn possible_ways(
        towels: &[Pattern],
        pattern: &[StripeColor],
        memoizer: &mut HashMap<Pattern, usize>,
    ) -> usize {
        if pattern.is_empty() {
            return 1;
        }

        if !memoizer.contains_key(pattern) {
            let c: usize = towels
                .iter()
                .map(|towel| {
                    let l = towel.len();
                    if l <= pattern.len() && towel == &pattern[0..l] {
                        possible_ways(towels, &pattern[l..], memoizer)
                    } else {
                        0
                    }
                })
                .sum();
            memoizer.insert(pattern.to_vec(), c);
        }

        *memoizer.get(pattern).unwrap()
    }

    patterns
        .iter()
        .map(|pattern| possible_ways(towels, pattern, &mut memoizer))
        .sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT)?;
    let (towels, patterns) = parse_input(&input);

    println!("The first answer is: {}", part1(&towels, &patterns));
    println!("The second answer is: {}", part2(&towels, &patterns));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        r, wr, b, g, bwu, rb, gb, br\n\
        \n\
        brwrr\n\
        bggr\n\
        gbbr\n\
        rrbgbr\n\
        ubwu\n\
        bwurrg\n\
        brgr\n\
        bbrgwb\n\
    ";

    #[test]
    fn test_part1() {
        let (towels, patterns) = parse_input(EXAMPLE);

        let actual = part1(&towels, &patterns);
        let expected = 6;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2() {
        let (towels, patterns) = parse_input(EXAMPLE);

        let actual = part2(&towels, &patterns);
        let expected = 16;

        assert_eq!(expected, actual);
    }
}
