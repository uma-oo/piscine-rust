use diamond_creation::*;

fn main() {
    // println!("{:?}", get_diamond('A'));
    println!("{:?}", get_diamond('F'));

    for line in get_diamond('F') {
        println!("{}", line);
    }
}

