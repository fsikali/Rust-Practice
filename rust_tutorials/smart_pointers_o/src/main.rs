/* - Example: Demonstrating we're not allowed to have two lists using Box<T> that try to share ownership 
     of a third list
   - When we compile this code, we get this error:
*/
enum List { 
    Cons(i32, Box<List>), 
    Nil,
}

use crate::List::{Cons, Nil}; 

fn main() { 
    let a = Cons(5, Box::new(Cons(10, Box::new(Nil)))); 
    let b = Cons(3, Box::new(a)); 
    let c = Cons(4, Box::new(a));
}
