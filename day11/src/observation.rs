use std::str::FromStr;

use anyhow::anyhow;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Observation {
    galaxies: Vec<(usize, usize)>,
}

impl Observation {
    pub fn shortest_paths(&self) -> usize {
        let mut sum = 0;
        for (i, g1) in self.galaxies.iter().enumerate() {
            for g2 in self.galaxies[(i + 1)..].iter() {
                sum += g1.0.abs_diff(g2.0);
                sum += g1.1.abs_diff(g2.1);
            }
        }
        sum
    }
}

impl FromStr for Observation {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut galaxies = s
            .lines()
            .enumerate()
            .flat_map(|(y, l)| {
                l.chars().enumerate().filter_map(move |(x, c)| match c {
                    '#' => Some(Ok((x, y))),
                    '.' => None,
                    _ => Some(Err(anyhow!("Illegal value in observation input: {}", c))),
                })
            })
            .collect::<Result<Vec<(usize, usize)>, anyhow::Error>>()?;
        // Currently sorted vertically
        let mut skips = 0;
        let mut last_y = 0;
        galaxies.iter_mut().for_each(|g| {
            skips += (g.1 - last_y).saturating_sub(1); // 1 or 0 = no skip
            last_y = g.1;
            g.1 += skips
        });

        galaxies.sort_unstable_by_key(|g| g.0);

        let mut skips = 0;
        let mut last_x = 0;
        galaxies.iter_mut().for_each(|g| {
            skips += (g.0 - last_x).saturating_sub(1);
            last_x = g.0;
            g.0 += skips
        });

        Ok(Observation { galaxies })
    }
}
