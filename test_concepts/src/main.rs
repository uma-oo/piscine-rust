
 use std::rc::Rc;
fn main() {
    let a = Rc::new(6);

    let b = &a;
    let c = a.clone();
    let d = Rc::clone(&a);

    println!("strong count : {:?}", Rc::strong_count(&a));
    println!("weak count : {:?}", Rc::weak_count(&a));
}