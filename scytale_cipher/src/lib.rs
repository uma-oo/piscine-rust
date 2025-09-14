pub fn scytale_cipher(message: &str, i: usize) -> String {

    if message.trim()==""{
        return "".to_string()
    }
    // let mut container = Vec::new();
    let mut container = Vec::new();
    let mut j = 0;
    // divide hadshi hnaaaa
    while j < message.chars().count() {
        let mut new_vect: Vec<char> = Vec::new();
        while new_vect.len() <= i {
            if new_vect.len() == i {
                container.push(new_vect);
                break;
            }
            match message.chars().nth(j) {
                Some(value) => new_vect.push(value),
                None => new_vect.push(' '),
            };

            j += 1;
        }
    }

    let len = container[0].len();
    let mut result = String::new();
    let mut k = 0;
    for _i in 0..len {
        for vect in &container {
            result.push(vect[k]);
        }
        k += 1;
    }

    result.trim().to_string()
}
