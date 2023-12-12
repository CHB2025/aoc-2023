mod observation;

use observation::Observation;

fn main() {
    let input = include_str!("input.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

fn part_one(input: &str) -> usize {
    let mut obs = input.parse::<Observation>().unwrap();
    obs.expand(2);
    obs.shortest_paths()
}

fn part_two(input: &str) -> usize {
    let mut obs = input.parse::<Observation>().unwrap();
    obs.expand(1_000_000);
    obs.shortest_paths()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = {
        "...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#....."
    };

    #[test]
    fn part_one_basic() {
        assert_eq!(part_one(INPUT), 374);
    }
}
