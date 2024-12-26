const INPUT: &str = "./input/day25.txt";
const HEIGHT: usize = 6;
const FILLED: char = '#';

type Lock = [usize; 5];
type Key = [usize; 5];

fn fit(lock: Lock, key: Key) -> bool {
    lock.into_iter().zip(key).all(|(l, k)| l + k < HEIGHT)
}

fn parse_input(input: &str) -> (Vec<Lock>, Vec<Key>) {
    let mut locks = Vec::new();
    let mut keys = Vec::new();

    for schematic in input.split("\n\n") {
        let mut lines = schematic.lines();
        if lines.next().unwrap().chars().all(|c| c == FILLED) {
            let lock = lines.take(5).fold(Lock::default(), |mut lock, row| {
                for (tumbler, filling) in lock.iter_mut().zip(row.chars()) {
                    if filling == FILLED {
                        *tumbler += 1;
                    }
                }
                lock
            });
            locks.push(lock);
        } else {
            let key = lines.take(5).fold(Key::default(), |mut key, row| {
                for (tumbler, filling) in key.iter_mut().zip(row.chars()) {
                    if filling == FILLED {
                        *tumbler += 1;
                    }
                }
                key
            });
            keys.push(key);
        }
    }

    (locks, keys)
}

fn part1(locks: &[Lock], keys: &[Key]) -> usize {
    locks
        .iter()
        .flat_map(|&lock| keys.iter().filter(move |&&key| fit(lock, key)))
        .count()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT)?;
    let (locks, keys) = parse_input(&input);

    println!("The first answer is: {}", part1(&locks, &keys));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        #####\n\
        .####\n\
        .####\n\
        .####\n\
        .#.#.\n\
        .#...\n\
        .....\n\
        \n\
        #####\n\
        ##.##\n\
        .#.##\n\
        ...##\n\
        ...#.\n\
        ...#.\n\
        .....\n\
        \n\
        .....\n\
        #....\n\
        #....\n\
        #...#\n\
        #.#.#\n\
        #.###\n\
        #####\n\
        \n\
        .....\n\
        .....\n\
        #.#..\n\
        ###..\n\
        ###.#\n\
        ###.#\n\
        #####\n\
        \n\
        .....\n\
        .....\n\
        .....\n\
        #....\n\
        #.#..\n\
        #.#.#\n\
        #####\n\
        ";

    #[test]
    fn test_part1() {
        let (locks, keys) = parse_input(EXAMPLE);
        let actual = part1(&locks, &keys);
        let expected = 3;

        assert_eq!(expected, actual);
    }
}
