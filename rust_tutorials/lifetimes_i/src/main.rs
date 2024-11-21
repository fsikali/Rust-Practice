/*
   Example:
   - The following code will compile
   - Check explanation in lib.rs 

*/ 

fn main() {
    let string1 = String::from("abcd"); 
    let string2 = "efghijklmnopqrstvwxyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result); 
} 

fn longest<'a>(x: &'a str, y: &str) -> &'a str {
    x
}

