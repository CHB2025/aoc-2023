use std::str::FromStr;

use card::Card;

mod card;

fn main() {
    let input = include_str!("input.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

fn part_one(input: &str) -> u32 {
    input
        .lines()
        .map(|l| Card::from_str(l).unwrap().score())
        .sum()
}

fn part_two(input: &str) -> u32 {
    let cards = input
        .lines()
        .map(Card::from_str)
        .collect::<Result<Box<[Card]>, anyhow::Error>>()
        .unwrap();
    let mut counts: Box<[u32]> = Box::from(vec![1; cards.len()]);
    for card in cards.iter() {
        let copies = card.matches();
        // cards are indexed from 1
        for i in card.id()..card.id() + copies {
            counts[i] += counts[card.id() - 1];
        }
    }
    counts.iter().sum()
}

#[cfg(test)]
mod tests {
    use crate::*;

    const TEST_INPUT: &str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

    #[test]
    fn test_part_one_basic() {
        assert_eq!(part_one(TEST_INPUT), 13);
    }

    #[test]
    fn test_part_two_basic() {
        assert_eq!(part_two(TEST_INPUT), 30);
    }
}
