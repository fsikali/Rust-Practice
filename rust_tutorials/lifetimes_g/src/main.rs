/*
   How the lifetime annotations restrict the longest function by passing in references that have
   different concrete lifetimes
   
   Example: 
   - Using the longest function with references to String values that have different concrete lifetimes

*/ 

fn main() {
    let string1 = String::from("long string is long");

    { 
        let string2 = String::from("xyz");
        let result = longest(string1.as_str(), string2.as_str());
        println!("The longest string is {}", result);
    }
} 

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
} 
