use std::num::ParseIntError;
use std::str::FromStr;

const INPUT: &str = "./input/day9.txt";

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Block {
    File(usize, usize),
    Empty(usize),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct DiskMap {
    blocks: Vec<Block>,
}

impl DiskMap {
    fn fragmented_compaction(&mut self) -> &mut Self {
        while let Some(idx) = self.blocks.iter().position(|block| {
            std::mem::discriminant(block) == std::mem::discriminant(&Block::Empty(0))
        }) {
            if let Some(Block::File(f_size, id)) = self.blocks.pop() {
                if let Block::Empty(e_size) = self.blocks[idx] {
                    match f_size.cmp(&e_size) {
                        std::cmp::Ordering::Less => {
                            self.blocks[idx] = Block::Empty(e_size - f_size);
                            self.blocks.insert(idx, Block::File(f_size, id));
                        }
                        std::cmp::Ordering::Equal => self.blocks[idx] = Block::File(f_size, id),
                        std::cmp::Ordering::Greater => {
                            self.blocks[idx] = Block::File(e_size, id);
                            self.blocks.push(Block::File(f_size - e_size, id));
                        }
                    }
                }
            }
        }

        self
    }

    fn compaction(&mut self) -> &mut Self {
        // We make only one attempt to move each file.
        for idx in (0..self.blocks.len()).rev() {
            match self.blocks[idx] {
                Block::Empty(_) => (),
                Block::File(f_size, _) => {
                    if let Some(empty) = self.blocks.iter().position(
                        |potential| matches!(potential, &Block::Empty(e_size) if e_size >= f_size),
                    ) {
                        if empty < idx {
                            let file = self.blocks[idx];
                            self.blocks[idx] = Block::Empty(f_size);
                            match self.blocks[empty] {
                                Block::Empty(e_size) => {
                                    self.blocks[empty] = file;
                                    if e_size > f_size {
                                        match self.blocks.get(empty + 1) {
                                            Some(Block::Empty(size)) => {
                                                self.blocks[empty + 1] =
                                                    Block::Empty(size + e_size - f_size)
                                            }
                                            _ => self
                                                .blocks
                                                .insert(empty + 1, Block::Empty(e_size - f_size)),
                                        }
                                    }
                                }
                                _ => unreachable!("This is the right index."),
                            }
                        }
                    }
                }
            }
        }

        self
    }

    fn checksum(&self) -> usize {
        self.blocks
            .iter()
            .flat_map(|&block| match block {
                Block::File(size, id) => std::iter::repeat_n(id, size),
                Block::Empty(size) => std::iter::repeat_n(0, size),
            })
            .enumerate()
            .map(|(position, id)| position * id)
            .sum()
    }
}

impl FromStr for DiskMap {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut blocks = Vec::new();
        for (id, chunk) in s.trim().as_bytes().chunks(2).enumerate() {
            blocks.push(Block::File(usize::from(chunk[0] - b'0'), id));
            if let Some(length) = chunk.get(1) {
                blocks.push(Block::Empty(usize::from(length - b'0')));
            }
        }
        Ok(Self { blocks })
    }
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let input = std::fs::read_to_string(INPUT)?;
    let disk = DiskMap::from_str(&input)?;

    println!(
        "The first answer is: {}",
        disk.clone().fragmented_compaction().checksum()
    );
    println!(
        "The second answer is: {}",
        disk.clone().compaction().checksum()
    );
    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    const SAMPLE: &str = "2333133121414131402";

    #[test]
    fn test_part1() {
        let mut disk = SAMPLE.parse::<DiskMap>().unwrap();
        let actual = disk.fragmented_compaction().checksum();
        let expected = 1928;
        assert_eq!(expected, actual);
    }

    #[test]
    fn test_part2() {
        let mut disk = SAMPLE.parse::<DiskMap>().unwrap();
        let actual = disk.compaction().checksum();
        let expected = 2858;
        assert_eq!(expected, actual);
    }
}
