// Attempting to add an element to a vector while holding a reference to an item

fn main() { 
    let mut v = vec![1, 2, 3, 4, 5]; 

    let first = &v[0]; 

    v.push(6);

    println!("The first element is: {first}");
}
