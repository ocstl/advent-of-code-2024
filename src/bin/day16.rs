use advent_of_code_2024::grid::{Direction, Grid, Position};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::str::FromStr;

const INPUT: &str = "./input/day16.txt";
const STEP_COST: u32 = 1;
const TURN_COST: u32 = 1000;
const START_DIRECTION: Direction = Direction::Right;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Wall,
    Open,
    Start,
    End,
}

impl TryFrom<char> for Tile {
    type Error = char;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '#' => Ok(Tile::Wall),
            '.' => Ok(Tile::Open),
            'S' => Ok(Tile::Start),
            'E' => Ok(Tile::End),
            _ => Err(value),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Maze(Grid<Tile>);

impl Maze {
    fn best_score(&self) -> u32 {
        let start_position = self
            .0
            .iter()
            .find_map(|(p, &t)| if t == Tile::Start { Some(p) } else { None })
            .unwrap();
        let mut to_visit = std::iter::once((Reverse(0), start_position, START_DIRECTION))
            .collect::<BinaryHeap<_>>();
        let mut visited = HashSet::new();

        while let Some((Reverse(score), position, direction)) = to_visit.pop() {
            if visited.insert((position, direction)) {
                match self.0.get(position) {
                    Some(Tile::Wall) => (),
                    Some(Tile::End) => return score,
                    Some(Tile::Start) | Some(Tile::Open) => {
                        // Step forward.
                        to_visit.push((
                            Reverse(score + STEP_COST),
                            (position + direction).unwrap(),
                            direction,
                        ));
                        // Turn left and right.
                        to_visit.push((
                            Reverse(score + TURN_COST),
                            position,
                            direction.rotate_left(),
                        ));
                        to_visit.push((
                            Reverse(score + TURN_COST),
                            position,
                            direction.rotate_right(),
                        ));
                    }
                    None => unreachable!("The maze is surrounded by walls."),
                }
            }
        }

        0
    }

    fn best_paths(&self) -> Vec<Vec<(Position, Direction)>> {
        let best_score = self.best_score();
        let start_position = self
            .0
            .iter()
            .find_map(|(p, &t)| if t == Tile::Start { Some(p) } else { None })
            .unwrap();
        let mut to_visit = BinaryHeap::new();
        to_visit.push((Reverse(0), vec![(start_position, START_DIRECTION)]));
        let mut visited = HashMap::new();
        let mut best_paths = Vec::new();

        while let Some((Reverse(score), path)) = to_visit.pop() {
            // Once we've reached past the best score, there is no point in continuing.
            if score > best_score {
                return best_paths;
            }

            let (position, direction) = *path.last().unwrap();
            // Keep track of the lowest score needed to reach a position-direction pair.
            // Any best path shouldn't have a higher score, but two best paths could share it.
            if *visited.entry((position, direction)).or_insert(score) == score {
                match self.0.get(position) {
                    Some(Tile::Wall) => (),
                    Some(Tile::End) => best_paths.push(path),
                    Some(Tile::Start) | Some(Tile::Open) => {
                        // Using the best score allows us to cut down on the size of the queue,
                        // but it might makes sense to cut down more (checking for walls here for
                        // example).
                        if score + STEP_COST <= best_score {
                            let new_path = path
                                .iter()
                                .copied()
                                .chain(std::iter::once((
                                    (position + direction).unwrap(),
                                    direction,
                                )))
                                .collect();
                            to_visit.push((Reverse(score + STEP_COST), new_path));
                        }
                        // Turn left and right.
                        if score + TURN_COST <= best_score {
                            let new_path = path
                                .iter()
                                .copied()
                                .chain(std::iter::once((position, direction.rotate_left())))
                                .collect();
                            to_visit.push((Reverse(score + TURN_COST), new_path));
                            let new_path = path
                                .iter()
                                .copied()
                                .chain(std::iter::once((position, direction.rotate_right())))
                                .collect();
                            to_visit.push((Reverse(score + TURN_COST), new_path));
                        }
                    }
                    None => unreachable!("The maze is surrounded by walls."),
                }
            }
        }

        best_paths
    }

    fn best_spots(&self) -> usize {
        self.best_paths()
            .into_iter()
            .flat_map(|path| path.into_iter().map(|(p, _)| p))
            .collect::<HashSet<Position>>()
            .len()
    }
}

impl FromStr for Maze {
    type Err = char;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height = s.lines().count();
        let width = s.lines().next().unwrap_or_default().len();
        let grid = s
            .lines()
            .flat_map(|line| line.chars())
            .map(Tile::try_from)
            .collect::<Result<Vec<Tile>, _>>()?;

        Ok(Self(Grid::new(height, width, grid).unwrap()))
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT)?;
    let maze: Maze = input.parse::<Maze>().unwrap();

    println!("The first answer is: {}", maze.best_score());
    println!("The second answer is: {}", maze.best_spots());

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE_1: &str = "\
        ###############\n\
        #.......#....E#\n\
        #.#.###.#.###.#\n\
        #.....#.#...#.#\n\
        #.###.#####.#.#\n\
        #.#.#.......#.#\n\
        #.#.#####.###.#\n\
        #...........#.#\n\
        ###.#.#####.#.#\n\
        #...#.....#.#.#\n\
        #.#.#.###.#.#.#\n\
        #.....#...#.#.#\n\
        #.###.#.#.#.#.#\n\
        #S..#.....#...#\n\
        ###############\n\
        ";

    const EXAMPLE_2: &str = "\
        #################\n\
        #...#...#...#..E#\n\
        #.#.#.#.#.#.#.#.#\n\
        #.#.#.#...#...#.#\n\
        #.#.#.#.###.#.#.#\n\
        #...#.#.#.....#.#\n\
        #.#.#.#.#.#####.#\n\
        #.#...#.#.#.....#\n\
        #.#.#####.#.###.#\n\
        #.#.#.......#...#\n\
        #.#.###.#####.###\n\
        #.#.#...#.....#.#\n\
        #.#.#.#####.###.#\n\
        #.#.#.........#.#\n\
        #.#.#.#########.#\n\
        #S#.............#\n\
        #################\n\
        ";

    #[test]
    fn part1_example1() {
        let maze = EXAMPLE_1.parse::<Maze>().unwrap();
        let actual = maze.best_score();
        let expected = 7036;

        assert_eq!(expected, actual);
    }

    #[test]
    fn part1_example2() {
        let maze = EXAMPLE_2.parse::<Maze>().unwrap();
        let actual = maze.best_score();
        let expected = 11048;

        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_example1() {
        let maze = EXAMPLE_1.parse::<Maze>().unwrap();
        let actual = maze.best_spots();
        let expected = 45;

        assert_eq!(expected, actual);
    }

    #[test]
    fn part2_example2() {
        let maze = EXAMPLE_2.parse::<Maze>().unwrap();
        let actual = maze.best_spots();
        let expected = 64;

        assert_eq!(expected, actual);
    }
}
