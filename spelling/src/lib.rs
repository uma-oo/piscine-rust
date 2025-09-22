pub fn spell(n: u64) -> String {
    let mut result: Vec<String> = Vec::new();
    let mut number = n;
    if n == 0 {
        result.push("zero".to_string());
        let res = &result[0];
        return res.to_string();
    }

    if n == 1_000_000 {
        result.push("one million".to_string());
        let res = &result[0];
        return res.to_string();
    }

    if n >= 1000 {
        let thousands = n / 1000;
        println!("tousands : {}", thousands);
        number = n % 1000;
        result.push(format!("{} thousand", spell_handreds(thousands)));
    }

    if number > 0 || result.is_empty() {
        result.push(spell_handreds(number));
    }

    result.join(" ")
}

pub fn spell_1_to_19(n: u64) -> String {
    match n {
        1 => "one".to_string(),
        2 => "two".to_string(),
        3 => "three".to_string(),
        4 => "four".to_string(),
        5 => "five".to_string(),
        6 => "six".to_string(),
        7 => "seven".to_string(),
        8 => "eight".to_string(),
        9 => "nine".to_string(),
        10 => "ten".to_string(),
        11 => "eleven".to_string(),
        12 => "twelve".to_string(),
        13 => "thirteen".to_string(),
        14 => "fourteen".to_string(),
        15 => "fifteen".to_string(),
        16 => "sixteen".to_string(),
        17 => "seventeen".to_string(),
        18 => "eighteen".to_string(),
        19 => "nineteen".to_string(),
        _ => "".to_string(),
    }
}

pub fn spell_tens(n: u64) -> String {
    match n {
        2 => "twenty".to_string(),
        3 => "thirty".to_string(),
        4 => "forty".to_string(),
        5 => "fifty".to_string(),
        6 => "sixty".to_string(),
        7 => "seventy".to_string(),
        8 => "eighty".to_string(),
        9 => "ninety".to_string(),
        _ => "".to_string(),
    }
}

pub fn spell_handreds(n: u64) -> String {
    let hundreds = n / 100;
    let mut result = String::new();
    if hundreds > 0 {
        let remainder = n % 100;
        result.push_str(&spell_1_to_19(hundreds));
        result.push_str(" hundred");
        if remainder > 0 {
            result.push(' ');
        }
        if remainder > 0 {
            if remainder < 20 {
                result.push_str(&spell_1_to_19(remainder));
            } else {
                let tens = remainder / 10;
                if tens > 0 {
                    let unities = n % 10;
                    result.push_str(&spell_tens(tens));
                    if unities > 0 {
                        result.push('-');
                        result.push_str(&spell_1_to_19(n%10));
                    }
                }
            }
        }
    } else {
        if n < 20 {
            result.push_str(&spell_1_to_19(n));
        } else {
            let tens = n / 10;
            if tens > 0 {
                let unities = n % 10;
                result.push_str(&spell_tens(tens));
                if unities > 0 {
                    result.push('-');
                    result.push_str(&spell_1_to_19(unities));
                }
            }
        }
    }

    result
}
