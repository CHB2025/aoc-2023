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
    let (map, start) = parse_map(input);

    let mut on_loop: Box<[Box<[bool]>]> = vec![vec![false; map[0].len()].into(); map.len()].into();
    on_loop[start.1][start.0] = true;

    let [mut left, finish] = map[start.1][start.0].as_ref().unwrap().connections(start);
    on_loop[finish.1][finish.0] = true;

    while left != finish {
        on_loop[left.1][left.0] = true;
        left = map[left.1][left.0]
            .as_ref()
            .unwrap()
            .connections(left)
            .into_iter()
            .find(|p| !on_loop[p.1][p.0] || p == &finish) // immediately returning to start
            .unwrap();
    }

    let mut count = 0;

    for (y, row) in map.iter().enumerate() {
        let mut state = false;
        let mut start_corner = None;
        for (x, pipe) in row.iter().enumerate() {
            // Going east to west
            if on_loop[y][x] {
                let pipe = pipe.as_ref().unwrap();
                match pipe {
                    Pipe::Ns => state = !state,
                    Pipe::Ew => (),
                    // L
                    Pipe::Ne => match start_corner {
                        Some(_) => unreachable!(),
                        None => start_corner = Some(Pipe::Ne),
                    },
                    Pipe::Nw => match start_corner {
                        // L--J
                        Some(Pipe::Ne) => start_corner = None,
                        // F--J
                        Some(Pipe::Se) => {
                            state = !state;
                            start_corner = None
                        }
                        _ => unreachable!(),
                    },
                    // 7
                    Pipe::Sw => match start_corner {
                        // L--7
                        Some(Pipe::Ne) => {
                            state = !state;
                            start_corner = None;
                        }
                        // F--7
                        Some(Pipe::Se) => start_corner = None,
                        _ => unreachable!(),
                    },
                    Pipe::Se => match start_corner {
                        Some(_) => unreachable!(),
                        None => start_corner = Some(Pipe::Se),
                    },
                };
            } else if state {
                count += 1;
            }
        }
        assert!(
            start_corner.is_none(),
            "Mismatched corner {:?} on line {}",
            start_corner,
            y
        );
        assert!(
            !state,
            "Expected to be out of the loop at the end of line {}",
            y
        )
    }

    count
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

    #[test]
    fn part_two_basic() {
        assert_eq!(part_two(SIMPLE), 1);
        assert_eq!(part_two(RH_SIMPLE), 1);
    }

    #[test]
    fn part_two_med() {
        let input = {
            "...........
.S-------7.
.|F-----7|.
.||.....||.
.||.....||.
.|L-7.F-J|.
.|..|.|..|.
.L--J.L--J.
..........."
        };

        assert_eq!(part_two(input), 4);
    }
}
