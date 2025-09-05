

pub fn to_url(s: &str) -> String {
  let mut  result = String::new();
  for ele in s.chars() {
     if ele == ' ' {
        result.push_str("%20");
     }
    result.push(ele);
  }
  result
}