pub fn add_curry(nbr: i32) -> impl Fn(i32) -> i32 {
    move |x| x + nbr
}

// pub fn twice(f: impl Fn(i32)-> i32)-> impl Fn(i32) -> i32 {
//     move |x| f(f(x))
// }



pub fn twice <T>(f: T) -> impl Fn(i32)-> i32 where T:Fn(i32)-> i32{
    move |x| f(f(x))
}