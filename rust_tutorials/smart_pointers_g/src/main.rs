/*
- This code won't compile because Rust doesn't know how to dereference MyBox
*/

// Example: Attempting to use MyBox<T> in the same way we used reference and Box<T>

fn main() { 
    let x = 5; 
    let y = MyBox::new(x); 

    assert_eq!(5, x); 
    assert_eq!(5, *y);
}
