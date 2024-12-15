use advent_of_code_2024::grid::{Direction, Grid, Position};
use std::collections::VecDeque;
use std::fmt::Debug;

const INPUT: &str = "./input/day15.txt";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Wall,
    Empty,
    Box,
}

impl TryFrom<char> for Tile {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Tile::Wall),
            '.' | '@' => Ok(Tile::Empty),
            'O' => Ok(Tile::Box),
            _ => Err(value),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum LargeTile {
    Wall,
    Empty,
    LeftBox,
    RightBox,
}

impl TryFrom<char> for LargeTile {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(LargeTile::Wall),
            '.' | '@' => Ok(LargeTile::Empty),
            '[' => Ok(LargeTile::LeftBox),
            ']' => Ok(LargeTile::RightBox),
            _ => Err(value),
        }
    }
}

pub trait Warehouse {
    fn sum_gps_coordinates(&self) -> usize;
    fn push(&mut self, position: Position, direction: Direction) -> bool;
    fn print(&self) -> Vec<Vec<char>>;
}

impl Warehouse for Grid<Tile> {
    fn sum_gps_coordinates(&self) -> usize {
        self.iter()
            .filter_map(|(p, v)| {
                if *v == Tile::Box {
                    Some(p.x() + 100 * p.y())
                } else {
                    None
                }
            })
            .sum()
    }

    fn push(&mut self, position: Position, direction: Direction) -> bool {
        match self.get(position) {
            Some(Tile::Wall) => false,
            Some(Tile::Empty) => true,
            Some(Tile::Box) => {
                let next_position = (position + direction).unwrap();
                if self.push(next_position, direction) {
                    *self.get_mut(next_position).unwrap() = Tile::Box;
                    *self.get_mut(position).unwrap() = Tile::Empty;
                    true
                } else {
                    false
                }
            }
            None => false,
        }
    }

    fn print(&self) -> Vec<Vec<char>> {
        self.rows()
            .map(|line| {
                line.map(|t| match t {
                    Tile::Wall => '#',
                    Tile::Empty => '.',
                    Tile::Box => 'O',
                })
                .collect::<Vec<char>>()
            })
            .collect()
    }
}

impl Warehouse for Grid<LargeTile> {
    fn sum_gps_coordinates(&self) -> usize {
        self.iter()
            .filter_map(|(p, v)| {
                if *v == LargeTile::LeftBox {
                    Some(p.x() + 100 * p.y())
                } else {
                    None
                }
            })
            .sum()
    }

    fn push(&mut self, position: Position, direction: Direction) -> bool {
        // We first have to check whether we can push. The `Tile` version works for left and
        // right, but not up and down, as we have wide boxes.
        if direction == Direction::Left || direction == Direction::Right {
            match self.get(position) {
                Some(LargeTile::Wall) => false,
                Some(LargeTile::Empty) => true,
                Some(&tile @ LargeTile::LeftBox) | Some(&tile @ LargeTile::RightBox) => {
                    let next_position = (position + direction).unwrap();
                    if self.push(next_position, direction) {
                        *self.get_mut(next_position).unwrap() = tile;
                        *self.get_mut(position).unwrap() = LargeTile::Empty;
                        true
                    } else {
                        false
                    }
                }
                None => false,
            }
        } else {
            // Use a VecDeque to make sure we visit the tiles in order (up or down), which is
            // important when it comes to emptying `to_push` afterward. Otherwise, we end up
            // moving boxes that still can't be moved.
            let mut to_push = Vec::new();
            let mut to_visit = VecDeque::new();
            to_visit.push_back(position);

            // Check whether we can push, and record those we need to push.
            while let Some(position) = to_visit.pop_front() {
                match self.get(position) {
                    Some(LargeTile::Wall) => return false,
                    Some(LargeTile::Empty) => (),
                    Some(LargeTile::LeftBox) => {
                        // Check that we haven't accounted for it. Otherwise, we'll be pushing
                        // the same tile twice (once full, once empty). Everything breaks.
                        if !to_push.contains(&position) {
                            to_push.push(position);
                            to_visit.push_back((position + direction).unwrap());
                        }
                        let right = (position + Direction::Right).unwrap();
                        if !to_push.contains(&right) {
                            to_push.push(right);
                            to_visit.push_back((right + direction).unwrap());
                        }
                    }
                    Some(LargeTile::RightBox) => {
                        // Check that we haven't accounted for it. Otherwise, we'll be pushing
                        // the same tile twice (once full, once empty). Everything breaks.
                        if !to_push.contains(&position) {
                            to_push.push(position);
                            to_visit.push_back((position + direction).unwrap());
                        }
                        let left = (position + Direction::Left).unwrap();
                        if !to_push.contains(&left) {
                            to_push.push(left);
                            to_visit.push_back((left + direction).unwrap());
                        }
                    }
                    None => unreachable!("We should hit a wall first."),
                }
            }

            // If we haven't hit a wall, we're good to go.
            while let Some(position) = to_push.pop() {
                let current = *self.get(position).unwrap();
                *self.get_mut((position + direction).unwrap()).unwrap() = current;
                *self.get_mut(position).unwrap() = LargeTile::Empty;
            }

            true
        }
    }

