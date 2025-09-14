pub fn number_logic(num: u32) -> bool {
    let vec_nums = itoa(num);

    match true {
        _ if calc(vec_nums) == num => true,
        _ => false,
    }
}

fn itoa(num: u32) -> Vec<u8> {
    let mut n = num;
    let mut result: Vec<u8> = Vec::new();
    if n == 0 {
        result.push(0);
        return result;
    }
    while n > 0 {
        result.push((n % 10) as u8);
        n = n / 10;
    }

    result.into_iter().rev().collect()
}

fn calc(number: Vec<u8>) -> u32 {
    let mut result = 0;
    for n in &number {
        result += (*n as u32).pow(number.len() as u32);
    }
    result as u32
}
