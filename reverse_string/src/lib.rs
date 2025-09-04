



pub fn rev_str(input: &str) -> String {
    let mut  my_vec: Vec<char> = input.chars().collect();
    my_vec.reverse();
    let mut result =String::new();
    for c in &my_vec {
        result.push(*c)
    }


 

    return result;

}