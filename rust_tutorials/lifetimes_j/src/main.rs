/*
   Example:
   - Attempted implemetation of the longest function that won't compile
   - Explanation in lib.rs

*/ 

fn main() {
    let string1 = String::from("abcd");
    let string2 = "xyz"; 

    let result = longest(string.as_str(), string2);
    println!("The longest string is {}", result);
}

fn longest<'a>(x: &str, y:&str) -> &'a str {
    let result = String::from("really long string");
    result.as_str()
}
 
