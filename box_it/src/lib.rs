pub fn parse_into_boxed(s: String) -> Vec<Box<u32>> {
    let mut result: Vec<Box<u32>> = Vec::new();
    let splitted: Vec<_> = s.split(" ").collect();
    for val in splitted {
        if val.ends_with("k") {
            let value: u32 = (val[0..val.len() - 1].parse::<f32>().unwrap() * 1000.0)  as u32;
            result.push(Box::new(value));
        } else {
            let value: u32 = val.parse().unwrap();
            result.push(Box::new(value));
        }
    }

    result
}

pub fn into_unboxed(a: Vec<Box<u32>>) -> Vec<u32> {
    let mut result = Vec::new();
    for element in a {
        result.push(*element);
    }
    result
}
