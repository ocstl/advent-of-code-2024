use advent_of_code_2024::grid::Position;
use counter::Counter;
use std::cmp::Ordering;
use std::fmt::Debug;

const INPUT: &str = "./input/day21.txt";

#[derive(Debug, Clone, Default, Copy, PartialEq, Eq, Hash)]
enum NumericButton {
    #[default]
    Activate,
    Button0,
    Button1,
    Button2,
    Button3,
    Button4,
    Button5,
    Button6,
    Button7,
    Button8,
    Button9,
}

impl NumericButton {
    fn to(self, other: NumericButton) -> Vec<DirectionalButton> {
        // A few considerations:
        // - first keypad:
        //   - it is better to move in a straight line then to zigzag, so we can press the same
        //     button repeatedly instead of moving around (at the first directional keypad).
        //   - reaching the left button is the most expensive, followed by the down button.
        //   - we always end up at the 'Activate' button at the end.
        // - second keypad:
        //   - it is better to move in a straight line than to zigzag (again). For pretty much any
        //     direction, the order doesn't matter, but... If we need to move left, we may need to
        //     move up as well; if we do up first, we miss our chance to do a 2 step to the left,
        //     so we should prefer left first. Otherwise, we end at 'up' then zigzag.
        //   - I'm less sure about down, but it may be similar.
        // So, we prefer left, then down, then up, then right. Unless we have to cross the empty
        // space, at which point we prefer avoiding the zigzag.
        let start = Position::from(self);
        let end = Position::from(other);
        let mut directions = Vec::new();

        match end.x().cmp(&start.x()) {
            Ordering::Less => directions.extend(std::iter::repeat_n(
                DirectionalButton::Left,
                start.x() - end.x(),
            )),
            Ordering::Equal => (),
            Ordering::Greater => directions.extend(std::iter::repeat_n(
                DirectionalButton::Right,
                end.x() - start.x(),
            )),
        };

        match end.y().cmp(&start.y()) {
            Ordering::Less => directions.extend(std::iter::repeat_n(
                DirectionalButton::Up,
                start.y() - end.y(),
            )),
            Ordering::Equal => (),
            Ordering::Greater => directions.extend(std::iter::repeat_n(
                DirectionalButton::Down,
                end.y() - start.y(),
            )),
        };

        // We've set up the ordering for the DirectionButton enum.
        directions.sort_unstable();

        // If we are dealing with '0' or 'A', and  '1', '4', '7', we need to reverse it to avoid
        // the blank space.
        if (self == NumericButton::Button0 || self == NumericButton::Activate)
            && (other == NumericButton::Button7
                || other == NumericButton::Button4
                || other == NumericButton::Button1)
        {
            directions.reverse();
        }
        if (other == NumericButton::Button0 || other == NumericButton::Activate)
            && (self == NumericButton::Button7
                || self == NumericButton::Button4
                || self == NumericButton::Button1)
        {
            directions.reverse();
        }

        directions
    }
}

impl TryFrom<char> for NumericButton {
    type Error = char;

    fn try_from(c: char) -> Result<Self, Self::Error> {
        match c {
            '0' => Ok(NumericButton::Button0),
            '1' => Ok(NumericButton::Button1),
            '2' => Ok(NumericButton::Button2),
            '3' => Ok(NumericButton::Button3),
            '4' => Ok(NumericButton::Button4),
            '5' => Ok(NumericButton::Button5),
            '6' => Ok(NumericButton::Button6),
            '7' => Ok(NumericButton::Button7),
            '8' => Ok(NumericButton::Button8),
            '9' => Ok(NumericButton::Button9),
            'A' => Ok(NumericButton::Activate),
            _ => Err(c),
        }
    }
}

