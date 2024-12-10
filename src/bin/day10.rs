use advent_of_code_2024::grid::{Grid, Position};
use std::collections::HashSet;
use std::num::ParseIntError;
use std::str::FromStr;

const INPUT: &str = "./input/day10.txt";

type Height = u32;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Map(Grid<Height>);

impl Map {
    const TRAILHEAD: Height = 0;
    const STEP: Height = 1;
    const SUMMIT: Height = 9;

    fn trailheads(&self) -> impl Iterator<Item = Position> + '_ {
        (0..self.0.height()).flat_map(move |row| {
            (0..self.0.width()).filter_map(move |col| {
                Some(Position::new(row, col)).filter(|p| self.0.get(*p) == Some(&Self::TRAILHEAD))
            })
        })
    }

    fn trailhead_score(&self, position: Position) -> Option<usize> {
        if self.0.get(position) != Some(&Self::TRAILHEAD) {
            return None;
        }

        let mut to_visit = vec![position];
        let mut visited = HashSet::new();
        let mut heights = HashSet::new();

        while let Some(position) = to_visit.pop() {
            if visited.insert(position) {
                match self.0.get(position) {
                    Some(&Self::SUMMIT) => {
                        heights.insert(position);
                    }
                    Some(current_height) => {
                        for neighbor in position.neighbours() {
                            if self.0.get(neighbor) == Some(&(current_height + Self::STEP)) {
                                to_visit.push(neighbor);
                            }
                        }
                    }
                    None => (),
                }
            }
        }

        Some(heights.len())
    }

    fn trailhead_rating(&self, position: Position) -> Option<usize> {
        if self.0.get(position) != Some(&Self::TRAILHEAD) {
            return None;
        }

        let mut to_visit = vec![position];
        let mut count = 0;

        while let Some(position) = to_visit.pop() {
            match self.0.get(position) {
                Some(&Self::SUMMIT) => count += 1,
                Some(current_height) => {
                    for neighbor in position.neighbours() {
                        if self.0.get(neighbor) == Some(&(current_height + Self::STEP)) {
                            to_visit.push(neighbor);
                        }
                    }
                }
                None => (),
            }
        }

        Some(count)
    }
}

impl FromStr for Map {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height = s.lines().count();
        let width = s.lines().next().unwrap_or_default().chars().count();

        let grid = s
            .lines()
            .flat_map(|line| line.chars().map(|c| c.to_digit(10).unwrap()))
            .collect();

        Ok(Self(Grid::new(height, width, grid).unwrap()))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT)?;
    let map = input.parse::<Map>()?;

    println!(
        "The first answer is: {}",
        map.trailheads()
            .filter_map(|p| map.trailhead_score(p))
            .sum::<usize>()
    );
    println!(
        "The second answer is: {}",
        map.trailheads()
            .filter_map(|p| map.trailhead_rating(p))
            .sum::<usize>()
    );

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE1: &str = "\
        0123\n\
        1234\n\
        8765\n\
        9876\n\
        ";
    const EXAMPLE2: &str = "\
        89010123\n\
        78121874\n\
        87430965\n\
        96549874\n\
        45678903\n\
        32019012\n\
        01329801\n\
        10456732\n\
        ";

    #[test]
    fn test_part1_ex1() {
        let map = EXAMPLE1.parse::<Map>().unwrap();
        let actual = map
            .trailheads()
            .filter_map(|p| map.trailhead_score(p))
            .sum();
        let expected = 1_usize;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part1_ex2() {
        let map = EXAMPLE2.parse::<Map>().unwrap();
        let actual = map
            .trailheads()
            .filter_map(|p| map.trailhead_score(p))
            .sum();
        let expected = 36_usize;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part_ex2() {
        let map = EXAMPLE2.parse::<Map>().unwrap();
        let actual = map
            .trailheads()
            .filter_map(|p| map.trailhead_rating(p))
            .sum();
        let expected = 81_usize;
        assert_eq!(expected, actual);
    }
}
