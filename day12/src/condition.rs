use std::{num::ParseIntError, str::FromStr};

use anyhow::anyhow;

#[derive(Default, Debug, PartialEq, Eq, Clone, Copy)]
enum Condition {
    Good,
    Bad,
    #[default]
    Unknown,
}

impl TryFrom<char> for Condition {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '.' => Self::Good,
            '#' => Self::Bad,
            '?' => Self::Unknown,
            _ => return Err(anyhow!("Unknown char for condition")),
        })
    }
}

impl Condition {
    #[allow(unused)]
    pub fn row_string(slice: &[Condition]) -> String {
        slice
            .iter()
            .map(|c| match c {
                Self::Good => '.',
                Self::Bad => '#',
                Self::Unknown => '?',
            })
            .collect()
    }
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RowCondition {
    conditions: Box<[Condition]>,
    sequences: Box<[usize]>,
}

impl RowCondition {
    pub fn possible_combinations(&self) -> u32 {
        let mut combo = vec![Condition::Good; self.conditions.len()];

        self.combo_rec(&mut combo, 0, &self.sequences)
    }

    fn combo_rec(&self, combo: &mut [Condition], filled: usize, sequence: &[usize]) -> u32 {
        if sequence.is_empty() || filled >= combo.len() {
            if self.is_partial_match(combo) {
                return 1; // exactly one way to fill
            } else {
                return 0;
            }
        }

        let mut count = 0;
        let space_needed: usize = sequence.iter().sum::<usize>() + sequence.len() - 1;

        for i in 0..=(combo.len() - (space_needed + filled)) {
            let (seq, rest) = sequence.split_first().expect("checked size");

            let fill = filled + i;

            combo[fill..fill + seq].fill(Condition::Bad);
            if self.is_partial_match(&combo[..(fill + seq + 1).min(combo.len())]) {
                count += Self::combo_rec(self, combo, filled + i + seq + 1, rest);
                // +1 for empty space
            }
            combo[fill..fill + seq].fill(Condition::Good);
        }
        count
    }

    // checks if given sequence could match the known conditions.
    // Doesn't worry about checking if the sequence is possible
    // Only checks up to the length of the test sequence
    fn is_partial_match(&self, test: &[Condition]) -> bool {
        assert!(self.conditions.len() >= test.len());
        for (i, test) in test.iter().enumerate() {
            if self.conditions[i] != Condition::Unknown && self.conditions[i] != *test {
                return false;
            }
        }
        true
    }
}

impl FromStr for RowCondition {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (conditions, sequences) = s
            .split_once(' ')
            .ok_or(anyhow!("Unknown format for a condition row"))?;
        Ok(Self {
            conditions: conditions
                .chars()
                .map(Condition::try_from)
                .collect::<Result<Box<[Condition]>, anyhow::Error>>()?,
            sequences: sequences
                .split(',')
                .map(usize::from_str)
                .collect::<Result<Box<[usize]>, ParseIntError>>()?,
        })
    }
}
