use std::{cmp::Reverse, str::FromStr};

use card::Hand;

mod card;
mod card2;

fn main() {
    let input = include_str!("input.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

fn part_one(input: &str) -> u64 {
    let mut hb: Vec<(Hand, u64)> = input
        .lines()
        .map(|l| {
            let mut wsp = l.split_whitespace();
            let hand = Hand::from_str(wsp.next().unwrap()).expect("Bad hand");
            let bid = u64::from_str(wsp.next().unwrap()).expect("Bad bid");
            assert!(wsp.next().is_none());
            (hand, bid)
        })
        .collect();
    hb.sort_by(|(h, _), (o, _)| Reverse(h).cmp(&Reverse(o)));
    hb.into_iter()
        .enumerate()
        .map(|(i, (_, bid))| bid * (i as u64 + 1))
        .sum()
}

fn part_two(input: &str) -> u64 {
    use card2::Hand;
    let mut hb: Vec<(Hand, u64)> = input
        .lines()
        .map(|l| {
            let mut wsp = l.split_whitespace();
            let hand = Hand::from_str(wsp.next().unwrap()).expect("Bad hand");
            let bid = u64::from_str(wsp.next().unwrap()).expect("Bad bid");
            assert!(wsp.next().is_none());
            (hand, bid)
        })
        .collect();
    hb.sort_by(|(h, _), (o, _)| Reverse(h).cmp(&Reverse(o)));
    hb.into_iter()
        .enumerate()
        .map(|(i, (_, bid))| bid * (i as u64 + 1))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = {
        "32T3K 765
        T55J5 684
        KK677 28
        KTJJT 220
        QQQJA 483"
    };

    const INPUT_REDDIT: &str = {
        "2345A 1
Q2KJJ 13
Q2Q2Q 19
T3T3J 17
T3Q33 11
2345J 3
J345A 2
32T3K 5
T55J5 29
KK677 7
KTJJT 34
QQQJA 31
JJJJJ 37
JAAAA 43
AAAAJ 59
AAAAA 61
2AAAA 23
2JJJJ 53
JJJJ2 41"
    };

    #[test]
    fn part_one_basic() {
        assert_eq!(part_one(INPUT), 6440);
    }

    #[test]
    fn part_two_basic() {
        assert_eq!(part_two(INPUT), 5905);
    }

    #[test]
    fn part_two_inter() {
        assert_eq!(part_two(INPUT_REDDIT), 6839);
    }
}
