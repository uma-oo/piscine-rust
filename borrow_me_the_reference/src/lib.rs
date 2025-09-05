


pub fn delete_and_backspace(s: &mut String) {
    let mut new_string = String::new();
     for (i , c) in s.clone().char_indices() {
        if c =='-' {
            new_string.push_str(&s[..i-1]);
            new_string.push_str(&s[i+1..]);
            *s = new_string.clone();
            new_string.clear();  
            delete_and_backspace(s);
        }
    }
    //println!("{}", new_string);
    //  println!("{}", new_string);
   
}

pub fn do_operations(v: &mut [String]) {
    println!("{:?}", v)
}