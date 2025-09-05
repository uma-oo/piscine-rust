pub fn nbr_function(c: i32) -> (i32, f64, f64) {
    return (c ,(c as f64).exp() as f64, (c.abs() as f64).ln() as f64);
}

pub fn str_function(a: String) -> (String, String) {
    let elements = a.split(" ");
    let mut res = String::new();
    for element in elements{
        let casted_element = element.parse::<f64>().unwrap();
        let exp_c = casted_element.exp();
        res.push_str(&exp_c.to_string());
        res.push_str(" ");
    }
    return (a,res.trim().to_string())
}

pub fn vec_function(b: Vec<i32>) -> (Vec<i32> , Vec<f64>) {
     let mut res : Vec<f64> = Vec::new();
     for element in b.iter() {
        let casted_element = element.abs() as f64;
        res.push(casted_element.ln())
     }

    return (b, res);
}