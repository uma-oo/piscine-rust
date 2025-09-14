use std::collections::HashSet;

pub fn is_pangram(s: &str) -> bool {
    let mut letters = HashSet::new();

    for c in s.chars() {
        if c.is_alphabetic() {
            letters.insert(c);
        }
    }
    letters.len()==26
}
