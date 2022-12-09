const VOCAL: [char; 5] = ['a', 'e', 'i', 'o', 'u'];
pub fn to_pig_latin(word: &str) -> String {
    let first_char = word.chars().next().unwrap();
    if VOCAL.contains(&first_char) {
        vocal(word)
    } else {
        consonantes(word)
    }
}

fn vocal(word: &str) -> String {
    let mut word = String::from(word);
    word.push_str("-hay");
    word
}

fn consonantes(word: &str) -> String {
    let mut word = String::from(word);
    let a = word.remove(0);
    word.push('-');
    word.push(a);
    word.push_str("ay");
    word
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_to_pig_latin() {
        assert_eq!(to_pig_latin("first"), "irst-fay");
        assert_eq!(to_pig_latin("apple"), "apple-hay")
    }
}
