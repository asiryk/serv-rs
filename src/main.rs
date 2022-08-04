use serv_rs::toolbox::reel_area::{Reel, ReelStrip, Spinnable, Symbol};

#[macro_use]
extern crate lazy_static;

lazy_static! {
    /// This is an example for using doc comment attributes
    static ref SYM0: Symbol = Symbol::new(String::from("SYM0"));
}

fn main() {
    let sym0 = Symbol::new(String::from("SYM0"));
    let sym1 = Symbol::new(String::from("SYM1"));
    let sym2 = Symbol::new(String::from("SYM2"));
    let sym3 = Symbol::new(String::from("SYM3"));
    let sym4 = Symbol::new(String::from("SYM4"));
    let sym5 = Symbol::new(String::from("SYM5"));

    let mut reels: Vec<ReelStrip<Symbol>> = vec![
        ReelStrip::new(vec![&sym0, &sym1, &sym2, &sym3, &sym4, &sym5], 3),
        ReelStrip::new(vec![&sym0, &sym1, &sym2, &sym3, &sym4, &sym5], 3),
        ReelStrip::new(vec![&sym0, &sym1, &sym2, &sym3, &sym4, &sym5], 3),
    ];

    reels.iter_mut().for_each(|reel| reel.spin());
    reels
        .iter()
        .for_each(|reel| println!("{:?}", reel.get_visible_symbols()));

    println!("{}", &*SYM0);
}
