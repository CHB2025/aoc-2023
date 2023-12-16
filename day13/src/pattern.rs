pub struct Pattern<'a> {
    rows: Box<[&'a str]>,
}

impl<'a> Pattern<'a> {
    pub fn new(s: &'a str) -> Self {
        // Should check all rows are equal length
        Pattern {
            rows: s.lines().collect(),
        }
    }

    pub fn get_row(&self, i: usize) -> Option<String> {
        self.rows.get(i).map(|&s| s.to_owned())
    }
    pub fn get_column(&self, i: usize) -> Option<String> {
        self.rows.iter().map(|r| r.chars().nth(i)).collect()
    }

    pub fn rows(&self) -> usize {
        self.rows.len()
    }

    pub fn columns(&self) -> usize {
        self.rows.first().map_or(0, |r| r.len())
    }
}
