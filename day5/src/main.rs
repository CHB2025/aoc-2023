use almanac::Almanac;
use almanac2::Almanac2;

mod almanac;
mod almanac2;

fn main() {
    let input = include_str!("input.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

fn part_one(input: &str) -> u64 {
    let almanac: Almanac = input.parse().unwrap();

    almanac.locations().min().unwrap()
}

fn part_two(input: &str) -> u64 {
    let a2: Almanac2 = input.parse().unwrap();

    a2.locations().min().unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    const BASIC_INPUT: &str = {
        "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4"
    };

    #[test]
    fn part_one_basic() {
        assert_eq!(part_one(BASIC_INPUT), 35);
    }
    #[test]
    fn part_two_basic() {
        assert_eq!(part_two(BASIC_INPUT), 46);
    }
}
