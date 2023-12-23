use std::str::FromStr;

use crate::lagoon::Lagoon;

mod lagoon;

fn main() -> anyhow::Result<()> {
    let input = include_str!("input.txt");

    println!("Part one: {}", part_one(input)?);
    println!("Part two: {}", part_two(input)?);

    Ok(())
}
fn part_one(input: &str) -> anyhow::Result<i64> {
    let lagoon = Lagoon::from_str(input)?;
    Ok(lagoon.area())
}
fn part_two(input: &str) -> anyhow::Result<i64> {
    let lagoon = Lagoon::from_hashes(input)?;

    Ok(lagoon.area())
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = {
        "R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)"
    };

    #[test]
    fn part_one_basic() {
        assert_eq!(62, part_one(INPUT).unwrap());
    }
    #[test]
    fn part_two_basic() {
        assert_eq!(952408144115, part_two(INPUT).unwrap());
    }
}
