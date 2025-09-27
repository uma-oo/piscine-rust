pub fn get_products(arr: Vec<usize>) -> Vec<usize> {
    let mut vector = Vec::new();
    for ele in &arr {
        let data: Vec<_> = arr
            .iter()
            .filter(|&a| a != ele)
            .collect();

        let mut product = 1;
        for item in data {
            product *= item;
        }
       
        vector.push(product);
    }
    vector
}
