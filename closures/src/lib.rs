pub fn first_fifty_even_square() -> Vec<i32> {
    let mut vector = Vec::new();
    let mut counter = 0;
    let mut x = 0;
    while counter < 50 {
        let mut adding = | x: &mut i32| {
            counter += 1;
            *x += 2;
            x.pow(2)
        };
        vector.push(adding(&mut x));
    }

    vector
}
