use advent_of_code_2024::position::Position;
use std::collections::{HashMap, HashSet};
use std::convert::Infallible;
use std::str::FromStr;

const INPUT: &str = "./input/day8.txt";

#[derive(Debug, Clone, PartialEq, Eq)]
struct Roof {
    height: isize,
    width: isize,
    antennas: HashMap<char, Vec<Position>>,
}

impl Roof {
    fn count_antinodes(&self) -> usize {
        let mut antinodes = HashSet::new();

        for frequency in self.antennas.values() {
            for idx in 0..frequency.len() {
                let first = frequency[idx];
                for &second in frequency.iter().skip(idx + 1) {
                    let direction = second - first;
                    // We have two candidates: before the first antenna, or after the second.
                    if self.is_position_valid(first - direction) {
                        antinodes.insert(first - direction);
                    }
                    if self.is_position_valid(second + direction) {
                        antinodes.insert(second + direction);
                    }
                }
            }
        }

        antinodes.len()
    }

    fn count_resonant_frequencies(&self) -> usize {
        let mut antinodes = HashSet::new();

        for frequency in self.antennas.values() {
            for idx in 0..frequency.len() {
                let first = frequency[idx];
                for &second in frequency.iter().skip(idx + 1) {
                    let direction = second - first;
                    // We need to go in both directions until we fall off the roof.
                    let mut p = first;
                    while self.is_position_valid(p) {
                        antinodes.insert(p);
                        p -= direction;
                    }
                    let mut p = second;
                    while self.is_position_valid(p) {
                        antinodes.insert(p);
                        p += direction;
                    }
                }
            }
        }

        antinodes.len()
    }

    fn is_position_valid(&self, position: Position) -> bool {
        position.x >= 0 && position.y >= 0 && position.x < self.width && position.y < self.height
    }
}

impl FromStr for Roof {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height = s.lines().count() as isize;
        let width = s.lines().next().unwrap_or_default().len() as isize;
        let mut antennas: HashMap<char, Vec<Position>> = HashMap::new();
        for (idy, line) in s.lines().enumerate() {
            for (idx, c) in line.char_indices() {
                if c.is_alphanumeric() {
                    antennas
                        .entry(c)
                        .or_default()
                        .push(Position::new(idx as isize, idy as isize));
                }
            }
        }

        Ok(Self {
            height,
            width,
            antennas,
        })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT)?;
    let roof = input.parse::<Roof>()?;

    println!("The first answer is: {}", roof.count_antinodes());
    println!(
        "The second answer is: {}",
        roof.count_resonant_frequencies()
    );

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = "\
        ............\n\
        ........0...\n\
        .....0......\n\
        .......0....\n\
        ....0.......\n\
        ......A.....\n\
        ............\n\
        ............\n\
        ........A...\n\
        .........A..\n\
        ............\n\
        ............\n\
    ";

    #[test]
    fn test_part1() {
        let actual = SAMPLE.parse::<Roof>().unwrap().count_antinodes();
        let expected = 14;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2() {
        let actual = SAMPLE.parse::<Roof>().unwrap().count_resonant_frequencies();
        let expected = 34;
        assert_eq!(expected, actual);
    }
}
