/*
- How a reference cycle might happen and how to prevent it, starting with the definition of the 
  List enum and a tail method
*/

// Example: A cons list definition that holds a RefCell<T> so we can modify what a Cona variant is referring to

use crate::List::{Cons, Nil};
use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug)]
enum List {
    Cons(i32, RefCell<Rc<List>>),
    Nil,
}

impl List {
    fn tail(&self) -> Option<&RefCell<Rc<List>>> {
        match self {
            Cons(_, item) => Some(item),
            Nil => None,
        }
    }
}

fn main() {}
