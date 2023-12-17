use hashmap::Hashmap;

mod hashmap;
fn main() {
    let input = include_str!("input.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

fn part_one(input: &str) -> usize {
    input
        .trim()
        .split(',')
        .map(|step| ascii_hash(step.as_bytes()))
        .sum()
}

fn part_two(input: &str) -> usize {
    let mut hashmap = Hashmap::new();
    for step in input.trim().split(',') {
        if let Some((boks, power)) = step.split_once('=') {
            let power: usize = power.parse().expect("All powers should be integers");
            hashmap.add(boks, power);
        } else {
            hashmap.remove(step.trim_end_matches('-'))
        }
    }
    hashmap.power()
}

fn ascii_hash(input: &[u8]) -> usize {
    let mut current = 0; //will overflow u8
    for c in input {
        current = (current + *c as usize) * 17 % 256
    }
    current
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7";

    #[test]
    fn part_one_basic() {
        assert_eq!(part_one(INPUT), 1320);
    }

    #[test]
    fn part_two_basic() {
        assert_eq!(part_two(INPUT), 145);
    }

    #[test]
    fn ascii_hash_hash() {
        assert_eq!(ascii_hash(b"HASH"), 52);
    }
}
