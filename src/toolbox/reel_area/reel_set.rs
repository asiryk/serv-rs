use std::{collections::HashMap, marker::PhantomData};

use super::{Reel, Spinnable, SymbolPosition};

#[derive(Debug)]
pub struct ReelSet<'a, S: 'a, R>
where
    R: Reel<'a, S>,
{
    reels: Vec<R>,

    /// I have no idea what am I doing, but seems it is possible
    /// to avoid dynamic dispatch with PhantomData.
    phantom: PhantomData<&'a S>,
}

impl<'a, S, R> ReelSet<'a, S, R>
where
    R: Reel<'a, S>,
{
    pub fn new(reels: Vec<R>) -> Self {
        ReelSet {
            reels,
            phantom: PhantomData,
        }
    }

    pub fn get_visible_symbols(&'a self) -> HashMap<SymbolPosition, &'a S> {
        self.reels
            .iter()
            .enumerate()
            .flat_map(|(reel_id, reel)| {
                reel.get_visible_symbols()
                    .iter()
                    .enumerate()
                    .map(move |(row_id, symbol)| (SymbolPosition::new(reel_id, row_id), *symbol))
                    .collect::<Vec<_>>()
            })
            .collect()
    }

    pub fn get_symbol(&'a self, pos: SymbolPosition) -> Option<&'a S> {
        let reel = self.reels.get(pos.reel)?;
        let symbol = reel.get_symbol(pos.row)?;
        Some(symbol)
    }

    pub fn set_symbol(&mut self, pos: SymbolPosition, symbol: &'a S) {
        if let Some(reel) = self.reels.get_mut(pos.reel) {
            reel.set_symbol(pos.row, symbol);
        }
    }
}

impl<'a, S, R> Spinnable for ReelSet<'a, S, R>
where
    R: Reel<'a, S> + Spinnable,
{
    /// Spin all reels. Requires reels to implement Spinnable trait.
    fn spin(&mut self) {
        self.reels.iter_mut().for_each(|reel| reel.spin());
    }
}
