use std::fmt::Display;

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

pub struct SymbolPosition {
    pub reel: u8,
    pub row: u8,
}

impl SymbolPosition {
    pub fn new(reel: u8, row: u8) -> Self {
        SymbolPosition { reel, row }
    }
}
