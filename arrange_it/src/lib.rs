pub fn arrange_phrase(phrase: &str) -> String {
    // Pre-allocate vector capacity (optimization)
    let mut splitted_phrase: Vec<String> = Vec::with_capacity(phrase.split(' ').count());
    splitted_phrase.extend(phrase.split(' ').map(|s| s.to_string()));

    // Sorting loop using swap - no clones for swap
    let len = splitted_phrase.len();
    let mut j = 0;
    while j < len {
        let element = &splitted_phrase[j];

        // Avoid collecting chars into a vector - iterate directly
        for c in element.chars() {
            if c.is_numeric() {
                let i = c as usize - '0' as usize;
                if i - 1 == j {
                    continue;
                }
                splitted_phrase.swap(j, i - 1);
                j = 0;
                break;
            }
        }
        j += 1;
    }

    for element in &mut splitted_phrase {
        let filtered: String = element.chars().filter(|c| !c.is_numeric()).collect();
        *element = filtered;
    }
    splitted_phrase.join(" ")
}
