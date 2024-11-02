/*
   Generic Data Types In Method Definitions

   Example - A method that uses generic types different from its struct's definition

*/ 

struct Point<X1, Y1> { 
    x: X1, 
    y: Y1,
} 

impl<X1, Y1> Point<X1, Y1> { 
    fn mixup<X2, Y2>(self, other: Point<X2, Y2>) -> Point<X1, Y2> { 
        Point { 
            x: self.x, 
            y: other.y,
        }
    }
} 

fn main() { 
    let p1 = Point { x: 5, y: 10.4};
    let p2 = Point { x: "Hello", y: 'c'}; 

    let p3 = p1.mixup(p2);

    println!("p3.x = {}, p3.y = {}", p3.x, p3.y);
}

/* 
   In main, we've defined a Point that has an i32 for x (with value 5) and an f64 
   for y (with value 10.4). 

   The p2 variable is a Point struct that has a string slice for (with value "Hello") 
   and a char for y (with value c). 

   Calling mixup on p1 with the argument p2 gives us p3, which will have i32 for x,
   because x came from p1.

   The p3 variable will have a char for y, because y came from p2.The println! macro
   call will print p3.x = 5, p3.y = c. 

   N/B - The purpose of this example is to demonstrete a situation in which some generic
         parameters are declared with impl and some are declared with the method definition.

         Here, the generic parameters X1 and Y1 are declared after impl because they go with
         the struct definition.

         The generic parameters X2 and Y2 are declared after fn mixup, because they're only
         relevant to the method
*/