use std::{iter, str::FromStr};

use condition::RowCondition;

mod condition;

fn main() {
    let input = include_str!("input.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

fn part_one(input: &str) -> u64 {
    input
        .lines()
        .map(|l| RowCondition::from_str(l).unwrap().possible_combinations())
        .sum()
}
fn part_two(input: &str) -> u64 {
    let folds = 5;
    input
        .lines()
        .map(|l| {
            let (s1, s2) = l.split_once(' ').unwrap();
            let mut out = iter::repeat(s1)
                .take(folds)
                .map(|s| s.to_owned())
                .collect::<Vec<_>>()
                .join("?");
            out += " ";
            out += iter::repeat(s2)
                .take(folds)
                .map(|s| s.to_owned())
                .collect::<Vec<_>>()
                .join(",")
                .as_str();
            out
        })
        .map(|l| {
            RowCondition::from_str(l.as_str())
                .unwrap()
                .possible_combinations()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = {
        "???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1"
    };

    #[test]
    fn part_one_basic() {
        let mut lines = INPUT.lines();
        assert_eq!(part_one(lines.next().unwrap()), 1);
        assert_eq!(part_one(lines.next().unwrap()), 4);
        assert_eq!(part_one(lines.next().unwrap()), 1);
        assert_eq!(part_one(lines.next().unwrap()), 1);
        assert_eq!(part_one(lines.next().unwrap()), 4);
        assert_eq!(part_one(lines.next().unwrap()), 10);
    }

    #[test]
    fn part_one_full() {
        assert_eq!(part_one(INPUT), 21);
    }

    #[test]
    fn part_one_manual() {
        let input = "???.?????##?#??????? 3,8,2";
        assert_eq!(part_one(input), 19);

        let input = ".???#??????#. 6,1";
        assert_eq!(part_one(input), 4);

        let input = "???##???##?#??#?#..# 1,14,1";
        assert_eq!(part_one(input), 2);
    }

    #[test]
    fn part_one_first() {
        let input = "?##?.??.???.. 3,2,2";
        assert_eq!(part_one(input), 4);
    }

    #[test]
    fn part_two_full() {
        assert_eq!(part_two(INPUT), 525152);
    }
}
