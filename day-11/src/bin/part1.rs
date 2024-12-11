mod shared;
use shared::{PUZZLE_INPUT, StoneList};

fn main() {
    let stone_list = StoneList::from(PUZZLE_INPUT);
    let new_list = stone_list.blink(25);
    println!("{}", new_list.len());

}

#[cfg(test)]
mod tests {
    use super::*;
    use test_case::test_case;

    #[test_case("125 17", 6, 22)]
    #[test_case("125 17", 25, 55312)]
    fn test_blink_count(inp: &str, count: usize, expected: usize) {
        let stone_list = StoneList::from(inp);
        let new_list = stone_list.blink(count);
        let actual = new_list.len();
        assert_eq!(actual, expected);
    }
}
