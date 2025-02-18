// Iterating over mutable references to elements in a vector

fn main() { 
    let mut v = vec![100, 32, 57]; 
    
    for i in &mut v { 
        *i += 50;  

        println!("{i}");
    } 
}
