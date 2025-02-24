// Attempting to call a closure whose types are inferred with two different types

fn main() { 
    let example_closure = |x| x;

    let s = example_closure(String::from("hello")); 
    let n = example_closure(5); 
}
