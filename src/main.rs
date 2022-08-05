use serv_rs::toolbox::reel_area::{ReelSet, ReelStrip, Spinnable, Symbol};

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
    let mut reel_set = ReelSet::new(vec![
        ReelStrip::new(vec![&*SYM0, &*SYM1, &*SYM2, &*SYM3, &*SYM4, &*SYM5], 3),
        ReelStrip::new(vec![&*SYM0, &*SYM1, &*SYM2, &*SYM3, &*SYM4, &*SYM5], 3),
        ReelStrip::new(vec![&*SYM0, &*SYM1, &*SYM2, &*SYM3, &*SYM4, &*SYM5], 3),
    ]);

    println!("{:?}", reel_set.get_visible_symbols());

    reel_set.spin();

    println!("\n\n{:?}", reel_set.get_visible_symbols());
}
