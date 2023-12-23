use std::str::FromStr;

use anyhow::{anyhow, Context};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Vertex {
    x: i64,
    y: i64,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Lagoon {
    verts: Box<[Vertex]>,
}

impl Lagoon {
    pub fn area(&self) -> i64 {
        if self.verts.len() < 3 {
            return 0;
        }

        let first = self.verts.first().expect("size > 2");
        let last = self.verts.last().expect("size > 2");
        let sum: i64 = self
            .verts
            .windows(2)
            .map(|s| {
                (s[0].x * s[1].y) - (s[0].y * s[1].x)
                    + (s[0].x.abs_diff(s[1].x) + s[0].y.abs_diff(s[1].y)) as i64
            })
            .sum::<i64>()
            + (first.y * last.x)
            - (first.x * last.y)
            + (last.x.abs_diff(first.x) + last.y.abs_diff(first.y)) as i64;
        sum / 2 + 1 // not sure why I need this + 1
    }

    pub fn from_hashes(input: &str) -> anyhow::Result<Self> {
        let vert_count = input.lines().count();
        let mut verts = Vec::with_capacity(vert_count);
        let mut prev = Vertex { x: 0, y: 0 };
        for l in input.lines() {
            let mut parts = l.split_whitespace();
            let hash = parts
                .nth(2)
                .context("Dig plan {l} is missing a hash")?
                .trim_matches(&['(', ')'][..]);
            if !hash.starts_with('#') || hash.len() != 7 {
                return Err(anyhow!("Invalid hash: {hash}"));
            }
            let dir = u8::from_str_radix(&hash[6..], 16)?;
            let count = i64::from_str_radix(&hash[1..6], 16)?;

            let mut next = prev;
            match dir {
                0 => next.x += count,
                1 => next.y += count,
                2 => next.x -= count,
                3 => next.y -= count,
                _ => return Err(anyhow!("Invalid direction {dir} from hash {hash}")),
            }
            verts.push(next);
            prev = next;
        }
        Ok(Lagoon {
            verts: verts.into(),
        })
    }
}

impl FromStr for Lagoon {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let vert_count = s.lines().count();
        let mut verts = Vec::with_capacity(vert_count);
        let mut prev = Vertex { x: 0, y: 0 };
        for l in s.lines() {
            let mut parts = l.split_whitespace();
            let dir = parts
                .next()
                .context(format!("Dig plan {l} is missing a direction"))?;
            let count: i64 = parts
                .next()
                .context(format!("Dig plan {l} is missing distance"))?
                .parse()
                .context(format!("Dig plan {l} has invalid distance"))?;
            let _ = parts
                .next()
                .context(format!("Dig plan {l} is missing color"))?;

            if parts.next().is_some() {
                return Err(anyhow!("Dig plan {l} has extra information"));
            }

            let mut next = prev;
            match dir {
                "U" => next.y -= count,
                "D" => next.y += count,
                "L" => next.x -= count,
                "R" => next.x += count,
                d => return Err(anyhow!("Invalid direction {d} in dig plan '{l}'")),
            };
            verts.push(next);
            prev = next;
        }

        Ok(Lagoon {
            verts: verts.into(),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! vert {
        ($x: expr, $y: expr) => {
            Vertex { x: $x, y: $y }
        };
    }

    #[test]
    fn lagoon_area() {
        // ###
        // ###
        // ###
        let lagoon = Lagoon {
            verts: vec![vert!(0, 0), vert!(2, 0), vert!(2, 2), vert!(0, 2)].into(),
        };
        assert_eq!(lagoon.area(), 9);

        // ###
        // ###
        // ##
        let l1 = Lagoon {
            verts: vec![
                vert!(0, 0),
                vert!(2, 0),
                vert!(2, 1),
                vert!(1, 1),
                vert!(1, 2),
                vert!(0, 2),
            ]
            .into(),
        };
        assert_eq!(l1.area(), 8);

        // #######
        // #######
        // #######
        // ..#####
        // ..#####
        // #######
        // #####..
        // #######
        // .######
        // .######
        let l2 = Lagoon {
            verts: vec![
                vert!(0, 0),
                vert!(6, 0),
                vert!(6, 5),
                vert!(4, 5),
                vert!(4, 7),
                vert!(6, 7),
                vert!(6, 9),
                vert!(1, 9),
                vert!(1, 7),
                vert!(0, 7),
                vert!(0, 5),
                vert!(2, 5),
                vert!(2, 2),
                vert!(0, 2),
            ]
            .into(),
        };
        assert_eq!(l2.area(), 62);
    }
}
