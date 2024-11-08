/* 
   Trait Bound Syntax 
   - The impl Trait syntax works for straightfoward cases but is actually syntax for a longer
     form known as a trait bound. 
     Example;
*/ 

pub fn notify<T: Summary>(item: &T) { 
    println!("Breaking news! {}", item.summarize());
}  

/*
   -This longer form is equivalent to the example: trait_d in the previous section but more
    verbose. 
   -We place trait bounds with the declaration of the generic type parameter after a colon
    and inside angle brackets.
   -The impl Trait syntax is convenient and makes for more concise code in simple cases, while
    the fuller trait bound syntax can express more complexity in other cases.
   -We can have two parameter that implement Summary. Doing so with the impl Trait syntax
    looks like this;
*/ 

pub fn notify(item: &impl Summary, item2: &impl Summary) 

/* 
   Using impl Trait is appropriate if we want this function to allow item1 and item2 to have 
   different types(as long as both types implement Summary).If we want to force both parameters
   to have the same type, however, we must use a trait bound, like this:
*/ 

pub fn notify<T: Summary>(item1: &T, item2: &T) {}

/* 
   The generic type T specified as the type of the item1 and item2 parameters constrains the
   function such that the concrete type of the value passed as an argument for item1 and item2
   must be the same
*/ 

/*
   Specifying Multiple Trait Bounds with the + Syntax
    - We can also specify more than one trait bound.
    - Say we wanted notify to use display formatting as well as summarize on item:
      we specify in the notify definition that item must implement both Display and Summary
      We can do so using the + syntax:
*/

pub fn notify(item: &(impl Summary + Display)) {}

// The + syntax is also valid with trait bounds onn generic types: 

pub fn notify<T: Summary + Display>(item: &T) {}

/* 
   With the two trait bounds specified, the body of notify can call summarize and
   use {} to format item.
*/

/*
    Clearer Trait Bounds with where Clauses
    - Using too many trait bounds has its downsides. Each generic has its own trait
      bounds, so functions with multiple generic type parameters can contain lots of 
      trait bound information between the function's name and its parameter list, 
      making the function signature hard to read.
    - For this reason, Rust has alternative syntax for specifying trait bounds inside
      a where clause after the function signature. So instead of writing this:
*/ 

fn some_function<T: Display + Clone, U: Clone + Debug>(t: &T, u: &U) -> i32 {}

// we can use a where clause, like this: 

fn some_function<T, U>(t: &T, u: &U) -> i32
where 
    T: Display + Clone, 
    U: Clone + Debug, 
{} 

/* This function's signature is less cluttered: the function name, parameter list,
   and return types are close together, similar to a function without lots of
   trait bounds.
*/