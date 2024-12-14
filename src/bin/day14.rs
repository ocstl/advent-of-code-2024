use advent_of_code_2024::position::{Direction, Position};
use counter::Counter;
use regex::Regex;
use std::collections::HashSet;
use std::fmt::{Display, Formatter};
use std::num::ParseIntError;
use std::str::FromStr;

const INPUT: &str = "./input/day14.txt";

const WIDTH: isize = 101;
const HEIGHT: isize = 103;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct Robot {
    position: Position,
    velocity: Direction,
}

impl Robot {
    fn new(position: Position, velocity: Direction) -> Self {
        Self { position, velocity }
    }
}

impl FromStr for Robot {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let captures = Regex::new(r"p=([-\d]+),([-\d]+) v=([-\d]+),([-\d]+)")
            .unwrap()
            .captures(s)
            .unwrap();
        let position = Position::new(captures[1].parse()?, captures[2].parse()?);
        let velocity = Direction::new(captures[3].parse()?, captures[4].parse()?);

        Ok(Self { position, velocity })
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct RobotGrid<const HEIGHT: isize, const WIDTH: isize>(Vec<Robot>);

impl<const HEIGHT: isize, const WIDTH: isize> RobotGrid<HEIGHT, WIDTH> {
    fn quadrant(position: Position) -> Option<(isize, isize)> {
        let qx = (position.x - crate::WIDTH / 2).signum();
        let qy = (position.y - crate::HEIGHT / 2).signum();

        if qx != 0 && qy != 0 {
            Some((qx, qy))
        } else {
            None
        }
    }

    fn robot_position(robot: Robot, seconds: isize) -> Position {
        let p = robot.position + robot.velocity * seconds;
        Position::new(p.x.rem_euclid(WIDTH), p.y.rem_euclid(HEIGHT))
    }

    fn safety_factor(&self, seconds: isize) -> usize {
        self.0
            .iter()
            .filter_map(|&r| Self::quadrant(Self::robot_position(r, seconds)))
            .collect::<Counter<(isize, isize)>>()
            .values()
            .product()
    }

    fn next(&mut self) -> &Self {
        for r in &mut self.0 {
            *r = Robot::new(Self::robot_position(*r, 1), r.velocity);
        }

        self
    }
}

impl<const HEIGHT: isize, const WIDTH: isize> FromStr for RobotGrid<HEIGHT, WIDTH> {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.lines()
            .map(str::parse)
            .collect::<Result<Vec<Robot>, _>>()
            .map(Self)
    }
}

impl<const HEIGHT: isize, const WIDTH: isize> Display for RobotGrid<HEIGHT, WIDTH> {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        let line: Vec<u8> = std::iter::repeat_n(b' ', WIDTH as usize).collect();
        let mut s: Vec<Vec<u8>> = std::iter::repeat_n(line.clone(), HEIGHT as usize).collect();

        for p in self.0.iter().map(|r| r.position).collect::<HashSet<_>>() {
            s[p.y as usize][p.x as usize] = b'#';
        }

        for mut line in s {
            line.push(b'\n');
            write!(f, "{}", &String::from_utf8(line).unwrap())?;
        }

        Ok(())
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT)?;
    let mut robots = input.parse::<RobotGrid<HEIGHT, WIDTH>>()?;

    println!("The first answer is: {}", robots.safety_factor(100));
    println!(
        "The second answer is: {}",
        (0..)
            .find(|_| {
                let answer = robots
                    .to_string()
                    .lines()
                    // This is the top (and bottom) of the Christmas tree.
                    .any(|line| line.contains("###############################"));
                robots.next();
                answer
            })
            .unwrap()
    );

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "\
        p=0,4 v=3,-3\n\
        p=6,3 v=-1,-3\n\
        p=10,3 v=-1,2\n\
        p=2,0 v=2,-1\n\
        p=0,0 v=1,3\n\
        p=3,0 v=-2,-2\n\
        p=7,6 v=-1,-3\n\
        p=3,0 v=-1,-2\n\
        p=9,3 v=2,3\n\
        p=7,3 v=-1,2\n\
        p=2,4 v=2,-3\n\
        p=9,5 v=-3,-3\n\
    ";

    #[test]
    fn part1_example() {
        let robots = EXAMPLE.parse::<RobotGrid<7, 11>>().unwrap();
        let actual = robots.safety_factor(100);
        let expected = 12;

        assert_eq!(expected, actual);
    }
}
