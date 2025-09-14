

pub fn search(array: &[i32], key: i32) -> Option<usize> {
    let mut result : Option<usize> = None;
    for i in  0..array.len() {
        if key == array[i] {
            result = Some(i as usize);
        }
    }

    result
}