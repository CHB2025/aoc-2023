use std::{num::ParseIntError, str::FromStr};

fn main() {
    let input = include_str!("input.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

fn part_one(input: &str) -> u32 {
    let mut lines = input.lines();
    let times = lines
        .next()
        .expect("Missing time line in input")
        .split_once(':')
        .expect("Improperly formatted time input")
        .1
        .split_whitespace()
        .map(u32::from_str)
        .collect::<Result<Box<[u32]>, ParseIntError>>()
        .expect("Incorrect times");
    let distances = lines
        .next()
        .expect("Missing distance line in input")
        .split_once(':')
        .expect("Improperly formatted distance input")
        .1
        .split_whitespace()
        .map(u32::from_str)
        .collect::<Result<Box<[u32]>, ParseIntError>>()
        .expect("Incorrect distances");
    assert!(times.len().eq(&distances.len()));

    times
        .iter()
        .zip(distances.iter())
        .map(|(t, d)| winning_races(*t, *d))
        .fold(1, |acc, num| acc * num as u32)
}

fn part_two(input: &str) -> u64 {
    let mut lines = input.lines();
    let time: u64 = lines
        .next()
        .expect("Missing time line in input")
        .split_once(':')
        .expect("Improperly formatted time input")
        .1
        .split_whitespace()
        .collect::<String>()
        .parse()
        .expect("Invalid time");
    let distance: u64 = lines
        .next()
        .expect("Missing distance line in input")
        .split_once(':')
        .expect("Improperly formatted distance input")
        .1
        .split_whitespace()
        .collect::<String>()
        .parse()
        .expect("Invalid distance");
    winning_races_two(time, distance)
}

fn winning_races(time: u32, distance: u32) -> usize {
    // don't move holding 0 or time seconds, so skip them
    // really only need to find the range that will work. don't need to loop through all
    (1..time)
        .map(|t| t * (time - t))
        .filter(|d| *d > distance)
        .count()
}

fn winning_races_two(time: u64, distance: u64) -> u64 {
    // don't move holding 0 or time seconds, so skip them
    let mut margin = 1;
    while margin * (time - margin) < distance {
        margin += 1;
    }
    time - 2 * margin + 1 // +1 since first one will work
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = {
        "Time:      7  15   30
Distance:  9  40  200"
    };

    #[test]
    fn part_one_basic() {
        assert_eq!(part_one(TEST_INPUT), 288);
    }

    #[test]
    fn part_two_basic() {
        assert_eq!(part_two(TEST_INPUT), 71503);
    }

    #[test]
    fn test_winning_races() {
        assert_eq!(winning_races(7, 9), 4)
    }
}
