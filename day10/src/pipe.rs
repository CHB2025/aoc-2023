use std::{fmt, str};

use super::Point;
use anyhow::anyhow;

#[derive(Debug, PartialEq, Eq)]
pub enum Pipe {
    Ns,
    Ew,
    Ne,
    Nw,
    Sw,
    Se,
}

impl str::FromStr for Pipe {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "|" => Self::Ns,
            "-" => Self::Ew,
            "L" => Self::Ne,
            "J" => Self::Nw,
            "7" => Self::Sw,
            "F" => Self::Se,

            _ => return Err(anyhow!("Attempted to create Pipe from unknown str: {}", s)),
        })
    }
}

impl TryFrom<char> for Pipe {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        Ok(match value {
            '|' => Self::Ns,
            '-' => Self::Ew,
            'L' => Self::Ne,
            'J' => Self::Nw,
            '7' => Self::Sw,
            'F' => Self::Se,
            _ => {
                return Err(anyhow!(
                    "Attempted to create Pipe from unknown str: {}",
                    value
                ))
            }
        })
    }
}

impl fmt::Display for Pipe {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            Self::Ns => "|",
            Self::Ew => "-",
            Self::Ne => "L",
            Self::Nw => "J",
            Self::Sw => "7",
            Self::Se => "F",
        };
        write!(f, "{}", s)
    }
}

impl Pipe {
    pub fn connections(&self, start: Point) -> [Point; 2] {
        match self {
            Self::Ns => [Self::north(start), Self::south(start)],
            Self::Ew => [Self::east(start), Self::west(start)],
            Self::Ne => [Self::north(start), Self::east(start)],
            Self::Nw => [Self::north(start), Self::west(start)],
            Self::Sw => [Self::south(start), Self::west(start)],
            Self::Se => [Self::south(start), Self::east(start)],
        }
    }

    fn north((x, y): Point) -> Point {
        (x, y.saturating_sub(1))
    }

    fn east((x, y): Point) -> Point {
        (x + 1, y)
    }
    fn south((x, y): Point) -> Point {
        (x, y + 1)
    }
    fn west((x, y): Point) -> Point {
        (x.saturating_sub(1), y)
    }
}
