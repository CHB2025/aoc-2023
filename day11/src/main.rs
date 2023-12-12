mod observation;

fn main() {
    let input = include_str!("input.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

fn part_one(input: &str) -> usize {
    use observation::Observation;
    let obs = input.parse::<Observation>().unwrap();
    obs.shortest_paths()
}

fn part_two(input: &str) -> u32 {
    let _ = input;
    0
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
    #[test]
    fn part_two_basic() {
        assert_eq!(part_two(INPUT), 0);
    }
}
