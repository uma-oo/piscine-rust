


pub fn delete_and_backspace(s: &mut String) {
  let mut result : Vec<char> = s.chars().collect();
  let mut i = 0;
  while i<result.len() {
    if result[i]=='-' {
        if i==0 {
            result.remove(i);
        } else {
            result.remove(i);
            result.remove(i-1);

        }
        
        i=0;
    } else if result[i]=='+' &&  i+1 < result.len() &&  result[i+1]!='+'{
            result.remove(i+1);
            result.remove(i);
            i=0;
     } else if result[i]=='+' && i==result.len() -1{
            result.remove(i);
            i=0;
         }   else {
        i+=1;
    }

   
  }

  *s = result.into_iter().collect();
   
}

pub fn do_operations(v: &mut [String]) {
    for element in v {
    let chars_element : Vec<char>  = element.chars().collect();
    let mut i = 0;
    for &c in chars_element.iter() {
        if c == '-'  ||  c =='+' {
          let parsed_left : String=  chars_element[0..i].iter().collect();
          let  parsed_left : i32= parsed_left.parse().unwrap();
          let parsed_right : String=  chars_element[i+1..].iter().collect();
          let  parsed_right : i32= parsed_right.parse().unwrap();
         if c == '+' {
            *element = (parsed_left+parsed_right).to_string();
         } else {
            *element = (parsed_left-parsed_right).to_string();
         }

        }
        i+=1;
    }
    }

    
    }
