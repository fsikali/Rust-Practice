// The Iterator Trait and the next Method
// All iterators implement a trait named Iterator that is defined in the standard library
// The definition of the trait looks like this:

#![allow(unused)]
fn main() { 
    pub trait Iterator { 
        type Item; 

        fn next(&mut self) -> Option<Self::Items>;

        // methods with default implementations elided
    }
}
