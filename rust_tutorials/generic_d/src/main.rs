/*
   Generic Data Types 
   Example - The largest function using generic type parameters; this doesn't yet compile

   - This example - shows how we can call the function with either a slice of i32 values
                    char values
                  - shows the combined largest function definition using the generic data types
                    in its signature
*/ 

// We read this definition as: the function largest is generic over some type T
// This function has one parameter named list, which is a slice of values of type T
// The largest function will return a reference to a value of the same type T

fn largest<T>(list: &[T]) -> &T { 
    let mut largest = &list[0];

    for item in list { 
        if item > largest { 
            largest = item;
        }
    }
}

fn main() { 

    let number_list = vec![34, 50, 25, 100, 65]; 

    let result = largest(&number_list); 
    println!("The largest number is {}", result);

    let char_list = vec!['y', 'm', 'a', 'q']; 

    let result = largest(&char_list); 
    println!("The largest char is {}", result);
}

// This error states that the body of largest won't work for all possible
// types that T could be.
// Because we want to compare values of type T in the body, we can only use
// types whose values can be ordered.
// To enable comparisons, the standard library has the std::cmp::PartialOrd trait
// you can implement on types.
// By following the help text's suggestion, we restrict the types valid for T to only 
// those that implement PartialOrd and this example will compile, because the standard
// library implements PartialOrd on both i32 and char.

