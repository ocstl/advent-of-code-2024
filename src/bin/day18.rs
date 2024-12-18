use advent_of_code_2024::grid::Position;
use std::cmp::Reverse;
use std::collections::{BinaryHeap, HashSet};

const INPUT: &str = "./input/day18.txt";
const MAX_DIMENSION: usize = 70;
const START: Position = Position::new(0, 0);
const END: Position = Position::new(MAX_DIMENSION, MAX_DIMENSION);
const KILOBYTE: usize = 1024;

fn shortest_path<const MAX_DIMENSION: usize>(
    start: Position,
    end: Position,
    falling_bytes: &[Position],
) -> Option<usize> {
    let mut falling_bytes: HashSet<Position> = HashSet::from_iter(falling_bytes.iter().cloned());

    let mut to_visit = BinaryHeap::new();
    to_visit.push((Reverse(0), start));
    falling_bytes.insert(start);

    while let Some((Reverse(steps), position)) = to_visit.pop() {
        if position == end {
            return Some(steps);
        }

        to_visit.extend(position.neighbours().filter_map(|p| {
            if p.x() <= MAX_DIMENSION && p.y() <= MAX_DIMENSION && falling_bytes.insert(p) {
                Some((Reverse(steps + 1), p))
            } else {
                None
            }
        }));
    }

    None
}

fn part2<const MAX_DIMENSION: usize>(
    start: Position,
    end: Position,
    falling_bytes: &[Position],
) -> Position {
    // We're reusing the BFS from part 1, but it would be probably be more efficient to use DFS.
    (0..falling_bytes.len())
        .find_map(|idx| {
            if shortest_path::<MAX_DIMENSION>(start, end, &falling_bytes[0..=idx]).is_none() {
                Some(falling_bytes[idx])
            } else {
                None
            }
        })
        .unwrap()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT)?;
    let falling_bytes: Vec<Position> = input
        .lines()
        .filter_map(|line| {
            line.split_once(',')
                .map(|(x, y)| Position::new(x.parse().unwrap(), y.parse().unwrap()))
        })
        .collect();

    println!(
        "The first answer is: {}",
        shortest_path::<MAX_DIMENSION>(START, END, &falling_bytes[..KILOBYTE]).unwrap()
    );
    println!(
        "The second answer is: {}",
        part2::<MAX_DIMENSION>(START, END, &falling_bytes)
    );

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        5,4\n\
        4,2\n\
        4,5\n\
        3,0\n\
        2,1\n\
        6,3\n\
        2,4\n\
        1,5\n\
        0,6\n\
        3,3\n\
        2,6\n\
        5,1\n\
        1,2\n\
        5,5\n\
        2,5\n\
        6,5\n\
        1,4\n\
        0,4\n\
        6,4\n\
        1,1\n\
        6,1\n\
        1,0\n\
        0,5\n\
        1,6\n\
        2,0\n\
    ";

    #[test]
    fn test_part1() {
        let falling_bytes: Vec<Position> = EXAMPLE
            .lines()
            .filter_map(|line| {
                line.split_once(',')
                    .map(|(x, y)| Position::new(x.parse().unwrap(), y.parse().unwrap()))
            })
            .collect();

        const TEST_DIMENSION: usize = 6;
        let start = Position::new(0, 0);
        let end = Position::new(TEST_DIMENSION, TEST_DIMENSION);

        let actual = shortest_path::<TEST_DIMENSION>(start, end, &falling_bytes[..12]);
        let expected = Some(22);

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2() {
        let falling_bytes: Vec<Position> = EXAMPLE
            .lines()
            .filter_map(|line| {
                line.split_once(',')
                    .map(|(x, y)| Position::new(x.parse().unwrap(), y.parse().unwrap()))
            })
            .collect();

        const TEST_DIMENSION: usize = 6;
        let start = Position::new(0, 0);
        let end = Position::new(TEST_DIMENSION, TEST_DIMENSION);

        let actual = part2::<TEST_DIMENSION>(start, end, &falling_bytes);
        let expected = Position::new(6, 1);

        assert_eq!(expected, actual);
    }
}
