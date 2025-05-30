// A match expression with an arm that introduces a shadowed variable y

fn main() { 
    let x = Some(5); 
    let y = 10; 

    match x { 
        Some(50) => println!("Got 50"),
        Some(y) => println!("Matched, y = {y}"), 
        _ => println!("Default case, x = {:?}", x),
    }

    println!("at the end: x = {:?}, y = {y}", x);
}
