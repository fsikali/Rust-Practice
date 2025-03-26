// A function with parameters that destructure a tuple

fn print_coordinates(&(x, y): &(i32, i32)) { 
    println!("Current location: ({}, {})", x, y);
}

fn main() { 
    let point = (3, 5); 
    print_coordinates(&point);
}
