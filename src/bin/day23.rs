use std::collections::{HashMap, HashSet};

const INPUT: &str = "./input/day23.txt";

fn parse_input(input: &str) -> HashMap<&str, HashSet<&str>> {
    input.lines().fold(HashMap::new(), |mut acc, line| {
        let (a, b) = line.split_once('-').unwrap();
        acc.entry(a).or_default().insert(b);
        acc.entry(b).or_default().insert(a);
        acc
    })
}

fn part1(connections: &HashMap<&str, HashSet<&str>>) -> usize {
    connections
        .iter()
        .flat_map(|(key, values)| {
            values.iter().flat_map(move |value| {
                if value > key {
                    connections.get(value).map(move |thirds| {
                        thirds.intersection(values).filter_map(move |third| {
                            if third > value {
                                Some((key, value, third))
                            } else {
                                None
                            }
                        })
                    })
                } else {
                    None
                }
            })
        })
        .flatten()
        .filter(|(a, b, c)| a.starts_with('t') || b.starts_with('t') || c.starts_with('t'))
        .count()
}

fn part2(connections: &HashMap<&str, HashSet<&str>>) -> String {
    // This is simply a matter of finding the maximal clique. We'll use a greedy algorithm.
    connections
        .iter()
        .map(|(key, values)| {
            let mut clique = HashSet::new();
            clique.insert(*key);
            let mut to_visit = values.iter().collect::<Vec<&&str>>();
            let mut visited = HashSet::new();
            visited.insert(key);
            while let Some(computer) = to_visit.pop() {
                if visited.insert(computer) {
                    // Add a new computer if it has connections to all the already connected
                    // computers.
                    if let Some(new_connections) = connections.get(computer) {
                        if new_connections.is_superset(&clique) {
                            clique.insert(computer);
                            to_visit.extend(new_connections);
                        }
                    }
                }
            }
            clique
        })
        .max_by_key(HashSet::len)
        .map(|clique| {
            let mut clique = clique.into_iter().collect::<Vec<&str>>();
            clique.sort();
            clique.join(",")
        })
        .unwrap()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT)?;
    let connections = parse_input(&input);

    println!("The first answer is: {}", part1(&connections));
    println!("The first answer is: {}", part2(&connections));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        kh-tc\n\
        qp-kh\n\
        de-cg\n\
        ka-co\n\
        yn-aq\n\
        qp-ub\n\
        cg-tb\n\
        vc-aq\n\
        tb-ka\n\
        wh-tc\n\
        yn-cg\n\
        kh-ub\n\
        ta-co\n\
        de-co\n\
        tc-td\n\
        tb-wq\n\
        wh-td\n\
        ta-ka\n\
        td-qp\n\
        aq-cg\n\
        wq-ub\n\
        ub-vc\n\
        de-ta\n\
        wq-aq\n\
        wq-vc\n\
        wh-yn\n\
        ka-de\n\
        kh-ta\n\
        co-tc\n\
        wh-qp\n\
        tb-vc\n\
        td-yn\n\
    ";

    #[test]
    fn test_part1() {
        let connections = parse_input(EXAMPLE);
        let actual = part1(&connections);
        let expected = 7;

        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2() {
        let connections = parse_input(EXAMPLE);
        let actual = part2(&connections);
        let expected = "co,de,ka,ta";

        assert_eq!(expected, actual);
    }
}
