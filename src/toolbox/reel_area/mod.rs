mod reel_set;
mod reel_strip;
mod symbols;

pub use reel_set::ReelSet;
pub use reel_strip::ReelStrip;
pub use symbols::Symbol;
pub use symbols::SymbolPosition;

pub trait Reel<'a, S: 'a> {
    fn get_symbol(&self, row: usize) -> Option<&S>;
    fn set_symbol(&mut self, row: usize, symbol: &'a S);
    fn get_visible_symbols(&self) -> Vec<&S>;
}

pub trait Spinnable {
    fn spin(&mut self);
}
