pub fn capitalize_first(input: &str) -> String {
    if input.chars().collect::<Vec<_>>().len()==  0 {
       return "".to_string()
    }
let first = input.chars().nth(0).unwrap();
   first.to_string().to_uppercase() + &input[1..]
}

pub fn title_case(input: &str) -> String {
    let fields = input.split_whitespace().collect::<Vec<&str>>();
    let res  =  input.chars().map(|s| {
      if s.is_whitespace() {
        s.to_string()
      } else if s.is_lowercase() {
        s.to_string().to_uppercase()
      } else {
        s.to_string().to_lowercase()
      }
    }).collect::<Vec<_>>().join("");
   
   

  res
}

pub fn change_case(input: &str) -> String {
    let  mut res = Vec::new();
    for c in input.chars() {
      if c.is_lowercase(){
        res.push(c.to_string().to_uppercase())
      } else {
        res.push(c.to_string().to_lowercase())
      }
    }
    res.join("")
    
}