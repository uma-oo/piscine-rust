use std::collections::HashMap;
// i mean exo smitu hashing walakin fiin n7tajuu l hashMap  fl9issa !!!!


pub fn mean(list: &[i32]) -> f64 {
    let res :i32 = list.iter().sum();

   res as f64 / list.len() as f64
}

pub fn median(list: &[i32]) -> i32 {
    let mut res = list.to_vec();
    res.sort();
    if res.len()%2 ==0 {
     return (res[res.len()/2]+res[res.len()/2-1])/2;
    }

    res[res.len()/2]
}

pub fn mode(list: &[i32]) -> i32 {
    let mut number_dict = HashMap::new();
    for item in list {
      let count = number_dict.entry(item).or_insert(0);
      *count +=1;
    }


    let vect  = Vec::from_iter(number_dict.values());
    let res = **vect.iter().max().unwrap();
    let mut mode : &i32= &0;
    for (key, &value) in &number_dict {
        if value == res {
            mode = &key;
            break;
        }
    }

   *mode
   
}