use anyhow::anyhow;
use std::{num::ParseIntError, str::FromStr};

#[derive(Debug)]
pub struct Almanac2 {
    seeds: Box<[(u64, u64)]>,
    maps: Box<[Map]>,
}

impl FromStr for Almanac2 {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (seeds, rest) = s
            .split_once("\n\n")
            .ok_or(anyhow!("Could not find seed numbers in input"))?;
        let seeds = seeds
            .split_once(':')
            .ok_or(anyhow!("Couldn't split seed input: {}", seeds))?
            .1
            .split_whitespace()
            .map(u64::from_str)
            .collect::<Result<Vec<u64>, ParseIntError>>()?;
        let seeds = seeds.chunks_exact(2).map(|c| (c[0], c[1])).collect();
        let maps: Box<[Map]> = rest
            .split("\n\n")
            .map(Map::from_str)
            .collect::<Result<Box<[Map]>, anyhow::Error>>()?;
        Ok(Self { seeds, maps })
    }
}

impl Almanac2 {
    /// Could definately use some optimization. Very slow on the large ranges of input in test 2
    pub fn locations(&self) -> impl Iterator<Item = u64> + '_ {
        self.seeds.iter().flat_map(|&(rs, rl)| {
            (rs..(rs + rl)).map(|s| {
                let mut d = s;
                for map in &self.maps[..] {
                    d = map.get(d);
                }
                d
            })
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Map {
    /// (dest, source, length)
    ranges: Box<[(u64, u64, u64)]>,
}

impl FromStr for Map {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines();
        let _ = lines.next().ok_or(anyhow!("Map missing title"))?.to_owned();
        let ranges: Box<[(u64, u64, u64)]> = lines
            .map(|l| {
                let nums: Vec<_> = l
                    .split(' ')
                    .map(u64::from_str)
                    .collect::<Result<Vec<u64>, ParseIntError>>()?;

                let [ds, ss, l] = nums[..] else {
                    return Err(anyhow!(
                        "Incorrect number of numbers for a map. Expected 3, but got {}",
                        nums.len()
                    ));
                };
                Ok((ds, ss, l))
            })
            .collect::<Result<Box<[(u64, u64, u64)]>, anyhow::Error>>()?;

        Ok(Map { ranges })
    }
}

impl Map {
    /// Takes a source number and converts it to the destination number
    /// If the number is not in any of the source ranges, the destination
    /// number is the same as the source.
    /// If the source is in a range. The destination is the corresponding
    /// destination number in the range
    fn get(&self, source: u64) -> u64 {
        for range in self.ranges.iter() {
            if source >= range.1 && source < range.1 + range.2 {
                return range.0 + (source - range.1);
            }
        }
        source
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_get() {
        let map = Map {
            ranges: vec![(50, 98, 2)].into_boxed_slice(),
        };
        assert_eq!(map.get(97), 97);
        assert_eq!(map.get(98), 50);
        assert_eq!(map.get(99), 51);
        assert_eq!(map.get(100), 100);
    }
}
