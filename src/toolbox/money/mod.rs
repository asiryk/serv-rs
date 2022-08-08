#![allow(dead_code, unused)]

use std::collections::{HashMap, HashSet};

use crate::toolbox::reel_area::ReelSet;

use super::reel_area::{Reel, SymbolPosition};

pub fn check_win<'a, S, R, F>(
    reel_set: &ReelSet<'a, S, R>,
    paytable: &HashMap<&'a S, HashMap<usize, usize>>,
    strategy: F,
) -> Vec<Box<dyn WinSituation<S>>>
where
    R: Reel<'a, S>,
    F: Fn(&'a S, &HashMap<usize, usize>, &ReelSet<'a, S, R>) -> Option<Box<dyn WinSituation<S>>>,
{
    paytable
        .iter()
        .map(move |(symbol, payments)| strategy(symbol, payments, reel_set))
        .flatten()
        .collect()
}

pub trait WinSituation<S> {
    fn get_coins(&self) -> usize;
    fn get_symbol(&self) -> &S;
    fn get_positions(&self) -> &Vec<SymbolPosition>;
}

#[derive(Debug)]
pub struct WaysWinSituation<S: 'static> {
    symbol: &'static S,
    coins: usize,
    positions: Vec<SymbolPosition>,
}

impl<S> WaysWinSituation<S> {
    pub fn new(symbol: &'static S, coins: usize, positions: Vec<SymbolPosition>) -> Self {
        WaysWinSituation {
            symbol,
            coins,
            positions,
        }
    }
}

impl<'a, S> WinSituation<S> for WaysWinSituation<S> {
    fn get_coins(&self) -> usize {
        self.coins
    }

    fn get_symbol(&self) -> &S {
        self.symbol
    }

    fn get_positions(&self) -> &Vec<SymbolPosition> {
        &self.positions
    }
}

pub fn ways_win_strategy<S, R>(
    symbol: &'static S,
    payments: &HashMap<usize, usize>,
    reel_set: &ReelSet<'static, S, R>,
) -> Option<Box<dyn WinSituation<S>>>
where
    S: Eq,
    R: Reel<'static, S>,
{
    let mut positions: Vec<SymbolPosition> = Vec::new();
    let mut ways = 1;
    for (reel_id, reel) in reel_set.get_reels().iter().enumerate() {
        let mut syms_per_reel = 0;
        for (row_id, reel_symbol) in reel.get_visible_symbols().iter().enumerate() {
            if symbol.eq(reel_symbol) {
                syms_per_reel += 1;
                positions.push(SymbolPosition::new(reel_id, row_id));
            }
        }

        if syms_per_reel == 0 {
            ways *= syms_per_reel;
        }
    }

    let max = payments
        .keys()
        .reduce(|acc, item| if acc < item { item } else { acc })?;

    let coins = *payments.get(std::cmp::min(&max, &positions.len()))?;

    Some(Box::new(WaysWinSituation::new(symbol, coins, positions)))
}
