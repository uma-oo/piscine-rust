pub fn pig_latin(text: &str) -> String {
    let mut result = String::new();

    let first = text.chars().nth(0).unwrap();
    let second = text.chars().nth(1).unwrap();
    let third = text.chars().nth(2).unwrap();

    if !is_vowel(first) {
        if (second == 'q' || second == 'q') && (third == 'u' || third == 'U') {
            result.push(first);
            result.push_str("qu");
            result.push_str(&text[3..]);
            result.push_str("ay");
            return result;
        }
    }

    let index = find_first_vowel(text);
    if index != -1 {
        let i  = index as usize;
        result.push_str(&text[i..]);
        result.push_str(&text[..i]);
        result.push_str("ay");
    } else {
        result.push_str(&text[0..]);
    }

    result
}

// aeiou

fn is_vowel(c: char) -> bool {
    c == 'a' ||
        c == 'i' ||
        c == 'e' ||
        c == 'o' ||
        c == 'u' ||
        c == 'A' ||
        c == 'I' ||
        c == 'E' ||
        c == 'O' ||
        c == 'U'
}

fn find_first_vowel(text: &str) -> i32 {
    for i in 0..text.chars().count() {
        if is_vowel(text.chars().nth(i).unwrap()) {
            return i as i32;
        }
    }
    -1
}
