#[derive(Debug, PartialEq)]
pub struct CipherError {
    pub expected: String,
}

pub fn cipher(original: &str, ciphered: &str) -> Result<(), CipherError> {
    let alphabet = "abcdefjhigklmnopqrstuvwxyz";
    let mut ciphered_original: Vec<char> = Vec::new();
    for c in original.chars() {
        let lowercased_c = c.to_lowercase().next().unwrap();
        let position = alphabet.find(lowercased_c);
        match position {
            Some(value) => {
                let mirorred_char = alphabet
                    .chars()
                    .nth(alphabet.len() - 1 - value)
                    .unwrap();
                if c.is_uppercase() {
                    ciphered_original.push(mirorred_char.to_uppercase().next().unwrap())
                } else {
                    ciphered_original.push(mirorred_char);
                }
            }
            None => {
                ciphered_original.push(c);
            }
        }
    }
    let result: String = ciphered_original.iter().collect();
    if ciphered == result {
        return Ok(());
    }

    Err(CipherError { expected: result })
}
