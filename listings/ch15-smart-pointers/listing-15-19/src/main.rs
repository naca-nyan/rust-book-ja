enum List {
    Cons(i32, Rc<List>),
    Nil,
}

use crate::List::{Cons, Nil};
use std::rc::Rc;

// ANCHOR: here
fn main() {
    let a = Rc::new(Cons(5, Rc::new(Cons(10, Rc::new(Nil)))));
    //       "a作成後のカウント = {}"
    println!("count after creating a = {}", Rc::strong_count(&a));
    let b = Cons(3, Rc::clone(&a));
    //       "b作成後のカウント = {}"
    println!("count after creating b = {}", Rc::strong_count(&a));
    {
        let c = Cons(4, Rc::clone(&a));
        //       "c作成後のカウント = {}"
        println!("count after creating c = {}", Rc::strong_count(&a));
    }
    //       "cがスコープを抜けた後のカウント = {}"
    println!("count after c goes out of scope = {}", Rc::strong_count(&a));
}
// ANCHOR_END: here
