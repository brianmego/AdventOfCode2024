pub const PUZZLE_INPUT: &str = include_str!("../../data/puzzle_input.txt");

#[derive(Debug)]
struct Block {
    file_size: usize,
    free_space: usize,
}

impl Block {
    fn new(file_size: usize, free_space: usize) -> Self {
        Self {
            file_size,
            free_space,
        }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct DiskMap {
    data: String,
}
impl DiskMap {
    pub fn new(data: &str) -> Self {
        Self { data: data.trim().to_string() }
    }

    fn as_blocks(&self) -> String {
        let blocks: Vec<Block> = self
            .data
            .chars()
            .into_iter()
            .collect::<Vec<char>>()
            .chunks(2)
            .map(|x| {
                let block_file_size = x[0].to_digit(10).unwrap();
                let free_space = match x.get(1) {
                    Some(c) => {
                        c.to_digit(10).unwrap()
                    },
                    None => 0,
                };
                Block::new(block_file_size as usize, free_space as usize)
            })
            .collect();
        let blocks_as_str: Vec<String> = blocks
            .iter()
            .enumerate()
            .map(|(i, b)| {
                let filled_space = i.to_string().repeat(b.file_size);
                let free_space = '.'.to_string().repeat(b.free_space);
                filled_space + &free_space
            })
            .collect();
        blocks_as_str.join("")
    }

    pub fn to_compact_file(&self) -> CompactFile {
        let mut blocks = self.as_blocks();
        let empty_indexes: Vec<usize> = blocks
            .chars()
            .into_iter()
            .enumerate()
            .filter(|(_, c)| c == &'.')
            .map(|(i, _)| i)
            .collect();
        let mut filled_indexes: Vec<usize> = blocks
            .chars()
            .into_iter()
            .enumerate()
            .filter(|(i, c)| c != &'.')
            .map(|(i, _)| i)
            .collect();
        filled_indexes.reverse();
        empty_indexes.iter().enumerate().for_each(|(i, idx)| {
            if *idx < filled_indexes[i] {
                blocks.replace_range(
                    idx..=idx,
                    &blocks.chars().nth(filled_indexes[i]).unwrap().to_string(),
                );
                blocks.replace_range(filled_indexes[i]..=filled_indexes[i], ".");
            }
        });
        CompactFile { data: blocks }
    }
}

#[derive(Debug, PartialEq, Eq)]
pub struct CompactFile {
    data: String,
}
impl CompactFile {
    pub fn checksum(&self) -> usize {
        dbg!(&self.data);
        let sum: usize = self
            .data
            .chars()
            .into_iter()
            .enumerate()
            .filter(|(_, c)| c != &'.')
            .map(|(i, c)| {dbg!(i, c, c.to_digit(10).unwrap());i * (c.to_digit(10).unwrap()) as usize})
            .sum();
        sum as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test]
    fn test_checksum() {
        let compact_file = CompactFile {
            data: "0099811188827773336446555566..............".into(),
        };
        let actual = compact_file.checksum();
        assert_eq!(actual, 1928);
    }

    #[test_case("12345", "0..111....22222")]
    #[test_case("2333133121414131402", "00...111...2...333.44.5555.6666.777.888899")]
    fn test_as_blocks(inp: &str, expected: &str) {
        let disk_map = DiskMap { data: inp.into() };
        let actual = disk_map.as_blocks();
        assert_eq!(actual, expected.to_string());
    }

    #[test_case("12345", "022111222......")]
    #[test_case("2333133121414131402", "0099811188827773336446555566..............")]
    fn test_to_compact_file(inp: &str, expected: &str) {
        let disk_map = DiskMap { data: inp.into() };
        let actual = disk_map.to_compact_file();
        assert_eq!(actual.data, expected.to_string());
    }
}
