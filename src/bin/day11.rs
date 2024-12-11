use radixal::IntoDigits;
use std::collections::HashMap;
use std::num::ParseIntError;
use std::str::FromStr;

const INPUT: &str = "./input/day11.txt";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Stone(u64);

impl Stone {
    fn new(number: u64) -> Self {
        Self(number)
    }

    fn evolve(self) -> Box<dyn Iterator<Item = Self>> {
        if self.0 == 0 {
            Box::new(std::iter::once(Self::new(1)))
        } else {
            let mut iter = self.0.into_decimal_digits();
            if iter.len() % 2 == 0 {
                let l = iter.len();
                let left = iter.by_ref().take(l / 2).fold(0, |acc, d| acc * 10 + d);

                Box::new(
                    std::iter::once(Self::new(left))
                        .chain(std::iter::once(Self::new(iter.into_number()))),
                )
            } else {
                Box::new(std::iter::once(Self::new(self.0 * 2024)))
            }
        }
    }
}

impl FromStr for Stone {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        s.trim().parse().map(Self)
    }
}

fn count_evolve(stones: &[Stone], generations: usize) -> usize {
    // While 25 generations is doable in a direct manner, 75 generations takes a very long time.
    // We can use recursion and memoization to speed up the process, especially as 0 and 1 seem
    // to recur often.
    let mut memoizer = HashMap::new();
    fn recursive(
        stone: Stone,
        generations: usize,
        memoizer: &mut HashMap<(Stone, usize), usize>,
    ) -> usize {
        if generations == 0 {
            return 1;
        }

        if let Some(count) = memoizer.get(&(stone, generations)) {
            *count
        } else {
            let count = stone
                .evolve()
                .map(|s| recursive(s, generations - 1, memoizer))
                .sum();
            memoizer.insert((stone, generations), count);
            count
        }
    }

    stones
        .iter()
        .map(|stone| recursive(*stone, generations, &mut memoizer))
        .sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT)?;
    let stones = input
        .split_whitespace()
        .map(Stone::from_str)
        .collect::<Result<Vec<_>, _>>()?;

    println!("The first answer is: {}", count_evolve(&stones, 25));
    println!("The second answer is: {}", count_evolve(&stones, 75));

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const EXAMPLE: &str = "125 17";

    #[test]
    fn test_evolution() {
        let mut stones: Vec<Stone> = EXAMPLE
            .split_whitespace()
            .map(Stone::from_str)
            .collect::<Result<Vec<Stone>, _>>()
            .unwrap();
        assert_eq!(&vec![Stone(125), Stone(17)], &stones);
        stones = stones.into_iter().flat_map(Stone::evolve).collect();
        assert_eq!(&vec![Stone(253000), Stone(1), Stone(7)], &stones);
        stones = stones.into_iter().flat_map(Stone::evolve).collect();
        assert_eq!(
            &vec![Stone(253), Stone(0), Stone(2024), Stone(14168)],
            &stones
        );
        stones = stones.into_iter().flat_map(Stone::evolve).collect();
        assert_eq!(
            &vec![
                Stone(512072),
                Stone(1),
                Stone(20),
                Stone(24),
                Stone(28676032)
            ],
            &stones
        );
        stones = stones.into_iter().flat_map(Stone::evolve).collect();
        assert_eq!(
            &vec![
                Stone(512),
                Stone(72),
                Stone(2024),
                Stone(2),
                Stone(0),
                Stone(2),
                Stone(4),
                Stone(2867),
                Stone(6032)
            ],
            &stones
        );
    }

    #[test]
    fn test_6_generations() {
        let stones: Vec<Stone> = EXAMPLE
            .split_whitespace()
            .map(Stone::from_str)
            .collect::<Result<Vec<Stone>, _>>()
            .unwrap();
        let actual = count_evolve(&stones, 6);
        let expected = 22;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_25_generations() {
        let stones: Vec<Stone> = EXAMPLE
            .split_whitespace()
            .map(Stone::from_str)
            .collect::<Result<Vec<Stone>, _>>()
            .unwrap();
        let actual = count_evolve(&stones, 25);
        let expected = 55312;
        assert_eq!(expected, actual);
    }
}
