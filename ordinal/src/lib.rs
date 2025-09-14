pub fn num_to_ordinal(x: u32) -> String {
    let ordinal = match true {
        _ if x % 100 != 11 && x % 10 == 1 => "st".to_string(),
        _ if x % 100 != 12 && x % 10 == 2 => "nd".to_string(),
        _ if x % 100 != 13 && x % 10 == 3 => "rd".to_string(),
        _ => "th".to_string(),
    };

    x.to_string() + &ordinal
}
