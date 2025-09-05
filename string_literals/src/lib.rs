pub fn is_empty(v: &str) -> bool {
    v.len()==0
}

pub fn is_ascii(v: &str) -> bool {
    for c in v.chars() {
        if !c.is_ascii(){
            return  false;
        }
    }

    return true;

}


//  first way ( using a for loop)
//  second way using the regex (lmhm test u saf)
pub fn contains(v: &str, pat: &str) -> bool {
    
    for (i , _) in v.chars().enumerate() {
        if pat.len()+i < v.len() {
            let sliced=&v[i..pat.len()+i];
            if sliced == pat {
                return true;
            } 
        }
    }


    return false;

}

pub fn split_at(v: &str, index: usize) -> (&str, &str) {
     (&v[..index], &v[index..])
}

pub fn find(v: &str, pat: char) -> usize {
     for (i, c) in v.chars().enumerate() {
        if c == pat {
           return i;
        } 
    }
    0
}