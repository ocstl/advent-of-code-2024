use advent_of_code_2024::grid::{Grid, Position};
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashMap, HashSet};
use std::str::FromStr;

const INPUT: &str = "./input/day20.txt";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Start,
    End,
    Wall,
    Track,
}

impl TryFrom<char> for Tile {
    type Error = String;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'S' => Ok(Tile::Start),
            'E' => Ok(Tile::End),
            '#' => Ok(Tile::Wall),
            '.' => Ok(Tile::Track),
            _ => Err(format!("Invalid tile character: {value}")),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Racetrack {
    track: Grid<Tile>,
    start: Position,
    end: Position,
}

impl Racetrack {
    fn shortest_path(&self) -> Option<usize> {
        let mut to_visit = BinaryHeap::new();
        let mut visited = HashSet::new();
        to_visit.push((Reverse(0), self.start));
        visited.insert(self.start);

        while let Some((Reverse(picoseconds), pos)) = to_visit.pop() {
            match self.track.get(pos) {
                Some(&Tile::End) => return Some(picoseconds),
                Some(&Tile::Wall) | None => (),
                Some(&Tile::Track) | Some(&Tile::Start) => {
                    to_visit.extend(pos.neighbours().filter_map(|neighbor| {
                        if visited.insert(neighbor) {
                            Some((Reverse(picoseconds + 1), neighbor))
                        } else {
                            None
                        }
                    }))
                }
            }
        }

        None
    }

    fn cheats(
        &self,
        shortest_time: usize,
        cheat_length: usize,
    ) -> HashMap<(Position, Position), usize> {
        // Precompute the shortest path to the end.
        let mut shortest_paths = HashMap::new();
        let mut to_visit = BinaryHeap::new();
        to_visit.push((Reverse(0), self.end));

        while let Some((Reverse(picoseconds), pos)) = to_visit.pop() {
            match self.track.get(pos) {
                Some(&Tile::Track) | Some(&Tile::End) | Some(&Tile::Start) => {
                    if !shortest_paths.contains_key(&pos) {
                        shortest_paths.entry(pos).or_insert(picoseconds);
                        to_visit.extend(
                            pos.neighbours()
                                .map(|neighbor| (Reverse(picoseconds + 1), neighbor)),
                        );
                    }
                }
                Some(&Tile::Wall) | None => (),
            }
        }

        // Precompute the shortest distance from the start.
        let mut shortest_distances = HashMap::new();
        let mut to_visit = BinaryHeap::new();
        to_visit.push((Reverse(0), self.start));

        while let Some((Reverse(picoseconds), pos)) = to_visit.pop() {
            match self.track.get(pos) {
                Some(&Tile::Start) | Some(&Tile::Track) | Some(&Tile::End) => {
                    if !shortest_distances.contains_key(&pos) {
                        shortest_distances.entry(pos).or_insert(picoseconds);
                        to_visit.extend(
                            pos.neighbours()
                                .map(|neighbor| (Reverse(picoseconds + 1), neighbor)),
                        );
                    }
                }
                Some(&Tile::Wall) | None => (),
            }
        }

        // Now, compare every reachable beginning and ending points for the cheats, keeping only:
        // - that are short enough (beginning end ending points are at most the given picoseconds
        //   apart.
        // - that actually shorten the path below the non-cheating shortest path (but do include
        //   the distance traveled while cheating!).
        shortest_distances
            .iter()
            .flat_map(|(&start, &distance)| {
                shortest_paths.iter().filter_map(move |(&end, &path)| {
                    shortest_time
                        .checked_sub(path + distance + end.manhattan_distance(start))
                        .filter(|c| *c > 0 && end.manhattan_distance(start) <= cheat_length)
                        .map(|c| ((start, end), c))
                })
            })
            .collect()
    }
}

impl FromStr for Racetrack {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let height = s.lines().count();
        let width = s.lines().next().unwrap_or_default().len();
        let track = s
            .lines()
            .flat_map(|line| line.chars().map(Tile::try_from))
            .collect::<Result<Vec<Tile>, _>>()?;

        let start = track
            .iter()
            .enumerate()
            .find_map(|(pos, &tile)| {
                if tile == Tile::Start {
                    Some(Position::new(pos % width, pos / width))
                } else {
                    None
                }
            })
            .unwrap();
        let end = track
            .iter()
            .enumerate()
            .find_map(|(pos, &tile)| {
                if tile == Tile::End {
                    Some(Position::new(pos % width, pos / width))
                } else {
                    None
                }
            })
            .unwrap();
        let track = Grid::new(height, width, track).unwrap();

        Ok(Self { track, start, end })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT)?;
    let racetrack = input.parse::<Racetrack>()?;

    let shortest_time = racetrack.shortest_path().unwrap();
    println!(
        "The first answer is: {}",
        racetrack
            .cheats(shortest_time, 2)
            .into_iter()
            .filter(|(_, picoseconds)| *picoseconds >= 100)
            .count()
    );
    println!(
        "The first answer is: {}",
        racetrack
            .cheats(shortest_time, 20)
            .into_iter()
            .filter(|(_, picoseconds)| *picoseconds >= 100)
            .count()
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        ###############\n\
        #...#...#.....#\n\
        #.#.#.#.#.###.#\n\
        #S#...#.#.#...#\n\
        #######.#.#.###\n\
        #######.#.#...#\n\
        #######.#.###.#\n\
        ###..E#...#...#\n\
        ###.#######.###\n\
        #...###...#...#\n\
        #.#####.#.###.#\n\
        #.#...#.#.#...#\n\
        #.#.#.#.#.#.###\n\
        #...#...#...###\n\
        ###############\n\
    ";

    #[test]
    fn test_part1() {
        let racetrack = EXAMPLE.parse::<Racetrack>().unwrap();

        let shortest_path = racetrack.shortest_path().unwrap();
        let expected = 84;
        assert_eq!(expected, shortest_path);

        let mut actual: Vec<usize> = racetrack.cheats(shortest_path, 2).into_values().collect();
        actual.sort_unstable();
        let expected = vec![
            2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 2, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 4, 6,
            6, 8, 8, 8, 8, 10, 10, 12, 12, 12, 20, 36, 38, 40, 64,
        ];

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2() {
        let racetrack = EXAMPLE.parse::<Racetrack>().unwrap();

        let shortest_path = racetrack.shortest_path().unwrap();
        let expected = 84;
        assert_eq!(expected, shortest_path);

        let actual: usize = racetrack
            .cheats(shortest_path, 20)
            .into_values()
            .filter(|c| *c >= 50)
            .sum();
        let expected = 16940;

        assert_eq!(expected, actual);
    }
}
