use std::io;
use rand::prelude::*;



fn main(){
    let guessed = rand::thread_rng().gen_range(0..=10);
    loop {
        let mut number = String::new();
        match io::stdin().read_line(& mut number) {
            Ok(_)=> {
            let  number : i32 = match number.trim().parse() {
            Ok(num)=> num,
            Err(err)=> {
                println!("there was an error while trying to convert the number!! {}", err);
                continue
            }
        };

        if number == guessed {
            println!("Hey u did well");
            break;
        } else {
            println!("Hey u did not do well");

        }
            },
            Err(e) => {
                println!("there was an error while reading the file! {:?}", e);
                continue
            }
        };
       
    }
    
}