use std::collections::HashMap;

use serv_rs::toolbox::{
    money::{check_win, ways_win_strategy},
    reel_area::{ReelSet, ReelStrip, Spinnable, Symbol},
};

#[macro_use]
extern crate lazy_static;

lazy_static! {
    // Initialize symbols.
    pub static ref SYM0: Symbol = Symbol::new(String::from("SYM0"));
    pub static ref SYM1: Symbol = Symbol::new(String::from("SYM1"));
    pub static ref SYM2: Symbol = Symbol::new(String::from("SYM2"));
    pub static ref SYM3: Symbol = Symbol::new(String::from("SYM3"));
    pub static ref SYM4: Symbol = Symbol::new(String::from("SYM4"));
    pub static ref SYM5: Symbol = Symbol::new(String::from("SYM5"));
}

fn main() {
    let mut paytable: HashMap<&Symbol, HashMap<usize, usize>> = HashMap::new();
    let mut sym0 = HashMap::new();
    let mut sym1 = HashMap::new();
    let mut sym2 = HashMap::new();
    sym0.insert(1, 5);
    sym1.insert(2, 7);
    sym2.insert(3, 9);
    paytable.insert(&*SYM0, sym0);
    paytable.insert(&*SYM1, sym1);
    paytable.insert(&*SYM2, sym2);

    let mut reel_set = ReelSet::new(vec![
        ReelStrip::new(vec![&*SYM0, &*SYM1, &*SYM2, &*SYM3, &*SYM4, &*SYM5], 3),
        ReelStrip::new(vec![&*SYM0, &*SYM1, &*SYM2, &*SYM3, &*SYM4, &*SYM5], 3),
        ReelStrip::new(vec![&*SYM0, &*SYM1, &*SYM2, &*SYM3, &*SYM4, &*SYM5], 3),
    ]);

    let win_situations = check_win(&reel_set, &paytable, ways_win_strategy);

    println!("win situations: ");
    for ws in win_situations.iter() {
        println!(
            "ws: symbol={}; positions={:?}",
            ws.get_symbol(),
            ws.get_positions()
        );
    }

    // println!("{:?}", reel_set.get_visible_symbols());
    reel_set.spin();
    // println!("\n\n{:?}", reel_set.get_visible_symbols());
}
