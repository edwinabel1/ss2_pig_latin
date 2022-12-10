mod pig_latin;
use rayon::prelude::*;
use std::env;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    let output: Vec<String> = args
        .par_iter()
        .map(|f| pig_latin::to_pig_latin(f))
        .collect();
    output.iter().for_each(|f| print!("{} ", f));
    println!();
}
