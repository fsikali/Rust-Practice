// Using a string slice after appending its contents to a String

fn main() { 
    let mut s1 = String::from("foo"); 
    let s2 = "bar";
    s1.push_str(s2); 
    println!("s2 is {s2}");
    println!("s1 is {s1}");
} 
