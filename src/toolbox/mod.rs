#![allow(dead_code)]

pub mod reel_area {
    pub use reel_strip::ReelStrip;
    pub use symbol_position::SymbolPosition;
    pub use symbols::Symbol;

    mod symbol_position {
        pub struct SymbolPosition {
            pub reel: u8,
            pub row: u8,
        }

        impl SymbolPosition {
            pub fn new(reel: u8, row: u8) -> Self {
                SymbolPosition { reel, row }
            }
        }
    }

    pub trait Reel<'a, S: 'a> {
        fn get_symbol(&self, row: usize) -> &S;
        fn set_symbol(&mut self, row: usize, symbol: &S);
        fn get_visible_symbols(&self) -> Vec<&S>;
    }

    pub trait Spinnable {
        fn spin(&mut self);
    }

    mod reel_strip {
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
            fn spin(&mut self) {
                let mut rng = thread_rng();
                self.head = rng.gen_range(0..self.strip.len());
            }
        }

        impl<'a, S> Reel<'a, S> for ReelStrip<'a, S> {
            fn get_symbol(&self, row: usize) -> &S {
                &self.strip[self.get_strip_index(row)]
            }

            fn set_symbol(&mut self, _row: usize, _symbol: &S) {
                todo!("gotta deal with lifetimes");
                // self.strip.insert(self.get_strip_index(row), symbol);
            }

            fn get_visible_symbols(&self) -> Vec<&S> {
                (0..self.visible_rows)
                    .into_iter()
                    .map(|i| self.get_symbol(i))
                    .collect()
            }
        }
    }

    mod symbols {
        use std::fmt::Display;

        #[derive(Hash, PartialEq, Eq, Debug, Clone)]
        pub struct Symbol {
            name: String,
        }

        impl Symbol {
            pub fn new(name: String) -> Self {
                Symbol { name }
            }
        }

        impl Display for Symbol {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "{}", self.name)
            }
        }
    }
}
