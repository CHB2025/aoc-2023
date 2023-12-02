fn main() {
    let input = include_str!("input.txt").trim();
    part_one(input);
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
