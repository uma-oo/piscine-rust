pub fn parse_into_boxed(s: String) -> Vec<Box<u32>> {
    let mut result: Vec<Box<f32>> = Vec::new();
    let splitted: Vec<_> = s.split(" ").collect();
    for val in splitted {
        if val.ends_with("k") {
            result.push(Box::new(val[0..val.len() - 1].parse::<f32>().unwrap()));
        }
    }

    let mut result_int: Vec<Box<u32>> = Vec::new();

    for box_float in result {
        result_int.push(Box::new((*box_float * 1000.0) as u32));
    }

    result_int
}

pub fn into_unboxed(a: Vec<Box<u32>>) -> Vec<u32> {
    let mut result = Vec::new();
    for element in a {
        result.push(*element);
    }
    result
}
