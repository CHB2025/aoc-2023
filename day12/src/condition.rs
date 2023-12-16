use std::{collections::HashMap, num::ParseIntError, str::FromStr};

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

fn condition_match(base: &[Condition], test: &[Condition]) -> bool {
    assert_eq!(base.len(), test.len());
    for (b, t) in base.iter().zip(test.iter()) {
        if *b != Condition::Unknown && b != t {
            return false;
        }
    }
    true
}

#[derive(Debug, PartialEq, Eq, Clone)]
pub struct RowCondition {
    conditions: Box<[Condition]>,
    sequences: Box<[usize]>,
}

impl RowCondition {
    pub fn possible_combinations(&self) -> u64 {
        let mut combo = vec![Condition::Good; self.conditions.len()];

        let mut cache = HashMap::new();
        self.combo_rec(&mut cache, &mut combo, 0, &self.sequences)
    }

    fn combo_rec(
        &self,
        cache: &mut HashMap<(usize, usize), u64>,
        combo: &mut [Condition],
        filled: usize,
        sequence: &[usize],
    ) -> u64 {
        if sequence.is_empty() || filled >= combo.len() {
            // Check the rest to see if all good is ok
            if condition_match(
                &self.conditions[filled.min(combo.len())..],
                &combo[filled.min(combo.len())..],
            ) {
                return 1;
            } else {
                return 0;
            }
        }

        let mut count = 0;
        let space_remaining = combo.len() - filled;
        // Could take this out of the loop, but usually not too long?
        let space_needed: usize = sequence.iter().sum::<usize>() + sequence.len() - 1;
        let (seq, rest) = sequence.split_first().expect("checked size");

        for i in 0..=(space_remaining - space_needed) {
            let fill_range = filled + i..filled + i + seq;
            let check_range = filled..combo.len().min(filled + i + seq + 1);
            combo[fill_range.clone()].fill(Condition::Bad);

            // Only really need to check the current section (filled..filled+i+seq)
            // since previous section is known to match
            if condition_match(&self.conditions[check_range.clone()], &combo[check_range]) {
                let key = (rest.len(), filled + i + seq);
                let c = cache.get(&key);
                let c = if let Some(v) = c {
                    *v
                } else {
                    let c = self.combo_rec(cache, combo, filled + i + seq + 1, rest);
                    cache.insert(key, c);
                    c
                };
                count += c;
            }
            combo[fill_range].fill(Condition::Good);
        }
        count
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
