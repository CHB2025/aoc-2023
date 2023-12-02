use anyhow::anyhow;
use std::str::FromStr;

fn main() {
    let input = include_str!("input.txt");
    println!("Part 1: {}", part_one(input));
    println!("Part 2: {}", part_two(input))
}

struct Game {
    id: u32,
    red_shown: u32,
    green_shown: u32,
    blue_shown: u32,
}

impl FromStr for Game {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (title, game_data) = s
            .split_once(':')
            .ok_or_else(|| anyhow!(format!("Invalid Game format: {}", s)))?;
        let id: u32 = title
            .strip_prefix("Game ")
            .ok_or(anyhow!(format!("Invalid Game format: {}", s)))?
            .parse()?;

        let (mut red, mut green, mut blue) = (0, 0, 0);

        for handful in game_data.split(';').map(str::trim) {
            let color_counts = handful.split(',').map(|s| {
                let (count, color) = s.trim().split_once(' ').unwrap(); // Should return error here
                (count.parse::<u32>().unwrap(), color) // Should return error here
            });
            for cc in color_counts {
                match cc {
                    (count, "red") if count > red => red = count,
                    (count, "green") if count > green => green = count,
                    (count, "blue") if count > blue => blue = count,
                    (_, "red" | "green" | "blue") => (),
                    (_, _) => unreachable!(),
                }
            }
        }
        Ok(Self {
            id,
            red_shown: red,
            green_shown: green,
            blue_shown: blue,
        })
    }
}

// Max values:
//  - 12 red
//  - 13 green
//  - 14 blue
fn part_one(input: &str) -> u32 {
    const MAX_RED: u32 = 12;
    const MAX_GREEN: u32 = 13;
    const MAX_BLUE: u32 = 14;
    let games = input.lines().map(|line| line.parse::<Game>().unwrap());
    games
        .filter(|g| {
            g.red_shown <= MAX_RED && g.green_shown <= MAX_GREEN && g.blue_shown <= MAX_BLUE
        })
        .map(|g| g.id)
        .sum()
}

fn part_two(input: &str) -> u32 {
    input
        .lines()
        .map(|line| {
            let g = line.parse::<Game>().unwrap();
            g.red_shown * g.blue_shown * g.green_shown
        })
        .sum()
}
