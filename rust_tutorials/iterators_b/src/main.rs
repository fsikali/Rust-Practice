// Using an iterator in a for loop 

fn main() { 
    let v1 = vec![1, 2, 3]; 

    let v1_iter = v1.iter(); 

    for val in v1_iter { 
        println!("Got: {}", val);
    }
}
