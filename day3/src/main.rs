use schematic::Schematic;

mod schematic;

fn main() {
    let input = include_str!("input.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

fn part_one(input: &str) -> u32 {
    let schematic = Schematic::new(input);
    schematic.part_numbers().sum()
}

fn part_two(input: &str) -> u32 {
    let schematic = Schematic::new(input);
    schematic.gears().map(|g| g.ratio()).sum()
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

    #[test]
    fn part_two_basic() {
        assert_eq!(part_two(TEST_INPUT), 467835);
    }
}
