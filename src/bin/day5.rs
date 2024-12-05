use std::collections::{HashMap, HashSet};

const INPUT: &str = "./input/day5.txt";

type Page = u32;
type Rules = HashMap<Page, HashSet<Page>>;
type Order = Vec<Page>;

trait PageOrdering {
    fn is_ordered(&self, rules: &Rules) -> bool;
}
impl PageOrdering for Order {
    fn is_ordered(&self, rules: &Rules) -> bool {
        let mut seen = HashSet::new();
        self.iter().all(|page| {
            seen.insert(*page);
            rules.get(page).map_or(true, |pre| pre.is_disjoint(&seen))
        })
    }
}

fn parse_input(input: &str) -> (Rules, Vec<Order>) {
    let (list_rules, list_orders) = input.split_once("\n\n").unwrap();

    let mut rules: Rules = HashMap::new();
    for rule in list_rules.lines() {
        if let Some((pre, post)) = rule.split_once('|') {
            let pre = pre.parse::<Page>().unwrap();
            let post = post.parse::<Page>().unwrap();
            rules.entry(pre).or_default().insert(post);
        }
    }

    let orders = list_orders
        .lines()
        .map(|line| {
            line.split(',')
                .map(|page| page.parse::<Page>().unwrap())
                .collect()
        })
        .collect();

    (rules, orders)
}

fn part1(rules: &Rules, orders: &[Order]) -> Page {
    orders
        .iter()
        .filter(|order| order.is_ordered(rules))
        .map(|order| order[order.len() / 2])
        .sum()
}

fn part2(rules: &Rules, orders: &[Order]) -> Page {
    orders
        .iter()
        .filter(|order| !order.is_ordered(rules))
        .map(|order| {
            let mut ordered = Vec::new();
            // For each page, find the first offending page that's out of order, and insert the
            // new page right before it. If there isn't any, put the new page at the end.
            for page in order {
                if let Some(idx) = rules
                    .get(page)
                    .and_then(|rule| ordered.iter().position(|p| rule.contains(p)))
                {
                    ordered.insert(idx, *page);
                } else {
                    ordered.push(*page);
                }
            }

            ordered[ordered.len() / 2]
        })
        .sum()
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT)?;
    let (rules, orders) = parse_input(&input);

    println!("The first answer is: {}", part1(&rules, &orders));
    println!("The second answer is: {}", part2(&rules, &orders));

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const SAMPLE: &str = "\
        47|53\n\
        97|13\n\
        97|61\n\
        97|47\n\
        75|29\n\
        61|13\n\
        75|53\n\
        29|13\n\
        97|29\n\
        53|29\n\
        61|53\n\
        97|53\n\
        61|29\n\
        47|13\n\
        75|47\n\
        97|75\n\
        47|61\n\
        75|61\n\
        47|29\n\
        75|13\n\
        53|13\n\
        \n\
        75,47,61,53,29\n\
        97,61,53,29,13\n\
        75,29,13\n\
        75,97,47,61,53\n\
        61,13,29\n\
        97,13,75,29,47\n\
    ";

    #[test]
    fn test_part1() {
        let (rules, orders) = parse_input(SAMPLE);
        let actual = part1(&rules, &orders);
        let expected = 143;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2() {
        let (rules, orders) = parse_input(SAMPLE);
        let actual = part2(&rules, &orders);
        let expected = 123;
        assert_eq!(expected, actual);
    }
}
