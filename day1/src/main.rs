fn main() {
    let input = include_str!("input.txt").trim();
    println!("Part 1: {}", part_one(input));
    println!("Part 2: {}", part_two(input));
}

fn part_one(input: &str) -> u32 {
    input.lines().fold(0, |acc, l| acc + calibrate(l))
}

fn calibrate(line: &str) -> u32 {
    let mut chars = line.chars();
    let first = chars
        .find(|c| c.is_ascii_digit())
        .expect("at least one number should exist in the input");
    let last = chars.rev().find(|c| c.is_ascii_digit()).unwrap_or(first);
    first.to_digit(10).unwrap() * 10 + last.to_digit(10).unwrap()
}

fn part_two(input: &str) -> usize {
    input.lines().fold(0, |acc, l| acc + calibrate_fixed(l))
}

fn calibrate_fixed(line: &str) -> usize {
    let numbers = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "0", "1",
        "2", "3", "4", "5", "6", "7", "8", "9",
    ];
    let (mut min_start, mut min_val) = (usize::MAX, 0);
    let (mut max_start, mut max_val) = (0, 0);

    for (i, num) in numbers.into_iter().enumerate() {
        if let Some(ind) = line.find(num) {
            if ind < min_start {
                min_start = ind;
                min_val = i % 10;
            }

            if let Some(ind) = line.rfind(num) {
                if ind >= max_start {
                    max_start = ind;
                    max_val = i % 10;
                }
            }
        }
    }
    min_val * 10 + max_val
}
