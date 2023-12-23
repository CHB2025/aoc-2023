use std::{
    fmt::Display,
    ops::{Add, Index, Sub},
};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct Point {
    pub x: usize,
    pub y: usize,
}

impl Point {
    pub fn distance(&self, other: &Self) -> usize {
        self.x.abs_diff(other.x) + self.y.abs_diff(other.y)
    }
}

impl Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}

impl Add<(usize, usize)> for Point {
    type Output = Option<Point>;

    fn add(self, (x, y): (usize, usize)) -> Self::Output {
        Some(Point {
            x: self.x.checked_add(x)?,
            y: self.y.checked_add(y)?,
        })
    }
}

impl Sub<(usize, usize)> for Point {
    type Output = Option<Point>;

    fn sub(self, (x, y): (usize, usize)) -> Self::Output {
        Some(Point {
            x: self.x.checked_sub(x)?,
            y: self.y.checked_sub(y)?,
        })
    }
}

impl<T> Index<Point> for [&[T]] {
    type Output = T;

    fn index(&self, index: Point) -> &Self::Output {
        &self[index.y][index.x]
    }
}

impl<T> Index<Point> for Box<[Box<[T]>]> {
    type Output = T;

    fn index(&self, index: Point) -> &Self::Output {
        &self.as_ref()[index.y][index.x]
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn add_points() {
        let start = Point { x: 0, y: 1 };
        let target = start + (0, 1);
        assert!(target.is_some());
        assert_eq!(target.unwrap().y, 2);
    }
}
