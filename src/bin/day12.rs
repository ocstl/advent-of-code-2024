use advent_of_code_2024::grid::{Direction, Grid, Position};
use std::collections::{BTreeSet, HashSet};
use std::convert::Infallible;
use std::str::FromStr;

const INPUT: &str = "./input/day12.txt";

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Farm(Grid<char>);

impl Farm {
    fn plots(&self) -> Vec<HashSet<Position>> {
        let mut plots = Vec::new();
        let mut positions: BTreeSet<Position> = (0..self.0.width())
            .flat_map(move |idx| ((0..self.0.height()).map(move |idy| Position::new(idx, idy))))
            .collect();

        while let Some(position) = positions.pop_first() {
            let plant = self.0.get(position).unwrap();

            let mut plot = HashSet::new();
            let mut to_visit = vec![position];

            while let Some(position) = to_visit.pop() {
                if plot.insert(position) {
                    positions.remove(&position);
                    for neighbour in position.neighbours() {
                        if self.0.get(neighbour) == Some(plant) {
                            to_visit.push(neighbour);
                        }
                    }
                }
            }

            plots.push(plot);
        }

        plots
    }

    fn fencing_cost(&self) -> usize {
        self.plots()
            .iter()
            .map(|plot| {
                let perimeter: usize = plot
                    .iter()
                    .map(|position| {
                        // Start with four sides, but remove any that are bordered by a similar
                        // plant.
                        4 - position
                            .neighbours()
                            .filter(|neighbour| plot.contains(neighbour))
                            .count()
                    })
                    .sum();
                perimeter * plot.len()
            })
            .sum()
    }

    fn discount_fencing_cost(&self) -> usize {
        const DIRECTIONS: [Direction; 4] = [
            Direction::Up,
            Direction::Down,
            Direction::Left,
            Direction::Right,
        ];

        self.plots()
            .iter()
            .map(|plot| {
                // Find the boundaries of our polygon (including interior boundaries), as pairs of
                // position and direction.
                let mut boundaries: BTreeSet<(Position, Direction)> = plot
                    .iter()
                    .flat_map(|&position| DIRECTIONS.iter().map(move |&d| (position, d)))
                    .filter(|&(p, d)| (p + d).map_or(true, |pd| !plot.contains(&pd)))
                    .collect();

                let mut sides = 0;
                // Start with any boundary, then follow that boundary left and right to form a side.
                // Remove those boundaries (position/direction pair) that this side accounts for.
                while let Some(pd) = boundaries.pop_first() {
                    sides += 1;
                    let mut to_visit = vec![pd];
                    while let Some((position, direction)) = to_visit.pop() {
                        if let Some(p) = position + direction.rotate_left() {
                            let left = (p, direction);
                            if boundaries.remove(&left) {
                                to_visit.push(left);
                            }
                        }
                        if let Some(p) = position + direction.rotate_right() {
                            let right = (p, direction);
                            if boundaries.remove(&right) {
                                to_visit.push(right);
                            }
                        }
                    }
                }

                sides * plot.len()
            })
            .sum()
    }
}

impl FromStr for Farm {
    type Err = Infallible;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height = s.lines().count();
        let width = s.lines().next().unwrap_or_default().chars().count();
        let grid = s.lines().flat_map(str::chars).collect();

        Ok(Self(Grid::new(height, width, grid).unwrap()))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT)?;
    let farm = input.parse::<Farm>()?;

    println!("The first answer is: {}", farm.fencing_cost());
    println!("The second answer is: {}", farm.discount_fencing_cost());

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = "\
        AAAA\n\
        BBCD\n\
        BBCC\n\
        EEEC\n\
    ";
    const EXAMPLE_2: &str = "\
        OOOOO\n\
        OXOXO\n\
        OOOOO\n\
        OXOXO\n\
        OOOOO\n\
    ";
    const EXAMPLE_3: &str = "\
        RRRRIICCFF\n\
        RRRRIICCCF\n\
        VVRRRCCFFF\n\
        VVRCCCJFFF\n\
        VVVVCJJCFE\n\
        VVIVCCJJEE\n\
        VVIIICJJEE\n\
        MIIIIIJJEE\n\
        MIIISIJEEE\n\
        MMMISSJEEE\n\
    ";

    #[test]
    fn part1_example1() {
        let farm = EXAMPLE_1.parse::<Farm>().unwrap();

        let actual = farm.fencing_cost();
        let expected = 140;

        assert_eq!(expected, actual);
    }

    #[test]
    fn part1_example2() {
        let farm = EXAMPLE_2.parse::<Farm>().unwrap();

        let actual = farm.fencing_cost();
        let expected = 772;

        assert_eq!(expected, actual);
    }

    #[test]
    fn part1_example3() {
        let farm = EXAMPLE_3.parse::<Farm>().unwrap();

        let actual = farm.fencing_cost();
        let expected = 1930;

        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_example1() {
        let farm = EXAMPLE_1.parse::<Farm>().unwrap();

        let actual = farm.discount_fencing_cost();
        let expected = 80;

        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_example2() {
        let farm = EXAMPLE_2.parse::<Farm>().unwrap();

        let actual = farm.discount_fencing_cost();
        let expected = 436;

        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_example3() {
        let farm = EXAMPLE_3.parse::<Farm>().unwrap();

        let actual = farm.discount_fencing_cost();
        let expected = 1206;

        assert_eq!(expected, actual);
    }
}
