use regex::Regex;
use std::num::ParseIntError;
use std::str::FromStr;

const INPUT: &str = "./input/day13.txt";

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
struct Coordinates {
    x: i64,
    y: i64,
}

impl Coordinates {
    fn new(x: i64, y: i64) -> Self {
        Self { x, y }
    }
}

impl std::ops::Add for Coordinates {
    type Output = Self;

    fn add(self, other: Self) -> Self {
        Self {
            x: self.x + other.x,
            y: self.y + other.y,
        }
    }
}

impl std::ops::Mul<i64> for Coordinates {
    type Output = Self;

    fn mul(self, other: i64) -> Self {
        Self {
            x: self.x * other,
            y: self.y * other,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Machine {
    a: Coordinates,
    b: Coordinates,
    prize: Coordinates,
}

impl Machine {
    const COST_A: i64 = 3;
    const COST_B: i64 = 1;
    const CORRECTION: i64 = 10000000000000;

    fn minimal_cost(&self) -> Option<i64> {
        // We can use Cramer's rule to solve the linear system of equations.
        // Note: this only works because our inputs are not collinear.
        let determinant = self.a.x * self.b.y - self.a.y * self.b.x;
        let a = (self.prize.x * self.b.y - self.prize.y * self.b.x) / determinant;
        let b = (self.prize.y * self.a.x - self.prize.x * self.a.y) / determinant;

        // Since our division above ignores remainders, we have to check that we are indeed falling
        // on our correction prize.
        if a <= 100 && b <= 100 && self.a * a + self.b * b == self.prize {
            Some(a * Self::COST_A + b * Self::COST_B)
        } else {
            None
        }
    }

    fn minimal_cost_corrected(&self) -> Option<i64> {
        let corrected_prize = Coordinates::new(
            self.prize.x + Self::CORRECTION,
            self.prize.y + Self::CORRECTION,
        );

        // We can use Cramer's rule to solve the linear system of equations.
        // Note: this only works because our inputs are not collinear.
        let determinant = self.a.x * self.b.y - self.a.y * self.b.x;
        let a = (corrected_prize.x * self.b.y - corrected_prize.y * self.b.x) / determinant;
        let b = (corrected_prize.y * self.a.x - corrected_prize.x * self.a.y) / determinant;

        // Since our division above ignores remainders, we have to check that we are indeed falling
        // on our correction prize.
        if self.a * a + self.b * b == corrected_prize {
            Some(a * Self::COST_A + b * Self::COST_B)
        } else {
            None
        }
    }
}

impl FromStr for Machine {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let a = Regex::new(r"Button A: X\+(\d+), Y\+(\d+)")
            .unwrap()
            .captures(lines.next().unwrap())
            .map(|captures| {
                Coordinates::new(
                    captures.get(1).unwrap().as_str().parse().unwrap(),
                    captures.get(2).unwrap().as_str().parse().unwrap(),
                )
            })
            .unwrap();
        let b = Regex::new(r"Button B: X\+(\d+), Y\+(\d+)")
            .unwrap()
            .captures(lines.next().unwrap())
            .map(|captures| {
                Coordinates::new(
                    captures.get(1).unwrap().as_str().parse().unwrap(),
                    captures.get(2).unwrap().as_str().parse().unwrap(),
                )
            })
            .unwrap();
        let prize = Regex::new(r"Prize: X=(\d+), Y=(\d+)")
            .unwrap()
            .captures(lines.next().unwrap())
            .map(|captures| {
                Coordinates::new(
                    captures.get(1).unwrap().as_str().parse().unwrap(),
                    captures.get(2).unwrap().as_str().parse().unwrap(),
                )
            })
            .unwrap();

        Ok(Machine { a, b, prize })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT)?;
    let machines = input
        .split("\n\n")
        .map(str::parse)
        .collect::<Result<Vec<Machine>, _>>()?;

    println!(
        "The first answer is: {}",
        machines
            .iter()
            .filter_map(Machine::minimal_cost)
            .sum::<i64>()
    );
    println!(
        "The second answer is: {}",
        machines
            .iter()
            .filter_map(Machine::minimal_cost_corrected)
            .sum::<i64>()
    );

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLES: &str = "\
        Button A: X+94, Y+34\n\
        Button B: X+22, Y+67\n\
        Prize: X=8400, Y=5400\n\
        \n\
        Button A: X+26, Y+66\n\
        Button B: X+67, Y+21\n\
        Prize: X=12748, Y=12176\n\
        \n\
        Button A: X+17, Y+86\n\
        Button B: X+84, Y+37\n\
        Prize: X=7870, Y=6450\n\
        \n\
        Button A: X+69, Y+23\n\
        Button B: X+27, Y+71\n\
        Prize: X=18641, Y=10279\n\
    ";

    #[test]
    fn test_part1() {
        let machines = EXAMPLES
            .split("\n\n")
            .map(str::parse)
            .collect::<Result<Vec<Machine>, _>>()
            .unwrap();
        let actual: Vec<Option<i64>> = machines
            .into_iter()
            .map(|machine| machine.minimal_cost())
            .collect();
        let expected = vec![Some(280), None, Some(200), None];

        assert_eq!(actual, expected);
    }

    #[test]
    fn test_part2() {
        let machines = EXAMPLES
            .split("\n\n")
            .map(str::parse)
            .collect::<Result<Vec<Machine>, _>>()
            .unwrap();
        let actual: Vec<Option<i64>> = machines
            .into_iter()
            .map(|machine| machine.minimal_cost_corrected())
            .collect();

        assert!(actual[0].is_none());
        assert!(actual[1].is_some());
        assert!(actual[2].is_none());
        assert!(actual[3].is_some());
    }
}
