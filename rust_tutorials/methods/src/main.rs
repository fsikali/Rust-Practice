use methods::Rectangle;

fn main() { 

    let rect1 = Rectangle { 
        width: 30,
        height: 50,
    };

    println!("The area of the rectangle is {} square pixels.", rect1.area() );
}
