use dijkstra::dijkstra;

mod dijkstra;
mod point;

fn main() {
    let input = include_str!("input.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

fn part_one(input: &str) -> usize {
    dijkstra(input, 0, 3)
}
fn part_two(input: &str) -> usize {
    dijkstra(input, 4, 10)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = {
        "2413432311323
3215453535623
3255245654254
3446585845452
4546657867536
1438598798454
4457876987766
3637877979653
4654967986887
4564679986453
1224686865563
2546548887735
4322674655533"
    };

    #[test]
    fn part_one_basic() {
        assert_eq!(part_one(INPUT), 102);
    }

    #[test]
    fn part_two_basic() {
        assert_eq!(part_two(INPUT), 94);
    }
}
