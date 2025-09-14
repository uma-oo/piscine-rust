pub fn get_diamond(c: char) -> Vec<String> {
    let mut result = Vec::new();
    let difference = ((c as u8) - ('A' as u8)) as usize;
    for i in 0..=difference{
        let mut line = String::new();
        for j in 0..2 * difference + 1 {
            if i% 2 == 0 {
                if j % 2 == 0 {
                    line.push(' ');
                } else {
                    line.push((('A' as u8) + (i as u8)) as char);
                }
            } else {
                if j % 2 == 0 {
                    line.push((('A' as u8) + (i as u8)) as char);
                    
                } else {
                    line.push(' ');
                }
            }
        }
        result.push(line);
    }

    result
}
