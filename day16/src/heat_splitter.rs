use std::{
    collections::{BTreeMap, BTreeSet},
    mem,
    str::FromStr,
};

use anyhow::anyhow;

#[derive(Clone, Copy, PartialEq, Eq, Debug, PartialOrd, Ord, Hash)]
pub enum Direction {
    Up,
    Down,
    Left,
    Right,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
pub struct Beam {
    pub direction: Direction,
    pub x: usize,
    pub y: usize,
}

impl Beam {
    pub fn step(mut self) -> Option<Self> {
        match self.direction {
            Direction::Up => self.y = self.y.checked_sub(1)?,
            Direction::Down => self.y = self.y.checked_add(1)?,
            Direction::Left => self.x = self.x.checked_sub(1)?,
            Direction::Right => self.x = self.x.checked_add(1)?,
        }
        Some(self)
    }
}

/// Named based on the direction the top of the mirror leans
#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Mirror {
    Right,
    Left,
}

impl Mirror {
    fn turn(self, mut beam: Beam) -> Beam {
        match (self, beam.direction) {
            (Self::Left, Direction::Up) => beam.direction = Direction::Left,
            (Self::Left, Direction::Left) => beam.direction = Direction::Up,
            (Self::Left, Direction::Down) => beam.direction = Direction::Right,
            (Self::Left, Direction::Right) => beam.direction = Direction::Down,

            (Self::Right, Direction::Up) => beam.direction = Direction::Right,
            (Self::Right, Direction::Right) => beam.direction = Direction::Up,
            (Self::Right, Direction::Down) => beam.direction = Direction::Left,
            (Self::Right, Direction::Left) => beam.direction = Direction::Down,
        }
        beam
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Splitter {
    Horizontal,
    Vertical,
}

impl Splitter {
    fn split(self, beam: Beam) -> Option<(Beam, Beam)> {
        match (self, beam.direction) {
            (Self::Horizontal, Direction::Left | Direction::Right) => None,
            (Self::Horizontal, _) => Some((
                Beam {
                    direction: Direction::Left,
                    x: beam.x,
                    y: beam.y,
                },
                Beam {
                    direction: Direction::Right,
                    x: beam.x,
                    y: beam.y,
                },
            )),
            (Self::Vertical, Direction::Up | Direction::Down) => None,
            (Self::Vertical, _) => Some((
                Beam {
                    direction: Direction::Up,
                    x: beam.x,
                    y: beam.y,
                },
                Beam {
                    direction: Direction::Down,
                    x: beam.x,
                    y: beam.y,
                },
            )),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
enum Cell {
    Empty,
    Mirror(Mirror),
    Splitter(Splitter),
}

impl Cell {
    fn activate(self, beam: Beam) -> Vec<Beam> {
        // Only really needs one or two
        let mut ret = Vec::new();
        match self {
            Self::Empty => {
                if let Some(b) = beam.step() {
                    ret.push(b);
                }
            }
            Cell::Mirror(m) => {
                if let Some(b) = m.turn(beam).step() {
                    ret.push(b);
                }
            }
            Cell::Splitter(s) => {
                if let Some((a, b)) = s.split(beam) {
                    if let Some(b) = b.step() {
                        ret.push(b);
                    }
                    if let Some(a) = a.step() {
                        ret.push(a);
                    }
                } else if let Some(b) = beam.step() {
                    ret.push(b);
                }
            }
        }
        ret
    }
}

impl TryFrom<char> for Cell {
    type Error = anyhow::Error;

    fn try_from(value: char) -> Result<Self, Self::Error> {
        match value {
            '.' => Ok(Self::Empty),
            '/' => Ok(Self::Mirror(Mirror::Right)),
            '\\' => Ok(Self::Mirror(Mirror::Left)),
            '|' => Ok(Self::Splitter(Splitter::Vertical)),
            '-' => Ok(Self::Splitter(Splitter::Horizontal)),
            c => Err(anyhow!("Cannot convert '{c}' to Cell")),
        }
    }
}

#[derive(Clone, PartialEq, Eq, Debug)]
pub struct HeatSplitter {
    cells: Box<[Cell]>,
    width: usize,
}

impl HeatSplitter {
    pub fn energize(&self, input: Beam) -> usize {
        let mut beams = Vec::new();
        beams.push(input);

        let mut historic_beams = BTreeMap::new();
        historic_beams
            .entry((input.x, input.y))
            .or_insert(BTreeSet::new())
            .insert(input.direction);

        while !beams.is_empty() {
            self.step(&mut beams);
            // Keep all new ones
            beams.retain(|b| {
                historic_beams
                    .entry((b.x, b.y))
                    .or_insert(BTreeSet::new())
                    .insert(b.direction)
            });
        }
        historic_beams.len()
    }
    pub fn step(&self, beams: &mut Vec<Beam>) {
        let prev = mem::replace(beams, Vec::with_capacity(beams.len()));

        beams.extend(prev.into_iter().flat_map(|b| {
            self.get(b.x, b.y)
                .unwrap_or(Cell::Empty)
                .activate(b)
                .into_iter()
                .filter(|b| b.x < self.width && b.y < self.height())
        }));
    }

    fn get(&self, x: usize, y: usize) -> Option<Cell> {
        self.cells.get(y * self.width + x).copied()
    }

    pub fn height(&self) -> usize {
        self.cells.len() / self.width
    }
    pub fn width(&self) -> usize {
        self.width
    }
}

impl FromStr for HeatSplitter {
    type Err = anyhow::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let width = s.lines().next().map_or(0, |l| l.len());
        let mut cells = Vec::with_capacity(s.len());
        for l in s.lines() {
            if l.len() != width {
                return Err(anyhow!("Lines in input with differing lengths."));
            }
            for c in l.chars() {
                cells.push(Cell::try_from(c)?);
            }
        }
        Ok(Self {
            cells: cells.into(),
            width,
        })
    }
}
