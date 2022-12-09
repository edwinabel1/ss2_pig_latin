mod pig_latin;
use crate::pig_latin::to_pig_latin;

fn main() {
    println!("{}", to_pig_latin("apple"));
    println!("{}", to_pig_latin("first"));
}
