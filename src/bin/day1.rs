use counter::Counter;

const INPUT: &str = "./input/day1.txt";

fn part1(left: &[u32], right: &[u32]) -> u32 {
    let mut left = left.to_vec();
    let mut right = right.to_vec();
    left.sort_unstable();
    right.sort_unstable();

    left.iter()
        .zip(right.iter())
        .map(|(&l, &r)| l.abs_diff(r))
        .sum()
}

fn part2(left: &[u32], right: &[u32]) -> u32 {
    let left: Counter<&u32, u32> = Counter::from_iter(left);
    let right: Counter<&u32, u32> = Counter::from_iter(right);

    left.iter()
        .filter_map(|(l, c)| right.get(l).map(|r| r * *l * c))
        .sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT)?;
    let (left, right): (Vec<u32>, Vec<u32>) = input
        .lines()
        .filter_map(|line| line.trim().split_once(' '))
        .map(|(left, right)| {
            (
                right.trim().parse::<u32>().expect("Not a number."),
                left.trim().parse::<u32>().expect("Not a number."),
            )
        })
        .unzip();

    println!("The first answer is: {}", part1(&left, &right));
    println!("The second answer is: {}", part2(&left, &right));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let left = vec![3, 4, 2, 1, 3, 3];
        let right = vec![4, 3, 5, 3, 9, 3];
        let actual = part1(&left, &right);
        let expected = 11;
        assert_eq!(expected, actual);
    }
    #[test]
    fn test_part2() {
        let left = vec![3, 4, 2, 1, 3, 3];
        let right = vec![4, 3, 5, 3, 9, 3];
        let actual = part2(&left, &right);
        let expected = 31;
        assert_eq!(expected, actual);
    }
}
