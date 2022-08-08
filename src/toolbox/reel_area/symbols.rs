use std::fmt::Display;

// todo probably use Symbol as an Enum
// e.g
//
// struct GameFeature;
//
// enum Symbol {
//     SYM1,
//     SYM2,
//     SYM3(GameFeature),
// }

#[derive(Hash, PartialEq, Eq, Debug, Clone)]
pub struct Symbol {
    name: String,
}

impl Symbol {
    pub fn new(name: String) -> Self {
        Symbol { name }
    }

    pub fn get_name(&self) -> &str {
        &self.name
    }
}

impl Display for Symbol {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Hash, PartialEq, Eq, PartialOrd, Ord, Debug)]
pub struct SymbolPosition {
    pub reel: usize,
    pub row: usize,
}

impl SymbolPosition {
    pub fn new(reel: usize, row: usize) -> Self {
        SymbolPosition { reel, row }
    }
}
