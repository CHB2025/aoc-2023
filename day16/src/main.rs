use heat_splitter::{Beam, Direction, HeatSplitter};

mod heat_splitter;

fn main() {
    let input = include_str!("input.txt");
    println!("Part one: {}", part_one(input));
    println!("Part two: {}", part_two(input));
}

fn part_one(input: &str) -> usize {
    let contraption: HeatSplitter = input.parse().unwrap();
    contraption.energize(Beam {
        direction: Direction::Right,
        x: 0,
        y: 0,
    })
}

// Obviously some inefficiencies here that could be cleaned up. But I want to try rayon
fn part_two(input: &str) -> usize {
    let contraption: HeatSplitter = input.parse().unwrap();
    let (_beam, max_energy) = (0..contraption.width())
        .map(|x| {
            // TOP DOWN
            let contraption: HeatSplitter = input.parse().unwrap();
            let beam = Beam {
                direction: Direction::Down,
                x,
                y: 0,
            };
            let heat = contraption.energize(beam);
            (beam, heat)
        })
        .chain((0..contraption.width()).map(|x| {
            // BOTTOM UP
            let contraption: HeatSplitter = input.parse().unwrap();
            let beam = Beam {
                direction: Direction::Up,
                x,
                y: contraption.height() - 1,
            };
            let heat = contraption.energize(beam);
            (beam, heat)
        }))
        .chain((0..contraption.height()).map(|y| {
            // LTR
            let contraption: HeatSplitter = input.parse().unwrap();
            let beam = Beam {
                direction: Direction::Right,
                y,
                x: 0,
            };
            let heat = contraption.energize(beam);
            (beam, heat)
        }))
        .chain((0..contraption.height()).map(|y| {
            // RTL
            let contraption: HeatSplitter = input.parse().unwrap();
            let beam = Beam {
                direction: Direction::Left,
                y,
                x: contraption.width() - 1,
            };
            let heat = contraption.energize(beam);
            (beam, heat)
        }))
        .max_by_key(|(_, heat)| *heat)
        .unwrap();
    max_energy
}

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = {
        ".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|...."
    };

    #[test]
    fn part_one_basic() {
        assert_eq!(part_one(INPUT), 46);
    }

    #[test]
    fn part_two_basic() {
        assert_eq!(part_two(INPUT), 51);
    }
}
