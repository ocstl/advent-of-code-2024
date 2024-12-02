use std::num::ParseIntError;
use std::str::FromStr;

const INPUT: &str = "./input/day2.txt";

#[derive(Debug, Clone)]
struct Report {
    levels: Vec<i32>,
}

impl Report {
    fn is_safe(&self) -> bool {
        // The levels have to be either strictly increasing or decreasing, as well as steps less
        // than or equal to 3.
        let changes: Vec<i32> = self
            .levels
            .windows(2)
            .map(|window| window[1] - window[0])
            .collect();

        match changes[0].signum() {
            0 => false,
            1 => changes
                .iter()
                .all(|change| change.is_positive() && *change <= 3),
            -1 => changes
                .iter()
                .all(|change| change.is_negative() && *change >= -3),
            _ => unreachable!("Signum yields 0, 1 or -1."),
        }
    }

    fn is_safe_with_dampener(&self) -> bool {
        // We'll just brute force it. Don't forget to check if it's already safe!
        self.is_safe()
            || (0..self.levels.len()).any(|idx| {
                let mut report = self.clone();
                report.levels.remove(idx);
                report.is_safe()
            })
    }
}

impl FromStr for Report {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.split_whitespace()
            .map(|level| level.parse())
            .collect::<Result<Vec<i32>, _>>()
            .map(|levels| Self { levels })
    }
}

fn part1(reports: &[Report]) -> usize {
    reports.iter().filter(|report| report.is_safe()).count()
}

fn part2(reports: &[Report]) -> usize {
    reports
        .iter()
        .filter(|report| report.is_safe_with_dampener())
        .count()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let reports = std::fs::read_to_string(INPUT)?
        .lines()
        .map(|line| line.parse())
        .collect::<Result<Vec<Report>, _>>()?;

    println!("The first answer is: {}", part1(&reports));
    println!("The second answer is: {}", part2(&reports));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        const SAMPLE: &str = "7 6 4 2 1
                             1 2 7 8 9
                             9 7 6 2 1
                             1 3 2 4 5
                             8 6 4 4 1
                             1 3 6 7 9";
        let reports = SAMPLE
            .lines()
            .map(|report| report.parse())
            .collect::<Result<Vec<Report>, _>>()
            .unwrap();
        let actual = part1(&reports);
        let expected = 2;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2() {
        const SAMPLE: &str = "7 6 4 2 1
                             1 2 7 8 9
                             9 7 6 2 1
                             1 3 2 4 5
                             8 6 4 4 1
                             1 3 6 7 9";
        let reports = SAMPLE
            .lines()
            .map(|report| report.parse())
            .collect::<Result<Vec<Report>, _>>()
            .unwrap();
        let actual = part2(&reports);
        let expected = 4;
        assert_eq!(expected, actual);
    }
}
