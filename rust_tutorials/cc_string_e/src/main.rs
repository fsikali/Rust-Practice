// Appending a string slice to a String using the push_str method

fn main() { 
    let mut s = String::from("foo"); 
    s.push_str("bar");  

    println!("{s}");
}
