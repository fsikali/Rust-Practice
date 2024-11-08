/* 
   Using Trait Bounds to Conditionally Implement Methods 
   - By using a trait bound with an impl block that uses generic type parameters, we
     implement methods conditionally for types that implement the specified traits.
   
   Example: 
   - Conditionally implementing methods on a generic type depending on trait bounds
*/ 

use std::fmt::Display; 

struct Pair<T> { 
    x: T,
    y: T,
} 

impl<T> Pair<T> { 
    fn new(x: T, y: T) -> Self {
        Self { x, y}
    }
} 

impl<T: Display + PartialOrd> Pair<T> { 
    fn cmp_display(&self) { 
        if.self.x >= self.y { 
            println!("The largest member is = {}", self.x);
        } else { 
            println!("The largest member is y = {}", self.y);
        }
    }
}

