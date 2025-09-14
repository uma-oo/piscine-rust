pub fn rotate(input: &str, key: i8) -> String {
    let new_key = match true {
        _ if key >= 0 => key,
        _ => key + 26,
    };

    let mut result = String::new();
    for c in input.chars() {
        match true {
            _ if c.is_lowercase() => {
                let new_char = (((c as u8) - ('a' as u8) + (new_key as u8)) % 26) + ('a' as u8);
                result.push(new_char as char);
            }
            _ if c.is_uppercase() => {
                let new_char = (((c as u8) - ('A' as u8) + (new_key as u8)) % 26) + ('A' as u8);
                result.push(new_char as char);
            }
            _ => {
                result.push(c);
            }
        }
    }

    result
}
