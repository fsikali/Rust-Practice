use methods_b::Rectangle; 

fn main() { 
    let rect = Rectangle { 
        width: 10,
        height: 25,
    }; 

    println!("The perimeter is {} ", rect.perimeter());
}
