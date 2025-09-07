use  std::collections::HashMap;

pub fn word_frequency_counter<'a>(words: &[&'a str]) -> HashMap<&'a str, usize> {
    let mut words_dict = HashMap::new();
    for word in words {
       match words_dict.get(word) {
        Some(&count)=> words_dict.insert(*word, count+1),
        None => words_dict.insert(*word, 1),
       };
    } 
   words_dict
}

pub fn nb_distinct_words(frequency_count: &HashMap<&str, usize>) -> usize {
   frequency_count.len()
}