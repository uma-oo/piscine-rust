pub fn capitalize_first(input: &str) -> String {
    if input.chars().collect::<Vec<_>>().len()==  0 {
       return "".to_string()
    }
let first = input.chars().nth(0).unwrap();
   first.to_string().to_uppercase() + &input[1..]
}

pub fn title_case(input: &str) -> String {
  let mut result : Vec<String> = Vec:: new();
  let mut is_first = true ;
   for c in input.chars(){
     if c.is_whitespace(){
        result.push(c.to_string());
        is_first = true;
     } else if c.is_lowercase() && is_first {
      result.push(c.to_string().to_uppercase());
      is_first = false;
     }  else {
      result.push(c.to_string());
     }
   }
   result.into_iter().collect()
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