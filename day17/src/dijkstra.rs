use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use crate::point::Point;

#[derive(Clone, Copy, PartialEq, Eq, Hash, Debug)]
enum Dir {
    Up,
    Down,
    Left,
    Right,
}

pub fn parse_map(input: &str) -> (Box<[usize]>, usize, usize) {
    let width = input.lines().next().expect("at least one line").len();
    let height = input.lines().count();
    let map: Box<[usize]> = input
        .lines()
        .flat_map(|l| {
            l.chars()
                .map(|c| usize::from_str(&format!("{c}")).expect("Only numeric character"))
        })
        .collect();
    assert!(map.len() / height == width);
    assert!(map.len() % height == 0);
    (map, height, width)
}

pub fn dijkstra(input: &str, min: usize, max: usize) -> usize {
    let (map, height, width) = parse_map(input);
    let start = Point { x: 0, y: 0 };
    let finish = Point {
        x: width - 1,
        y: height - 1,
    };
    let mut costs = HashMap::new();
    let mut unvis = HashSet::new();
    for x in 0..width {
        for y in 0..height {
            for dir in 0..4 {
                for run in 0..max {
                    let p = Point { x, y };
                    let dir = match dir {
                        0 => Dir::Up,
                        1 => Dir::Down,
                        2 => Dir::Left,
                        3 => Dir::Right,
                        _ => unreachable!(),
                    };
                    unvis.insert((p, dir, run));
                }
            }
        }
    }
    costs.insert((start, Dir::Up, 0), 0);
    costs.insert((start, Dir::Down, 0), 0);
    costs.insert((start, Dir::Left, 0), 0);
    costs.insert((start, Dir::Right, 0), 0);
    let mut count = 0;

    loop {
        let (key, _) = costs
            .iter()
            .min_by_key(|&(_, v)| *v)
            .expect("Will always find the key");
        unvis.remove(key);
        let ((point, dir, run), cost) = costs.remove_entry(&key.clone()).expect("Valid key");
        count += 1;
        if count % 1000 == 0 {
            print!("{count}/{}\r", height * width * max * 4);
        }

        if point == finish {
            if run >= min {
                return cost;
            } else {
                continue;
            }
        }

        let mut add_dir = |d: Dir| {
            let p = match d {
                Dir::Up => point - (0, 1),
                Dir::Down => point + (0, 1),
                Dir::Left => point - (1, 0),
                Dir::Right => point + (1, 0),
            };
            if let Some(p) = p {
                if p.x < width && p.y < height {
                    let i = p.y * width + p.x;
                    let r = if dir == d { run + 1 } else { 0 };
                    let key = (p, d, r);
                    if unvis.contains(&key) {
                        let c = costs.entry(key).or_insert(usize::MAX);
                        *c = (*c).min(cost + map[i]);
                    }
                }
            }
        };

        // update cost of left, right and forward if run less than 2
        let before_range = run < min.saturating_sub(1); // only forward
        let in_range = run >= min.saturating_sub(1) && run < max - 1;
        // println!("{point} {dir:?} {run}\n\tbefore_range: {before_range}\n\tin_range: {in_range}");
        if before_range || in_range {
            // println!("\tAdding {dir:?}");
            add_dir(dir);
        }
        if before_range {
            continue;
        }
        if dir != Dir::Down && dir != Dir::Up {
            // println!("\tAdding Up and Down");
            add_dir(Dir::Up);
            add_dir(Dir::Down);
        }
        if dir != Dir::Left && dir != Dir::Right {
            // println!("\tAdding Left and Right");
            add_dir(Dir::Right);
            add_dir(Dir::Left);
        }
    }
}
