#[derive(Debug)]
pub struct Schematic<'a> {
    raw: Box<[&'a str]>,
    width: usize,
}

impl<'a> Schematic<'a> {
    pub fn new(input: &'a str) -> Self {
        let mut line_length = None;
        Schematic {
            raw: input
                .lines()
                .map(|l| {
                    if let Some(exp) = line_length {
                        assert_eq!(l.len(), exp);
                    } else {
                        line_length = Some(l.len());
                    }
                    l
                })
                .collect(),
            width: line_length.unwrap(),
        }
    }

    fn number_ind(&'a self) -> Numbers<impl Iterator<Item = (usize, usize, u32)> + 'a> {
        Numbers::new(self.raw.iter().enumerate().flat_map(|(y, &inner)| {
            inner
                .match_indices(char::is_numeric)
                .map(move |(x, txt)| (x, y, txt.parse().unwrap()))
        }))
    }

    pub fn part_numbers(&'a self) -> impl Iterator<Item = u32> + 'a {
        self.number_ind()
            .filter_map(|v| if self.is_part_num(v) { Some(v.2) } else { None })
    }

    fn is_part_num(&'a self, (x, y, n): (usize, usize, u32)) -> bool {
        let num_width = n.to_string().len();
        for x in x.saturating_sub(1)..self.width.min(x + num_width + 1) {
            for y in y.saturating_sub(1)..self.raw.len().min(y + 2) {
                if self
                    .get(x, y)
                    .is_some_and(|c| c != '.' && c.is_ascii_punctuation())
                {
                    return true;
                }
            }
        }

        false
    }

    fn get(&'a self, x: usize, y: usize) -> Option<char> {
        self.raw[y].chars().nth(x)
    }

    fn get_num(&'a self, x: usize, y: usize) -> Option<(usize, usize, u32)> {
        //start by going backwards, then go forwards, picking up full number
        if !self.get(x, y).is_some_and(|c| c.is_numeric()) {
            return None;
        }
        let mut start = x;
        // Not particularly efficient
        while start > 0 && self.get(start - 1, y).is_some_and(|c| c.is_numeric()) {
            start -= 1;
        }

        let mut end = x;
        while end < self.width && self.get(end + 1, y).is_some_and(|c| c.is_numeric()) {
            end += 1;
        }

        Some((start, y, self.raw[y][start..=end].parse().unwrap()))
    }

    pub fn gears(&'a self) -> impl Iterator<Item = Gear> + 'a {
        self.raw.iter().enumerate().flat_map(move |(y, &line)| {
            line.chars().enumerate().filter_map(move |(x, c)| {
                if c == '*' {
                    Gear::from_star(self, (x, y))
                } else {
                    None
                }
            })
        })
    }
}

#[derive(Debug, PartialEq, Eq)]
struct Numbers<I>
where
    I: Iterator<Item = (usize, usize, u32)>,
{
    iter: I,
    next_start: Option<(usize, usize, u32)>,
}

impl<I> Numbers<I>
where
    I: Iterator<Item = (usize, usize, u32)>,
{
    pub fn new(mut iter: I) -> Self {
        let next = iter.next();
        Self {
            iter,
            next_start: next,
        }
    }
}

impl<I> Iterator for Numbers<I>
where
    I: Iterator<Item = (usize, usize, u32)>,
{
    type Item = (usize, usize, u32);

    fn next(&mut self) -> Option<Self::Item> {
        let (x, y, mut num) = self.next_start?;
        let mut exp_x = x + 1;
        for (nx, ny, d) in self.iter.by_ref() {
            if nx == exp_x && ny == y {
                num *= 10;
                num += d;
                exp_x += 1;
            } else {
                self.next_start = Some((nx, ny, d));
                break;
            }
        }
        if let Some((nx, ny, _)) = self.next_start {
            if nx == x && ny == y {
                // Emptied iterator
                self.next_start = None;
            }
        }

        Some((x, y, num))
    }
}

pub struct Gear(u32, u32);

impl Gear {
    pub fn from_star(sc: &Schematic, (x, y): (usize, usize)) -> Option<Self> {
        let (mut first, mut second) = (None, None);
        for y in y.saturating_sub(1)..sc.raw.len().min(y + 2) {
            let mut s_x = x.saturating_sub(1);
            while s_x < sc.width.min(x + 2) {
                if let Some(num) = sc.get_num(s_x, y) {
                    if first.is_none() {
                        first = Some(num.2);
                    } else if second.is_none() {
                        second = Some(num.2);
                    } else {
                        // Three numbers around this star
                        return None;
                    }
                    // Move to the end of the number
                    s_x = num.0 + num.2.to_string().len();
                } else {
                    s_x += 1;
                }
            }
        }

        if let Some((first, second)) = first.zip(second) {
            Some(Gear(first, second))
        } else {
            None
        }
    }

    pub fn ratio(&self) -> u32 {
        self.0 * self.1
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST_INPUT: &str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";

    #[test]
    pub fn test_number_ind() {
        let sc = Schematic::new(TEST_INPUT);
        let expected = [
            (0, 0, 467),
            (5, 0, 114),
            (2, 2, 35),
            (6, 2, 633),
            (0, 4, 617),
            (7, 5, 58),
            (2, 6, 592),
            (6, 7, 755),
            (1, 9, 664),
            (5, 9, 598),
        ]
        .into_iter();

        sc.number_ind().for_each(|(_, _, n)| eprintln!("{}", n));

        assert!(sc.number_ind().eq(expected))
    }
}
