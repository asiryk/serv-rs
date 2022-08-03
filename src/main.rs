use crate::toolbox::reel_area::{Reel, ReelStrip, Spinnable, Symbol};

mod toolbox;

fn main() {
    let sym0 = Symbol::new(String::from("SYM0"));
    let sym1 = Symbol::new(String::from("SYM1"));
    let sym2 = Symbol::new(String::from("SYM2"));
    let sym3 = Symbol::new(String::from("SYM3"));
    let sym4 = Symbol::new(String::from("SYM4"));
    let sym5 = Symbol::new(String::from("SYM5"));

    let mut strip = ReelStrip::new(vec![&sym0, &sym1, &sym2, &sym3, &sym4, &sym5], 3);

    println!("{:?}", strip.get_visible_symbols());

    strip.spin();

    println!("{:?}", strip.get_visible_symbols());
}