    fn print(&self) -> Vec<Vec<char>> {
        self.rows()
            .map(|line| {
                line.map(|t| match t {
                    LargeTile::Wall => '#',
                    LargeTile::Empty => '.',
                    LargeTile::LeftBox => '[',
                    LargeTile::RightBox => ']',
                })
                .collect::<Vec<char>>()
            })
            .collect()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Robot {
    position: Position,
    directions: Vec<Direction>,
}

impl Robot {
    fn new(position: Position, directions: Vec<Direction>) -> Self {
        Self {
            position,
            directions,
        }
    }

    fn operate<T: Warehouse + Debug>(&mut self, warehouse: &mut T) -> &mut Self {
        for &direction in &self.directions {
            let next_position = (self.position + direction).unwrap();
            if warehouse.push(next_position, direction) {
                self.position = next_position;
            }
        }

        self
    }
}

fn part1(input: &str) -> usize {
    let (warehouse, directions) = input.split_once("\n\n").unwrap();

    let robot_position = warehouse
        .lines()
        .enumerate()
        .find_map(|(idy, line)| {
            line.chars()
                .position(|c| c == '@')
                .map(|idx| Position::new(idx, idy))
        })
        .unwrap();

    let height = warehouse.lines().count();
    let width = warehouse.lines().next().unwrap_or_default().len();
    let grid = warehouse
        .lines()
        .flat_map(|line| line.chars().map(Tile::try_from))
        .collect::<Result<Vec<Tile>, _>>()
        .unwrap();
    let mut warehouse = Grid::new(height, width, grid).unwrap();

    let directions = directions
        .lines()
        .flat_map(str::chars)
        .map(|c| match c {
            '^' => Direction::Up,
            '<' => Direction::Left,
            '>' => Direction::Right,
            'v' => Direction::Down,
            _ => panic!("Invalid character '{}'", c),
        })
        .collect::<Vec<_>>();
    let mut robot = Robot::new(robot_position, directions);

    robot.operate(&mut warehouse);
    warehouse.sum_gps_coordinates()
}

fn part2(input: &str) -> usize {
    let (warehouse, directions) = input.split_once("\n\n").unwrap();
    let warehouse = warehouse
        .lines()
        .map(|line| {
            line.chars()
                .flat_map(|c| {
                    match c {
                        '#' => "##",
                        '.' => "..",
                        '@' => "@.",
                        'O' => "[]",
                        _ => panic!("Invalid character {c}."),
                    }
                    .chars()
                })
                .chain(std::iter::once('\n'))
                .collect::<String>()
        })
        .collect::<String>();

    let robot_position = warehouse
        .lines()
        .enumerate()
        .find_map(|(idy, line)| {
            line.chars()
                .position(|c| c == '@')
                .map(|idx| Position::new(idx, idy))
        })
        .unwrap();

    let height = warehouse.lines().count();
    let width = warehouse.lines().next().unwrap_or_default().len();
    let grid = warehouse
        .lines()
        .flat_map(|line| line.chars().map(LargeTile::try_from))
        .collect::<Result<Vec<LargeTile>, _>>()
        .unwrap();
    let mut warehouse = Grid::new(height, width, grid).unwrap();

    let directions = directions
        .lines()
        .flat_map(str::chars)
        .map(|c| match c {
            '^' => Direction::Up,
            '<' => Direction::Left,
            '>' => Direction::Right,
            'v' => Direction::Down,
            _ => panic!("Invalid character '{}'", c),
        })
        .collect::<Vec<_>>();
    let mut robot = Robot::new(robot_position, directions);

    robot.operate(&mut warehouse);
    warehouse.sum_gps_coordinates()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT)?;

    println!("The first answer is: {}", part1(&input));
    println!("The second answer is: {}", part2(&input));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const SMALL_EXAMPLE: &str = "\
        ########\n\
        #..O.O.#\n\
        ##@.O..#\n\
        #...O..#\n\
        #.#.O..#\n\
        #...O..#\n\
        #......#\n\
        ########\n\
        \n\
        <^^>>>vv<v>>v<<\n\
    ";

    const LARGE_EXAMPLE: &str = "\
        ##########\n\
        #..O..O.O#\n\
        #......O.#\n\
        #.OO..O.O#\n\
        #..O@..O.#\n\
        #O#..O...#\n\
        #O..O..O.#\n\
        #.OO.O.OO#\n\
        #....O...#\n\
        ##########\n\
        \n\
        <vv>^<v^>v>^vv^v>v<>v^v<v<^vv<<<^><<><>>v<vvv<>^v^>^<<<><<v<<<v^vv^v>^\n\
        vvv<<^>^v^^><<>>><>^<<><^vv^^<>vvv<>><^^v>^>vv<>v<<<<v<^v>^<^^>>>^<v<v\n\
        ><>vv>v^v^<>><>>>><^^>vv>v<^^^>>v^v^<^^>v^^>v^<^v>v<>>v^v^<v>v^^<^^vv<\n\
        <<v<^>>^^^^>>>v^<>vvv^><v<<<>^^^vv^<vvv>^>v<^^^^v<>^>vvvv><>>v^<<^^^^^\n\
        ^><^><>>><>^^<<^^v>>><^<v>^<vv>>v>>>^v><>^v><<<<v>>v<v<v>vvv>^<><<>^><\n\
        ^>><>^v<><^vvv<^^<><v<<<<<><^v<<<><<<^^<v<^^^><^>>^<v^><<<^>>^v<v^v<v^\n\
        >^>>^v>vv>^<<^v<>><<><<v<<v><>v<^vv<<<>^^v^>^^>>><<^v>>v^v><^^>>^<>vv^\n\
        <><^^>^^^<><vvvvv^v<v<<>^v<v>v<<^><<><<><<<^^<<<^<<>><<><^^^>^^<>^>v<>\n\
        ^^>vv<^v^v<vv>^<><v<^v>^^^>>>^^vvv^>vvv<>>>^<^>>>>>^<<^v>^vvv<>^<><<v>\n\
        v^^>>><<^^<>>^v^<v^vv<>v^<<>^<^v^v><^<<<><<^<v><v<>vv>>v><v^<vv<>v^<<^\n\
    ";

    const SMALL_EXAMPLE_2: &str = "\
        #######\n\
        #...#.#\n\
        #.....#\n\
        #..OO@#\n\
        #..O..#\n\
        #.....#\n\
        #######\n\
        \n\
        <vv<<^^<<^^\n\
    ";

    #[test]
    fn part1_small_example() {
        let actual = part1(SMALL_EXAMPLE);
        let expected = 2028;

        assert_eq!(expected, actual);
    }

    #[test]
    fn part1_large_example() {
        let actual = part1(LARGE_EXAMPLE);
        let expected = 10092;

        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_small_example() {
        let actual = part2(SMALL_EXAMPLE_2);
        let expected = 618;

        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_large_example() {
        let actual = part2(LARGE_EXAMPLE);
        let expected = 9021;

        assert_eq!(expected, actual);
    }
}
