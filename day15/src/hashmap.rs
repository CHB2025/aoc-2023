use super::ascii_hash;
use std::array;

// It's an acronym, so all lowercase
pub struct Hashmap<'a> {
    boxes: [Vec<(&'a str, usize)>; 256],
}

impl<'a> Hashmap<'a> {
    pub fn new() -> Self {
        Self {
            boxes: array::from_fn(|_| Vec::new()),
        }
    }
    pub fn add(&mut self, key: &'a str, value: usize) {
        let ind = ascii_hash(key.as_bytes());
        let bucket = self
            .boxes
            .get_mut(ind)
            .expect("Ascii hash always between 0 and 255");
        if let Some(kv) = bucket.iter_mut().find(|(k, _)| *k == key) {
            kv.1 = value;
        } else {
            bucket.push((key, value));
        }
    }

    pub fn remove(&mut self, key: &'a str) {
        let ind = ascii_hash(key.as_bytes());
        self.boxes
            .get_mut(ind)
            .expect("Ascii hash always between 0 and 255")
            .retain(|(k, _)| *k != key);
    }

    pub fn power(&self) -> usize {
        self.boxes
            .iter()
            .enumerate()
            .flat_map(|(mi, boks)| {
                boks.iter()
                    .enumerate()
                    .map(move |(bi, (_, f))| (mi + 1) * (bi + 1) * f)
            })
            .sum()
    }
}
