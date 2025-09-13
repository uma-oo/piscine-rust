use convert_case::{ Case, Casing };

pub fn expected_variable(compared: &str, expected: &str) -> Option<String> {
    if !compared.to_ascii_lowercase().is_case(Case::Snake) && !compared.is_case(Case::Camel) {
        return None;
    }
    let len = expected.chars().collect::<Vec<_>>().len() as f64;
    let distance = edit_distance(
        &compared.to_ascii_lowercase(),
        &expected.to_ascii_lowercase()
    ) as f64;

    let alikness = (len - distance) / len;
    if alikness > 0.5 {
        return Some(format!("{}%", (100.0 * alikness).round()));
    }
    None
}

pub fn edit_distance(source: &str, target: &str) -> usize {
    let a_chars: Vec<char> = source.chars().collect();
    let b_chars: Vec<char> = target.chars().collect();

    let len_a = a_chars.len();
    let len_b = b_chars.len();

    let mut dp = vec![vec![0; len_b + 1]; len_a + 1];

 
    for i in 0..=len_a {
        dp[i][0] = i;
    }
    for j in 0..=len_b {
        dp[0][j] = j;
    }

    for i in 1..=len_a {
        for j in 1..=len_b {
            if a_chars[i - 1] == b_chars[j - 1] {
                dp[i][j] = dp[i - 1][j - 1]; 
            } else {
                dp[i][j] = 1 + std::cmp::min(dp[i - 1][j], std::cmp::min(dp[i][j - 1],dp[i - 1][j - 1], ));
            }
        }
    }
    for _row in &dp {
        // println!("{row:?}")
    }

    dp[len_a][len_b]
}