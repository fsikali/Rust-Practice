/*
- A consequence of the borrowing rules is that when you have an immutable value, you can't borrow it mutably.
- This code won't compile:
*/

fn main() { 
    let x = 5; 
    let y = &mut x;
}
