use methods_a::Rectangle;

fn main() {  
    let rect1 = Rectangle { 
        width: 30,
        height: 50,
    }; 

    if rect1.width() { 
        println!("The rectangle has a nonzero width; it is {}", rect1.width);
    }
} 




