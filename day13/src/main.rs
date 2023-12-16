use pattern::Pattern;

mod pattern;

fn main() {
    let input = include_str!("input.txt");
    println!("{}", part_one(input));
    println!("{}", part_two(input));
}

fn part_one(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|section| {
            let pattern = Pattern::new(section);
            // check for horizontal mirrors
            for i in 0..pattern.rows() - 1 {
                // check for match after i
                if rows_mirror(&pattern, i, 0) {
                    return 100 * (i + 1);
                }
            }

            // check for vertical mirrors
            for i in 0..pattern.columns() - 1 {
                if cols_mirror(&pattern, i, 0) {
                    return i + 1;
                }
            }
            unreachable!()
        })
        .sum()
}

fn rows_mirror(pattern: &Pattern, mirror: usize, target_diff: usize) -> bool {
    let mut diff = 0;
    for i in 0..=mirror.min(pattern.rows() - mirror - 2) {
        // compare i to mirror + i + 1 ?
        // println!(
        //     "cmp row {}({:?}) to {}({:?})",
        //     mirror - i,
        //     pattern.get_row(mirror - i),
        //     mirror + i + 1,
        //     pattern.get_row(mirror + i + 1)
        // );
        for (a, b) in pattern
            .get_row(mirror - i)
            .unwrap()
            .chars()
            .zip(pattern.get_row(mirror + i + 1).unwrap().chars())
        {
            if a != b {
                diff += 1;
                if diff > target_diff {
                    return false;
                }
            }
        }
        // if pattern.get_row(mirror - i) != pattern.get_row(mirror + i + 1) {
        //     return false;
        // }
    }
    diff == target_diff
}
fn cols_mirror(pattern: &Pattern, mirror: usize, target_diff: usize) -> bool {
    let mut diff = 0;

    for i in 0..=mirror.min(pattern.columns() - mirror - 2) {
        // println!(
        //     "cmp col {}({:?}) to {}({:?})",
        //     mirror - i,
        //     pattern.get_column(mirror - i),
        //     mirror + i + 1,
        //     pattern.get_column(mirror + i + 1)
        // );
        for (a, b) in pattern
            .get_column(mirror - i)
            .unwrap()
            .chars()
            .zip(pattern.get_column(mirror + i + 1).unwrap().chars())
        {
            if a != b {
                diff += 1;
                if diff > target_diff {
                    return false;
                }
            }
        }
        //     // compare i to mirror + i + 1 ?
        //     if pattern.get_column(mirror - i) != pattern.get_column(mirror + i + 1) {
        //         return false;
        //     }
    }
    diff == target_diff
}

fn part_two(input: &str) -> usize {
    input
        .split("\n\n")
        .map(|section| {
            let pattern = Pattern::new(section);
            // check for horizontal mirrors
            for i in 0..pattern.rows() - 1 {
                // check for match after i
                if rows_mirror(&pattern, i, 1) {
                    return 100 * (i + 1);
                }
            }

            // check for vertical mirrors
            for i in 0..pattern.columns() - 1 {
                if cols_mirror(&pattern, i, 1) {
                    return i + 1;
                }
            }
            unreachable!()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = {
        "#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#"
    };

    #[test]
    fn part_one_basic() {
        assert_eq!(part_one(INPUT), 405);
    }
    #[test]
    fn part_two_basic() {
        assert_eq!(part_two(INPUT), 400);
    }
}
