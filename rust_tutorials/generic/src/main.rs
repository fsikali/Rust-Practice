/** Removing Duplication by Extracting a Function 
 *  Generics allow us to replace specific types with a placeholder that represents
 *  multiple types to remove code duplication.
 *  
 *  How to remove duplication in a way that doesn't involve generic multiple values
 *  Apply same technique to extract a generic function 
 *  By looking at how to recognize duplicated code you can extract into a function, 
 *  you'll start to recognize duplicated code that can use generics 
 *  
 */ 

 // Example - Finding the largest number in a list of numbers 

fn main() { 
    let number_list = vec![34, 50, 68, 90, 100];

    let mut largest = &number_list[0];

    for number in &number_list { 
        if number > largest { 
            largest = number;
        }
    } 

    println!("The largest number is {}", largest); 
    assert_eq!(*largest, 100);
}


