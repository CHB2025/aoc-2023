use std::str::FromStr;

use anyhow::{anyhow, Context, Error};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Part {
    pub x: usize,
    pub m: usize,
    pub a: usize,
    pub s: usize,
}

impl FromStr for Part {
    type Err = Error;

    fn from_str(part: &str) -> std::result::Result<Self, Self::Err> {
        let input = part.trim_matches(&['{', '}'][..]);
        let (mut x, mut m, mut a, mut s) = (None, None, None, None);
        let assign = |holder: &mut Option<usize>, value: usize| {
            if holder.is_some() {
                return Err(anyhow!("Multiple values for the same category: {part}"));
            }
            *holder = Some(value);
            Ok(())
        };

        for kv_pair in input.split(',').map(str::trim) {
            let (key, value) = kv_pair
                .split_once('=')
                .context("Invalid key-value pair in part {part}")?;
            let value = value.trim().parse::<usize>()?;
            match key.trim() {
                "x" => assign(&mut x, value)?,
                "m" => assign(&mut m, value)?,
                "a" => assign(&mut a, value)?,
                "s" => assign(&mut s, value)?,
                _ => return Err(anyhow!("Unknown key '{key}' in part")),
            };
        }

        let x = x.context("Missing x in part '{part}'")?;
        let m = m.context("Missing m in part '{part}'")?;
        let a = a.context("Missing a in part '{part}'")?;
        let s = s.context("Missing s in part '{part}'")?;

        Ok(Self { x, m, a, s })
    }
}
