struct Rectangle { 
    length: i32, 
    width: i32,
} 

fn main() { 
    let rect = Rectangle { 
        length: 10, 
        width: 5,
    }; 

    let area =  rect.length * rect.width; 

    println!("This is the area of the Rectangle: {}", area);
} 