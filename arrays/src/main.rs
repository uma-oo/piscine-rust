use arrays::*;

fn main() {
    let a = (1..=10).collect::<Vec<_>>();
    println!("The sum of the elements in {:?} is {}", a, sum(&a));
    println!(
        "Array of {} elements filled with 10 = {:?}",
        thirtytwo_tens().len(),
        thirtytwo_tens()
    );
}