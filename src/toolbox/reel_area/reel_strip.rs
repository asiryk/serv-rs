use super::{Reel, Spinnable};
use rand::prelude::*;

#[derive(Debug)]
pub struct ReelStrip<'a, S: 'a> {
    strip: Vec<&'a S>,
    visible_rows: usize,
    head: usize,
}

impl<'a, S> ReelStrip<'a, S> {
    pub fn new(strip: Vec<&'a S>, visible_rows: usize) -> Self {
        ReelStrip {
            strip,
            visible_rows,
            head: 0,
        }
    }

    fn get_strip_index(&self, row: usize) -> usize {
        let row = self.head + row;
        let len = self.strip.len();

        row - if row < len { 0 } else { len }
    }
}

impl<'a, S> Spinnable for ReelStrip<'a, S> {
    /// Randomly change self.head position on the reel strip.
    fn spin(&mut self) {
        let mut rng = thread_rng();
        self.head = rng.gen_range(0..self.strip.len());
    }
}

impl<'a, S> Reel<'a, S> for ReelStrip<'a, S> {
    /// Get symbol at row, starting from self.head.
    fn get_symbol(&self, row: usize) -> &S {
        &self.strip[self.get_strip_index(row)]
    }

    /// Set symbol directly to reel strip.
    fn set_symbol(&mut self, row: usize, symbol: &'a S) {
        self.strip.insert(self.get_strip_index(row), symbol);
    }

    /// Returns visible symbols of size self.visible_rows, starting from self.head.
    fn get_visible_symbols(&self) -> Vec<&S> {
        (0..self.visible_rows)
            .into_iter()
            .map(|i| self.get_symbol(i))
            .collect()
    }
}
