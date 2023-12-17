use std::{
    collections::{hash_map::Entry, HashMap},
    str::FromStr,
};

use platform::{Direction, Platform};

mod platform;

fn main() -> anyhow::Result<()> {
    let input = include_str!("input.txt");
    println!("Part one: {}", part_one(input)?);
    println!("Part two: {}", part_two(input)?);
    Ok(())
}

fn part_one(input: &str) -> anyhow::Result<usize> {
    let mut platform = Platform::from_str(input)?;
    platform.tilt(Direction::North);

    Ok(platform.load())
}

fn part_two(input: &str) -> anyhow::Result<usize> {
    let mut platform = Platform::from_str(input)?;

    let mut map = HashMap::new();
    map.insert(platform.clone(), 0);

    let mut n = 0;
    let (start, end) = loop {
        n += 1;
        platform.spin();
        match map.entry(platform.clone()) {
            Entry::Occupied(e) => break (*e.get(), n),
            Entry::Vacant(e) => e.insert(n),
        };
    };
    let num_cycles = 1_000_000_000;
    let rem = (num_cycles - start) % (end - start);
    for _ in 0..rem {
        platform.spin();
    }

    Ok(platform.load())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = {
        "O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#...."
    };

    #[test]
    fn part_one_basic() {
        assert!(part_one(INPUT).is_ok_and(|l| l == 136));
    }

    #[test]
    fn spin_cycle() {
        let mut platform = Platform::from_str(INPUT).unwrap();
        platform.spin();
        platform.spin();
        platform.spin();

        let expected = Platform::from_str(
            ".....#....
....#...O#
.....##...
..O#......
.....OOO#.
.O#...O#.#
....O#...O
.......OOO
#...O###.O
#.OOO#...O",
        )
        .unwrap();
        assert_eq!(platform, expected)
    }

    #[test]
    fn part_two_basic() {
        assert!(part_two(INPUT).is_ok_and(|l| l == 64));
    }
}
