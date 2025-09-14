pub fn talking(text: &str) -> &str {
    let len = (text.len() as i32) - 1;
    let result = match true {
        _ if text.trim() == "" => "Just say something!",
        _ if
            is_uppercase_sentence(text) &&
            text
                .chars()
                .nth(len as usize)
                .unwrap() != '?'
        => "There is no need to yell, calm down!",
        _ if
            is_uppercase_sentence(text) &&
            text
                .chars()
                .nth(len as usize)
                .unwrap() == '?'
        => "Quiet, I am thinking!",
        _ if
            !is_uppercase_sentence(text) &&
            text
                .chars()
                .nth(len as usize)
                .unwrap() == '?'
        => "Sure.",
        _ => "Interesting",
    };

    result
}

fn is_uppercase_sentence(text: &str) -> bool {
    text[0..text.len() - 1]
        .chars()
        .filter(|c| c.is_ascii_alphabetic())
        .count() >= 1 &&
        text
            .chars()
            .filter(|c| c.is_ascii_alphabetic())
            .all(|c| c.is_uppercase())
}
