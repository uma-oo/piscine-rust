use closures::*;

fn main() {
    println!("Hello, world!");
    let v1 = first_fifty_even_square();

    println!("All elements in {:?}, len = {}", v1, v1.len());
}

// fn main() {
//     let store = Inventory {
//         shirts: vec![ShirtColor::Blue, ShirtColor::Red, ShirtColor::Blue],
//     };

//     let user_pref1 = Some(ShirtColor::Red);
//     let giveaway1 = store.giveaway(user_pref1);
//     println!("The user with preference {:?} gets {:?}", user_pref1, giveaway1);

//     let user_pref2 = None;
//     let giveaway2 = store.giveaway(user_pref2);
//     println!("The user with preference {:?} gets {:?}", user_pref2, giveaway2);

//     let example_closure = |x| x;

//     let s = example_closure(String::from("hello"));
//     let n = example_closure(5);
//     println!("{:?}", n);
//     println!("{:?}", s)
// }
