use std::collections::HashMap;

type Directions = Box<[usize]>;
type Map<'a> = HashMap<&'a str, (&'a str, &'a str)>;

fn main() {
    let input = include_str!("input.txt");
    println!("Part one: {}", part_one(input, "AAA"));
    println!("Part two: {}", part_two(input));
}

fn part_one(input: &str, start: &str) -> usize {
    let (dirs, map) = parse_map(input);

    let mut steps = 0;
    let mut cur = start;
    // let dest = "ZZZ";

    while !(cur.ends_with('Z') && steps != 0) {
        // Reach ZZZ from AAA even in this case
        cur = match dirs[steps % dirs.len()] {
            0 => map.get(cur).unwrap().0,
            1 => map.get(cur).unwrap().1,
            _ => unreachable!(),
        };
        steps += 1;
    }

    steps
}

/// Stepping by one is way too slow
/// **A -> **Z -> **Z
///      a      b
/// res = a1 + b1x when equal to  a2 + b2x,...
/// an = bn
/// This equality is not expressly stated in the problem, but it is the case in my test data
/// So find each a, then find LCM
fn part_two(input: &str) -> usize {
    let (_, map) = parse_map(input);

    let multiples: Box<[usize]> = map
        .keys()
        .cloned()
        .filter(|s| s.ends_with('A'))
        .map(|s| part_one(input, s))
        .collect();

    let max = *multiples.iter().max().unwrap();
    let mut steps = max;
    while multiples.iter().any(|m| steps % m != 0) {
        steps += max;
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
        assert_eq!(part_one(INPUT_1, "AAA"), 2);
        assert_eq!(part_one(INPUT_2, "AAA"), 6);
    }

    #[test]
    fn part_two_basic() {
        let input = {
            "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)"
        };
        assert_eq!(part_two(input), 6);
    }
}
