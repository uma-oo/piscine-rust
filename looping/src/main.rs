use std::io;

const RIDDLE : &str =  "I am the beginning of the end, and the end of time and space. I am essential to creation, and I surround every place. What am I?";
const ANSWER : &str = "The letter e";


fn main() -> io::Result<()> {
    let mut i = 0;

    loop {
        let mut input = String::new();
        println!("{}",RIDDLE );
        io::stdin().read_line(&mut input).expect("failed!");
        i+=1;
        if input.trim()== ANSWER {
            println!("Number of trials: {}", i);
            break;
        }
        
    }

    Ok(())
}
