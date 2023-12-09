use std::{num::ParseIntError, str::FromStr};

fn main() {
    let input = include_str!("input.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

fn part_one(input: &str) -> i32 {
    input
        .lines()
        .map(|l| {
            let nums = l
                .split_whitespace()
                .map(i32::from_str)
                .collect::<Result<Box<[i32]>, ParseIntError>>()
                .unwrap();
            sequence(&nums).next().unwrap()
        })
        .sum()
}
fn part_two(input: &str) -> i32 {
    input
        .lines()
        .map(|l| {
            let nums = l
                .split_whitespace()
                .map(i32::from_str)
                .collect::<Result<Box<[i32]>, ParseIntError>>()
                .unwrap();
            sequence_rev(&nums).next().unwrap()
        })
        .sum()
}

fn sequence(nums: &[i32]) -> SequenceIter {
    let mut offsets = Vec::new();
    let mut layer = nums.to_vec();
    while layer.iter().any(|&n| n != 0) {
        offsets.push(*layer.last().unwrap());
        layer = layer.windows(2).map(|n| n[1] - n[0]).collect();
    }
    SequenceIter {
        offsets: offsets.into(),
    }
}

fn sequence_rev(nums: &[i32]) -> SequenceIter {
    let mut offsets = Vec::new();
    let mut layer = nums.to_vec();
    while layer.iter().any(|&n| n != 0) {
        offsets.push(layer[0]);
        layer = layer.windows(2).map(|n| n[0] - n[1]).collect();
    }
    SequenceIter {
        offsets: offsets.into(),
    }
}

struct SequenceIter {
    offsets: Box<[i32]>,
}

impl Iterator for SequenceIter {
    type Item = i32;

    fn next(&mut self) -> Option<Self::Item> {
        for i in (1..self.offsets.len()).rev() {
            // can always access offsets[i-1]
            self.offsets[i - 1] += self.offsets[i]
        }
        self.offsets.first().cloned() // always returns Some
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = {
        "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45"
    };

    #[test]
    fn part_one_basic() {
        assert_eq!(part_one(INPUT), 114);
    }
    #[test]
    fn part_two_basic() {
        assert_eq!(part_two(INPUT), 2);
    }
}
