// Example : A main function that calls the longest function to find the longer of two string slices

fn main() { 
    let string1 = String::from("abcd");
    let string2 = "xyz";

    let result = longest(string1.as_str(), string2);
    println!("The longest string is {}", result);
}
