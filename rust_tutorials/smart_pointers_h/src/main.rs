/*
- Implemetation of Deref to add to the definition of MyBox:
*/

// Example: Implementing Deref on MyBox<T>

use std::ops::Deref;

impl<T> Deref for MyBox<T> { 
    type Target = T; 

    fn deref(&self) -> &Self::Target { 
        &self.0
    }
}

struct MyBox<T>(T); 

impl<T> MyBox<T> { 
    fn new(x: T) -> MyBox<T> {
        MyBox(x)
    }
}

fn main() { 
    let x = 5; 
    let y = MyBox::new(x); 

    assert_eq!(5, x); 
    assert_eq!(5, *y);
}
