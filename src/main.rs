mod pig_latin;
use std::env;

use crate::pig_latin::to_pig_latin;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    args.iter()
        .map(|f| to_pig_latin(f))
        .for_each(|f| println!("{} ", f));
    println!("{}", to_pig_latin("apple"));
    println!("{}", to_pig_latin("first"));
}
