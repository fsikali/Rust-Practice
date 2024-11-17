/* 
   Using Trait Bounds to Conditionally Implement Methods
   - We can also conditionally implement a trait for any type that implements another
     trait.
   - Implementations of a trait on any type that satifies the trait bounds are called
     blanket implementations and are extensively used in the Rust standard library.
   Example: 
   - The standard library implements the ToString trait on any type that implements 
     the Display trait. The impl block in the standard library looks similar to this
     code:
*/ 

impl<T: Display> ToString for T { 

} 

/* 
   Because the standard library has this blanket implementation, we can call the
   to_string method defined by the ToString trait on any type that implements the 
   Display trait.
   Example: 
   - The standard library implements the ToString trait on any type that implements
     Display trait. The impl block in the standard library looks similar to this code:
*/ 

impl<T: Display> ToString for T { 

} 

/* 
   Because the standard library has this blanket implementation, we can call the to_string
   method defined by the ToString trait on any type that implements the Display trait

   Example: We can turn integers into their corresponding String values like this because
            integers implement Display
*/ 

let s = 3.to_string(); 

/* Blanket implementations appear in the documentation for the trait in the "implementors"section 
   
   - Trait and trait bounds let us write code that uses generic type parametes to reduce
     duplication but also specify to the compiler that we want the generic type to have particular
     behavior.
   - The compiler can then use the trait bound information to check that all the conrete types
     used with our code provide the correct behavior.
   - In dynamically typed languages, we would get an error at runtime if we called a method on 
     a type which didn't define the method.
   - But Eust moves these errors to compile time so we're forced to fix the problems before our
     code is even able to run
   - Additionally, we don't have to write code that checks for behavior at runtime because we've 
     already checked at compile time. Doing so improves performance without having to give up the
     flexibility of generics.
*/
 