impl From<NumericButton> for Position {
    fn from(value: NumericButton) -> Position {
        match value {
            NumericButton::Button7 => Position::new(0, 0),
            NumericButton::Button8 => Position::new(1, 0),
            NumericButton::Button9 => Position::new(2, 0),
            NumericButton::Button4 => Position::new(0, 1),
            NumericButton::Button5 => Position::new(1, 1),
            NumericButton::Button6 => Position::new(2, 1),
            NumericButton::Button1 => Position::new(0, 2),
            NumericButton::Button2 => Position::new(1, 2),
            NumericButton::Button3 => Position::new(2, 2),
            NumericButton::Button0 => Position::new(1, 3),
            NumericButton::Activate => Position::new(2, 3),
        }
    }
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
enum DirectionalButton {
    Left,
    Down,
    Up,
    Right,
    #[default]
    Activate,
}

impl DirectionalButton {
    fn to(self, other: Self) -> Vec<DirectionalButton> {
        match self {
            DirectionalButton::Left => match other {
                DirectionalButton::Left => vec![],
                DirectionalButton::Down => vec![DirectionalButton::Right],
                DirectionalButton::Up => vec![DirectionalButton::Right, DirectionalButton::Up],
                DirectionalButton::Right => {
                    vec![DirectionalButton::Right, DirectionalButton::Right]
                }
                DirectionalButton::Activate => vec![
                    DirectionalButton::Right,
                    DirectionalButton::Right,
                    DirectionalButton::Up,
                ],
            },
            DirectionalButton::Down => match other {
                DirectionalButton::Left => vec![DirectionalButton::Left],
                DirectionalButton::Down => vec![],
                DirectionalButton::Up => vec![DirectionalButton::Up],
                DirectionalButton::Right => vec![DirectionalButton::Right],
                DirectionalButton::Activate => {
                    vec![DirectionalButton::Up, DirectionalButton::Right]
                }
            },
            DirectionalButton::Up => match other {
                DirectionalButton::Left => vec![DirectionalButton::Down, DirectionalButton::Left],
                DirectionalButton::Down => vec![DirectionalButton::Down],
                DirectionalButton::Up => vec![],
                DirectionalButton::Right => vec![DirectionalButton::Down, DirectionalButton::Right],
                DirectionalButton::Activate => vec![DirectionalButton::Right],
            },
            DirectionalButton::Right => match other {
                DirectionalButton::Left => vec![DirectionalButton::Left, DirectionalButton::Left],
                DirectionalButton::Down => vec![DirectionalButton::Left, DirectionalButton::Down],
                DirectionalButton::Up => vec![DirectionalButton::Left, DirectionalButton::Up],
                DirectionalButton::Right => vec![],
                DirectionalButton::Activate => vec![DirectionalButton::Up],
            },
            DirectionalButton::Activate => match other {
                DirectionalButton::Left => vec![
                    DirectionalButton::Down,
                    DirectionalButton::Left,
                    DirectionalButton::Left,
                ],
                DirectionalButton::Down => vec![DirectionalButton::Left, DirectionalButton::Down],
                DirectionalButton::Up => vec![DirectionalButton::Left],
                DirectionalButton::Right => vec![DirectionalButton::Down],
                DirectionalButton::Activate => vec![],
            },
        }
    }
}

trait Code {
    fn shortest_sequence(&self, nbr_intermediaries: usize) -> usize;
    fn complexity(&self, nbr_intermediaries: usize) -> usize;
}

impl<T: AsRef<str> + Debug> Code for T {
    fn shortest_sequence(&self, nbr_intermediaries: usize) -> usize {
        // The first robot on a directional keypad can press the same key repeatedly. But, the
        // robot operating its keypad will always return (thus start) on the `Activate` button.
        // Thus, the actual order stops mattering at the second robot, and we can keep a running
        // tally of the transitions between keys (also keeping track of having to go back to the
        // `Activate` button to actually press it).
        let s = self
            .as_ref()
            .chars()
            .map(NumericButton::try_from)
            .collect::<Result<Vec<_>, _>>()
            .expect("invalid sequence");
        let mut numeric_button = NumericButton::default();
        let mut sequence: Counter<(DirectionalButton, DirectionalButton), usize> = Counter::new();

        for digit in s {
            let mut start = DirectionalButton::default();
            for d in numeric_button.to(digit).into_iter() {
                sequence[&(start, d)] += 1;
                start = d;
            }
            // Don't forget to press the activate button to actually press it.
            sequence[&(start, DirectionalButton::Activate)] += 1;
            numeric_button = digit;
        }

        for _ in 0..nbr_intermediaries {
            let mut new_sequence: Counter<(DirectionalButton, DirectionalButton), usize> =
                Counter::new();
            for ((start, end), c) in sequence {
                let mut current = DirectionalButton::default();
                for d in start.to(end) {
                    new_sequence[&(current, d)] += c;
                    current = d;
                }
                // Don't forget to press the activate button to actually press it.
                new_sequence[&(current, DirectionalButton::Activate)] += c;
            }
            sequence = new_sequence;
        }

        sequence.values().sum()
    }

    fn complexity(&self, nbr_intermediaries: usize) -> usize {
        let s = self.as_ref();
        self.shortest_sequence(nbr_intermediaries)
            * s.bytes()
                .filter(u8::is_ascii_digit)
                .map(|c| usize::from(c - b'0'))
                .fold(0, |acc, d| acc * 10 + d)
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT)?;

    let part1 = input.lines().map(|code| code.complexity(2)).sum::<usize>();
    println!("The first answer is: {}", part1);
    let part2 = input.lines().map(|code| code.complexity(25)).sum::<usize>();
    println!("The second answer is: {}", part2);

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    const EXAMPLE: &str = "\
        029A\n\
        980A\n\
        179A\n\
        456A\n\
        379A\n\
    ";

    #[test]
    fn test_part1() {
        let codes = EXAMPLE.lines();
        let actual = codes
            .into_iter()
            .map(|code| code.complexity(2))
            .sum::<usize>();
        let expected = 126384;

        assert_eq!(expected, actual);
    }
}
