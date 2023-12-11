use pipe::Pipe;

mod pipe;

pub type Point = (usize, usize);
pub type Map = Box<[Box<[Option<Pipe>]>]>;

/// Quite a mess currently
fn main() {
    let input = include_str!("input.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

fn part_one(input: &str) -> u32 {
    let (map, start) = parse_map(input);

    let mut visited: Box<[Box<[bool]>]> = vec![vec![false; map[0].len()].into(); map.len()].into();
    visited[start.1][start.0] = true;

    let [mut left, mut right] = map[start.1][start.0].as_ref().unwrap().connections(start);
    let mut steps = 1;

    while left != right && !visited[left.1][left.0] {
        visited[left.1][left.0] = true;
        visited[right.1][right.0] = true;
        left = map[left.1][left.0]
            .as_ref()
            .unwrap()
            .connections(left)
            .into_iter()
            .find(|p| !visited[p.1][p.0] || p == &right)
            .unwrap();
        right = map[right.1][right.0]
            .as_ref()
            .unwrap()
            .connections(right)
            .into_iter()
            .find(|p| !visited[p.1][p.0] || left == right) // since left already moved
            .unwrap();
        steps += 1;
    }
    steps
}

fn part_two(input: &str) -> u32 {
    let _ = input;
    0
}

fn parse_map(input: &str) -> (Map, Point) {
    let mut start = (0, 0);
    let mut map: Box<[Box<[Option<Pipe>]>]> = input
        .lines()
        .enumerate()
        .map(|(y, l)| {
            l.chars()
                .enumerate()
                .map(|(x, c)| match c {
                    'S' => {
                        start = (x, y);
                        None
                    }
                    '.' => None,
                    c => Pipe::try_from(c).ok(),
                })
                .collect()
        })
        .collect();

    // Identify the correct pipe type for the starting point
    let north = start.1 > 0
        && map[start.1 - 1][start.0]
            .as_ref()
            .is_some_and(|p| p.connections((start.0, start.1 - 1)).contains(&start));
    let south = map[start.1 + 1][start.0]
        .as_ref()
        .is_some_and(|p| p.connections((start.0, start.1 + 1)).contains(&start));
    let east = map[start.1][start.0 + 1]
        .as_ref()
        .is_some_and(|p| p.connections((start.0 + 1, start.1)).contains(&start));
    let west = start.0 > 0
        && map[start.1][start.0 - 1]
            .as_ref()
            .is_some_and(|p| p.connections((start.0 - 1, start.1)).contains(&start));
    match (north, south, east, west) {
        (true, true, _, _) => map[start.1][start.0] = Some(Pipe::Ns),
        (true, _, true, _) => map[start.1][start.0] = Some(Pipe::Ne),
        (true, _, _, true) => map[start.1][start.0] = Some(Pipe::Nw),
        (_, _, true, true) => map[start.1][start.0] = Some(Pipe::Ew),
        (_, true, true, _) => map[start.1][start.0] = Some(Pipe::Se),
        (_, true, _, true) => map[start.1][start.0] = Some(Pipe::Sw),
        _ => unreachable!(),
    }

    (map, start)
}

#[cfg(test)]
mod tests {
    use super::*;

    const SIMPLE: &str = {
        ".....
.S-7.
.|.|.
.L-J.
....."
    };
    const RH_SIMPLE: &str = {
        "-L|F7
7S-7|
L|7||
-L-J|
L|-JF"
    };

    const COMPLEX: &str = {
        "..F7.
.FJ|.
SJ.L7
|F--J
LJ..."
    };

    #[test]
    fn part_one_basic() {
        assert_eq!(part_one(SIMPLE), 4);
        assert_eq!(part_one(RH_SIMPLE), 4);
    }

    #[test]
    fn part_one_complex() {
        assert_eq!(part_one(COMPLEX), 8);
    }
}
