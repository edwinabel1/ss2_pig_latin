mod pig_latin;
use std::env;

fn main() {
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    args.iter()
        .map(|f| pig_latin::to_pig_latin(f))
        .for_each(|f| print!("{} ", f));
    println!();
}
