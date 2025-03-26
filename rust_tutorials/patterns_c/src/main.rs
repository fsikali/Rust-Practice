// Using a pattern in a for loop to destructure a tuple

fn main() { 
    let v = vec!['a', 'b', 'c']; 

    for (index, value) in v.iter().enumerate() { 
        println!("{} is at index {}", value, index);
    }
}
