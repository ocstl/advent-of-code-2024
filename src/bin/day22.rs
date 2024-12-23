use std::collections::HashMap;
use std::hash::Hash;

const INPUT: &str = "./input/day22.txt";

const LEFT_SHIFT: u32 = 6;
const RIGHT_SHIFT: u32 = 5;
const MULTIPLIER: u32 = 11;
const MODULO: u64 = 16777216;

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, Hash)]
struct SecretNumbers(u64);

impl SecretNumbers {
    fn new(initial: u64) -> Self {
        Self(initial)
    }
}

impl Iterator for SecretNumbers {
    type Item = u64;

    fn next(&mut self) -> Option<Self::Item> {
        let current = self.0;
        self.0 = (self.0 ^ self.0 << LEFT_SHIFT) % MODULO;
        self.0 = (self.0 ^ self.0 >> RIGHT_SHIFT) % MODULO;
        self.0 = (self.0 ^ self.0 << MULTIPLIER) % MODULO;
        Some(current)
    }
}

fn part2(buyers: &[SecretNumbers]) -> u64 {
    let mut counter: HashMap<[i32; 4], u64> = HashMap::new();
    for buyer in buyers {
        let mut current: HashMap<[i32; 4], u64> = HashMap::new();
        // Take 2001 prices, not 2000, because we need the initial one for the changes.
        let prices: Vec<u64> = (*buyer).take(2001).map(|n| n % 10).collect();
        let changes = prices
            .windows(2)
            .map(|p| p[1] as i32 - p[0] as i32)
            .collect::<Vec<_>>();
        for (c, p) in changes.windows(4).zip(prices.iter().skip(4)) {
            // The monkey can only sell once, the first time he sees the changes.
            current.entry(c.try_into().unwrap()).or_insert(*p);
        }

        for (c, p) in current {
            *counter.entry(c).or_default() += p;
        }
    }

    counter.into_values().max().unwrap()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT)?;
    let buyers: Vec<SecretNumbers> = input
        .lines()
        .map(|line| SecretNumbers::new(line.parse().unwrap()))
        .collect();

    println!(
        "The first answer is: {}",
        buyers
            .iter()
            .filter_map(|buyer| buyer.clone().nth(2000))
            .sum::<u64>()
    );
    println!("The second answer is: {}", part2(&buyers));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
    1\n\
    10\n\
    100\n\
    2024\n\
    ";

    #[test]
    fn test_secret_numbers() {
        let secret = SecretNumbers::new(123);
        let actual: Vec<u64> = secret.take(11).collect();
        let expected = vec![
            123, 15887950, 16495136, 527345, 704524, 1553684, 12683156, 11100544, 12249484,
            7753432, 5908254,
        ];

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part1() {
        let buyers = EXAMPLE
            .lines()
            .map(|line| SecretNumbers::new(line.parse::<u64>().unwrap()));
        let actual = buyers.filter_map(|mut buyer| buyer.nth(2000)).sum();
        let expected: u64 = 37327623;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2() {
        let buyers = vec![
            SecretNumbers::new(1),
            SecretNumbers::new(2),
            SecretNumbers::new(3),
            SecretNumbers::new(2024),
        ];
        let actual = part2(&buyers);
        let expected: u64 = 23;

        assert_eq!(expected, actual);
    }
}
