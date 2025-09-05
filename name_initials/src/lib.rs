

pub fn initials(names: Vec<&str>)-> Vec<String>{
    let  mut res  = Vec::with_capacity(names.len());
    for (i , &elem) in names.iter().enumerate() {
      let mut inner_res = String::new();
       let elements = elem.split(" ");
       for el in elements {
        inner_res.push_str(&el[..1]);
        inner_res.push_str(". ");
       }
    
    res.push(inner_res.trim().to_string());
    
    }

  return res
    
}





