use  std::collections::HashMap;

pub fn is_permutation(s1: &str, s2: &str) -> bool {
    let mut dict_words_s1 : HashMap<char , usize> = HashMap::new();
    let mut dict_words_s2 : HashMap<char , usize> = HashMap::new();
    for s in s1.chars() {
        let count = dict_words_s1.entry(s).or_insert(0);
        *count += 1;
    }
    for s in s2.chars(){
        let count = dict_words_s2.entry(s).or_insert(0);
        *count+=1
    }

    dict_words_s1 == dict_words_s2
}