






fn main(){
    let outer_ref : &u32 ;
    {
        let inner_ref: u32 = 5;
        outer_ref = &inner_ref;
        println!("{}", outer_ref)
    }

    println!("{}",outer_ref)
}