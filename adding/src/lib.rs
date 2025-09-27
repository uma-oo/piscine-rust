pub fn add_curry(nbr: i32) -> impl Fn(i32) -> i32 {
    move |x| x + nbr
}
