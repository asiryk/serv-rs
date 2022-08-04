use std::collections::HashMap;

use super::{Reel, Spinnable, SymbolPosition};

pub struct ReelSet<'a, S: 'a> {
    reels: Vec<Box<dyn Reel<'a, S>>>, // todo refactor to static dispatch
}

impl<'a, S> ReelSet<'a, S> {
    fn get_visible_symbols(&'a self) -> HashMap<SymbolPosition, &'a S> {
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

    fn get_symbol(&'a self, pos: SymbolPosition) -> Option<&'a S> {
        let reel = self.reels.get(pos.reel)?;
        let symbol = reel.get_symbol(pos.row)?;
        Some(symbol)
    }

    fn set_symbol(&mut self, pos: SymbolPosition, sym: &'a S) {
        // if let Some(reel) = self.reels.as_mut().get(pos.reel) {
        //     reel.set_symbol(pos.row, sym);
        // }
    }
}

// impl<'a, S> Spinnable for ReelSet<'a, S> {
//     fn spin(&mut self) {
//         self.reels.spin();
//     }
// }
