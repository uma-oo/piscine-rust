use diamond_creation::*;

fn main() {
    // println!("{:?}", get_diamond('A'));
    println!("{:?}", get_diamond('A'));
    for line in get_diamond('B') {
        println!("{}", line);
    }
}

