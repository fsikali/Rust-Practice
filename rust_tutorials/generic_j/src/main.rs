/* 
    Generic Data Types In Method Definitions

    We can also specify constraints on generic types when defining methods
    on the type.
    We could, for example, implement methods only on Point<f32> instances rather
    than on Point<T> instances with any generic type.

    Example - An impl block that only applies to a struct with a particular concreta
              type for the generic type parameter T.
              We use the concrete type f32,meaning we don't declare any types after impl
*/

struct Point<T> { 
    x: T, 
    y: T,
} 

impl<T> Point<T> { 
    fn x(&self) -> &T {
        &self.x
    }
} 

impl Point<f32> { 
    fn dustance_from_origin(&self) -> f32 { 
        (self.x.powi(2) + self.y.powi(2)).sqrt()
    }
} 

fn main() { 
    let p = Point { x: 5, y: 10 }; 

    println!("p.x = {}", p.x() );
}

/* 
   This code means the type Point<f32> will have a distance_from_origin method;
   other instances of Point<T> where T is not of type f32 will not have this method defined

   The method measures how far our point is from the point at coordinates(0.0, 0.0) and uses
   mathematical operations that are available only for floating point types

   N/B: Generic type parameters in a struct definition aren't always the same as those you use
   in that same struct's method signatures
*/
