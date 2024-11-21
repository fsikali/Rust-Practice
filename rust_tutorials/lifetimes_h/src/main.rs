/*
    Example: 
    - Attempting to use result after string2 has gone out of scope
    - When we try to compile this code we get an error
    - Explanation is lib.rs
*/  

fn main() { 
    let string1 = String::from("long string is long"); 
    let result;

    { 
        let string2 = String::from("xyz"); 
        result = longest(string1.as_str(), string2.as_str());
    }
    println!("The longest string is {}", result);
} 

fn longest<'a>(x: &'a str, y: &'a str) -> &'a str {
    if x.len() > y.len() {
        x
    } else {
        y
    }
}
