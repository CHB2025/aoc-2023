use std::collections::HashMap;

type Directions = Box<[usize]>;
type Map<'a> = HashMap<&'a str, (&'a str, &'a str)>;

fn main() {
    let input = include_str!("input.txt");
    println!("Part one: {}", part_one(input));
}

fn part_one(input: &str) -> usize {
    let (dirs, map) = parse_map(input);

    let mut steps = 0;
    let mut cur = "AAA";
    let dest = "ZZZ";

    while cur != dest {
        cur = match dirs[steps % dirs.len()] {
            0 => map.get(cur).unwrap().0,
            1 => map.get(cur).unwrap().1,
            _ => unreachable!(),
        };
        steps += 1;
    }

    steps
}

fn parse_map(input: &str) -> (Directions, Map) {
    let mut lines = input.lines();
    let dirs = lines.next().unwrap().chars().map(|c| match c {
        'L' => 0,
        'R' => 1,
        _ => panic!("Improperly formatted direction line"),
    });
    _ = lines.next();
    let mut map = HashMap::new();
    for line in lines {
        let (key, dests) = line.split_once('=').unwrap();
        let mut dests: (&str, &str) = dests
            .trim()
            .trim_matches(&['(', ')'][..])
            .split_once(',')
            .unwrap();
        dests.0 = dests.0.trim();
        dests.1 = dests.1.trim();
        assert!(map.insert(key.trim(), dests).is_none());
    }
    (dirs.collect(), map)
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT_1: &str = {
        "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)"
    };

    const INPUT_2: &str = {
        "LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)"
    };

    #[test]
    fn part_one_basic() {
        assert_eq!(part_one(INPUT_1), 2);
        assert_eq!(part_one(INPUT_2), 6);
    }
}
