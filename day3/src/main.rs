use schematic::Schematic;

mod schematic;

fn main() {
    let input = include_str!("input.txt");
    println!("Part one: {}", part_one(input))
}

fn part_one(input: &str) -> u32 {
    let schematic = Schematic::new(input);
    schematic.part_numbers().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    fn part_one_basic() {
        assert_eq!(part_one(TEST_INPUT), 4361);
    }
}
