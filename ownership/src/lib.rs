

pub fn first_subword(mut s: String) -> String {
    for (i , letter) in s.chars().enumerate(){
       if i!=0 && letter.is_uppercase() || letter == '_'  {
        return (&s[..i]).to_string();
       }

    }
    s
}