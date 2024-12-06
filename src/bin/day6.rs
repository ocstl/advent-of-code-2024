use advent_of_code_2024::grid::{Direction, Grid, Position};
use std::collections::HashSet;

const INPUT: &str = "./input/day6.txt";
const GUARD: char = '^';

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Open,
    Obstacle,
}

impl TryFrom<char> for Tile {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Tile::Open,
            '#' => Tile::Obstacle,
            '^' => Tile::Open,
            _ => Err(value)?,
        })
    }
}

fn parse_input(input: &str) -> (Grid<Tile>, Position) {
    let height = input.lines().count();
    let width = input.lines().next().unwrap_or_default().len();
    let grid = input
        .lines()
        .flat_map(|line| line.chars().map(Tile::try_from))
        .collect::<Result<Vec<Tile>, _>>()
        .unwrap();

    let guard = input
        .lines()
        .enumerate()
        .find_map(|(idy, row)| {
            row.chars()
                .position(|c| c == GUARD)
                .map(|idx| Position::new(idx, idy))
        })
        .unwrap();

    (Grid::new(height, width, grid).unwrap(), guard)
}

fn generate_path(
    grid: &Grid<Tile>,
    guard: Position,
    direction: Direction,
) -> impl Iterator<Item = (Position, Direction)> + use<'_> {
    std::iter::successors(Some((guard, direction)), |&(p, mut d)| {
        // If faced with an obstacle, turn right.
        while (p + d).and_then(|new_position| grid.get(new_position)) == Some(&Tile::Obstacle) {
            d = d.rotate_right();
        }
        (p + d)
            // Don't forget to check that we are still in the grid!
            .filter(|&new_position| grid.get(new_position).is_some())
            .map(|new_position| (new_position, d))
    })
}

fn part1(grid: &Grid<Tile>, guard: Position) -> usize {
    generate_path(grid, guard, Direction::Up)
        // Check for unique positions.
        .map(|(p, _)| p)
        .collect::<HashSet<Position>>()
        .len()
}

fn part2(grid: &Grid<Tile>, guard: Position) -> usize {
    // An obstacle needs to be placed in the guard's path, so gather his path first.
    // But do not put one at his initial position.
    generate_path(grid, guard, Direction::Up)
        // Check for unique positions.
        .map(|(p, _)| p)
        .collect::<HashSet<Position>>()
        .into_iter()
        .filter(|&p| p != guard)
        .filter(|&obstacle| {
            // Add the obstacle to a new grid.
            let mut new_grid = grid.clone();
            *new_grid.get_mut(obstacle).unwrap() = Tile::Obstacle;

            // Check for a loop by keeping track of our (Position, Direction) pairs. If we have
            // seen it before, we are in a loop. Otherwise, it will terminate.
            let mut visited = HashSet::new();
            let result =
                !generate_path(&new_grid, guard, Direction::Up).all(|pair| visited.insert(pair));
            result
        })
        .count()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT)?;
    let (grid, guard) = parse_input(&input);

    println!("The first answer is: {}", part1(&grid, guard));
    println!("The second answer is: {}", part2(&grid, guard));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
        ....#.....\n\
        .........#\n\
        ..........\n\
        ..#.......\n\
        .......#..\n\
        ..........\n\
        .#..^.....\n\
        ........#.\n\
        #.........\n\
        ......#...\n\
    ";

    #[test]
    fn test_part1() {
        let (grid, guard) = parse_input(SAMPLE);
        let actual = part1(&grid, guard);
        let expected = 41;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2() {
        let (grid, guard) = parse_input(SAMPLE);
        let actual = part2(&grid, guard);
        let expected = 6;
        assert_eq!(expected, actual);
    }
}
