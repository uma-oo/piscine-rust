pub fn get_diamond(c: char) -> Vec<String> {
    let mut result = Vec::new();

    let len = (c as u8) - ('A' as u8) + 1;
    let c_index = (c as u8) - ('A' as u8);
    for i in 0..len {
        let mut line = String::new();
        line.push_str(&" ".repeat((c_index - i) as usize));
        line.push((('A' as u8) + (i as u8)) as char);

        if i > 0 {
            println!("char :{:?} len {} i: {}", (('A' as u8) + (i as u8)) as char, len, i);
            line.push_str(&" ".repeat((2 * i - 1) as usize));
            line.push((('A' as u8) + (i as u8)) as char);
            line.push_str(&" ".repeat((c_index - i) as usize));
        } else if i == 0 {
            line.push_str(&" ".repeat((c_index - i) as usize));
        }

        result.push(line);
    }

    let cloned = result.clone();
    println!("cloned: {:?}", cloned);
    let reversed: Vec<_> = cloned[0..(len - 1) as usize].into_iter().rev().collect();
    for item in reversed {
        result.push(item.to_string());
    }

    result
}
