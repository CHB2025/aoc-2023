use std::str::FromStr;

use anyhow::anyhow;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum Rock {
    Round,
    Cube,
}

impl TryFrom<char> for Rock {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            'O' => Ok(Self::Round),
            '#' => Ok(Self::Cube),
            c => Err(anyhow!("Unknown char {c} used to make rock")),
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    North,
    South,
    East,
    West,
}

impl Direction {
    fn y_range(self, min: usize, max: usize) -> Box<dyn Iterator<Item = usize>> {
        let range = min..max;
        match self {
            Self::South => Box::new(range.rev()),
            _ => Box::new(range),
        }
    }
    fn x_range(self, min: usize, max: usize) -> Box<dyn Iterator<Item = usize>> {
        let range = min..max;
        match self {
            Self::East => Box::new(range.rev()),
            _ => Box::new(range),
        }
    }

    fn add_offset(self, x: usize, y: usize) -> Option<(usize, usize)> {
        match self {
            Self::North => Some(x).zip(y.checked_sub(1)),
            Self::South => Some(x).zip(y.checked_add(1)),
            Self::East => x.checked_add(1).zip(Some(y)),
            Self::West => x.checked_sub(1).zip(Some(y)),
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub struct Platform {
    rocks: Box<[Option<Rock>]>,
    width: usize,
}

impl Platform {
    pub fn width(&self) -> usize {
        self.width
    }
    pub fn height(&self) -> usize {
        self.rocks.len() / self.width
    }
    pub fn load(&self) -> usize {
        let mut load = 0;
        for col in 0..self.height() {
            for row in 0..self.width() {
                if self.get(row, col) == Some(Rock::Round) {
                    load += self.height() - col;
                }
            }
        }
        load
    }
    pub fn tilt(&mut self, dir: Direction) {
        for y in dir.y_range(0, self.height()) {
            for x in dir.x_range(0, self.width()) {
                if self.get(x, y) == Some(Rock::Round) {
                    let (mut nx, mut ny) = (x, y);
                    while let Some(pos) = dir.add_offset(nx, ny).filter(|(x, y)| {
                        *x < self.width && *y < self.height() && self.get(*x, *y).is_none()
                    }) {
                        (nx, ny) = pos;
                    }
                    self.set(x, y, None);
                    self.set(nx, ny, Some(Rock::Round));
                }
            }
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<Rock> {
        self.rocks.get(y * self.width + x).copied().flatten()
    }
    pub fn set(&mut self, x: usize, y: usize, rock: Option<Rock>) -> bool {
        match self.rocks.get_mut(y * self.width + x) {
            Some(r) => *r = rock,
            None => return false,
        }
        true
    }

    pub fn spin(&mut self) {
        self.tilt(Direction::North);
        self.tilt(Direction::West);
        self.tilt(Direction::South);
        self.tilt(Direction::East);
    }
}

impl FromStr for Platform {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut width = None;
        let mut rocks = Vec::new();
        for l in s.lines() {
            if let Some(w) = width {
                if l.len() != w {
                    return Err(anyhow!("Not all lines are the same length"));
                }
            } else {
                width = Some(l.len());
            }
            rocks.extend(l.chars().map(|c| Rock::try_from(c).ok()))
        }
        Ok(Self {
            rocks: rocks.into(),
            width: width.unwrap_or(0),
        })
    }
}
