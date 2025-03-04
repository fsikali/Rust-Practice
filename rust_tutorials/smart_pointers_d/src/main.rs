/*
- Creating a reference to an i32 value and then use the derefence operator to follow the reference to the value: 
*/

// Example: Using the dereference operator to follow a reference to an i32 value

fn main() { 
    let x = 5; 
    let y = &x;

    assert_eq!(5, x);
    assert_eq!(5, *y);
}
