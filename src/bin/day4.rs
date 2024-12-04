use advent_of_code_2024::grid::{Direction, Grid, Position, DIRECTIONS};

const INPUT: &str = "./input/day4.txt";

fn part1(grid: &Grid<char>) -> usize {
    const TARGET: [char; 4] = ['X', 'M', 'A', 'S'];
    let mut count = 0;

    for idy in 0..grid.height() {
        for idx in 0..grid.width() {
            let position = Position::new(idx, idy);
            if grid.get(position) == Some(&'X') {
                for direction in DIRECTIONS {
                    let mut characters = grid.get_iter(position.line(direction));
                    if TARGET
                        .iter()
                        .all(|target| Some(target) == characters.next())
                    {
                        count += 1;
                    }
                }
            }
        }
    }

    count
}

fn part2(grid: &Grid<char>) -> usize {
    let mut count = 0;

    for idy in 0..grid.height() {
        for idx in 0..grid.width() {
            let position = Position::new(idx, idy);
            if grid.get(position) == Some(&'A') {
                let upleft = (position + Direction::UpLeft).and_then(|p| grid.get(p));
                let downright = (position + Direction::DownRight).and_then(|p| grid.get(p));

                match (upleft, downright) {
                    (Some(&'M'), Some(&'S')) | (Some(&'S'), Some(&'M')) => {
                        let downleft = (position + Direction::DownLeft).and_then(|p| grid.get(p));
                        let upright = (position + Direction::UpRight).and_then(|p| grid.get(p));
                        match (downleft, upright) {
                            (Some(&'M'), Some(&'S')) | (Some(&'S'), Some(&'M')) => count += 1,
                            _ => (),
                        }
                    }
                    _ => (),
                }
            }
        }
    }

    count
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT)?;
    let height = input.lines().count();
    let width = input.lines().next().unwrap_or_default().len();

    let grid = Grid::new(height, width, input.lines().flat_map(str::chars).collect()).unwrap();

    println!("The first answer is: {}", part1(&grid));
    println!("The second answer is: {}", part2(&grid));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        const SAMPLE: &str = "\
            MMMSXXMASM\n\
            MSAMXMSMSA\n\
            AMXSXMAAMM\n\
            MSAMASMSMX\n\
            XMASAMXAMM\n\
            XXAMMXXAMA\n\
            SMSMSASXSS\n\
            SAXAMASAAA\n\
            MAMMMXMMMM\n\
            MXMXAXMASX\n\
        ";
        let grid = Grid::new(10, 10, SAMPLE.lines().map(str::chars).flatten().collect()).unwrap();
        let actual = part1(&grid);
        let expected = 18;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2() {
        const SAMPLE: &str = "\
            MMMSXXMASM\n\
            MSAMXMSMSA\n\
            AMXSXMAAMM\n\
            MSAMASMSMX\n\
            XMASAMXAMM\n\
            XXAMMXXAMA\n\
            SMSMSASXSS\n\
            SAXAMASAAA\n\
            MAMMMXMMMM\n\
            MXMXAXMASX\n\
        ";
        let grid = Grid::new(10, 10, SAMPLE.lines().map(str::chars).flatten().collect()).unwrap();
        let actual = part2(&grid);
        let expected = 9;
        assert_eq!(expected, actual);
    }
}
