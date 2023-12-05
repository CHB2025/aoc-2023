use std::{num::ParseIntError, str::FromStr};

use anyhow::anyhow;

#[derive(Debug)]
pub struct Card {
    id: usize,
    win: Box<[u32]>,
    given: Box<[u32]>,
}

impl FromStr for Card {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (header, numbers) = s
            .split_once(':')
            .ok_or(anyhow!("Improperly formatted card: Missing header"))?;
        let (winners, yours) = numbers.trim().split_once('|').ok_or(anyhow!(
            "Improperly formatted card: Missing number sections"
        ))?;
        Ok(Self {
            id: header
                .split_whitespace()
                .nth(1)
                .ok_or(anyhow!("Improperly formatted card: Missing id number"))?
                .parse()?,
            win: winners
                .split_whitespace()
                .map(u32::from_str)
                .collect::<Result<Box<[u32]>, ParseIntError>>()?,
            given: yours
                .split_whitespace()
                .map(u32::from_str)
                .collect::<Result<Box<[u32]>, ParseIntError>>()?,
        })
    }
}

impl Card {
    pub fn id(&self) -> usize {
        self.id
    }

    pub fn score(&self) -> u32 {
        let mut score = 0;
        for num in self.given.iter() {
            if self.win.contains(num) {
                if score == 0 {
                    score = 1;
                } else {
                    score *= 2;
                }
            }
        }
        score
    }

    pub fn matches(&self) -> usize {
        self.given.iter().filter(|n| self.win.contains(n)).count()
    }
}
