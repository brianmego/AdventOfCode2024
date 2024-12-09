#[cfg(test)]
pub const PUZZLE_INPUT: &str = include_str!("../../data/sample_input.txt");

#[cfg(not(test))]
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
struct DiskMap {
    data: String,
}
impl DiskMap {
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
                    Some(c) => c.to_digit(10).unwrap(),
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

    fn to_compact_file(&self) -> CompactFile {
        let blocks = self.as_blocks();
        let empty_indexes: Vec<usize> = blocks
            .chars()
            .into_iter()
            .enumerate()
            .filter(|(i, c)| {
                dbg!(c);
                c == &'.'
            })
            .map(|(i, _)| i)
            .collect();
        let filled_indexes: Vec<usize> = blocks
            .chars()
            .into_iter()
            .enumerate()
            .filter(|(i, c)| c != &'.')
            .map(|(i, _)| i)
            .collect();
        dbg!(empty_indexes);
        dbg!(filled_indexes);
        CompactFile { data: "123".into() }
    }
}

#[derive(Debug, PartialEq, Eq)]
struct CompactFile {
    data: String,
}
impl CompactFile {
    fn checksum(&self) -> usize {
        let sum: u32 = self
            .data
            .chars()
            .into_iter()
            .enumerate()
            .filter(|(_, c)| c != &'.')
            .map(|(i, c)| i as u32 * (c.to_digit(10).unwrap()))
            .sum();
        sum as usize
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    // #[test]
    // fn test_parse_input() {
    //     let inp = InputList::new(PUZZLE_INPUT);
    //     let actual = "";
    //     let expected = "";
    //     assert_eq!(actual, expected);
    // }
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
    // #[test_case("2333133121414131402", "0099811188827773336446555566..............")]
    fn test_to_compact_file(inp: &str, expected: &str) {
        let disk_map = DiskMap { data: inp.into() };
        let actual = disk_map.to_compact_file();
        assert_eq!(actual.data, expected.to_string());
    }
}
