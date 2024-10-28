/**
 * Removing Duplication by Extracting a Function
 */ 


 /* Example - Abstracted code to find the largest number in two lists

 We extract the code that finds the largest number into a function named largest. 
 The we call the function to find the largest number in the two lists 
 We could also use the function on any other list of i32 values we might have in the future

*/ 

fn largest(list: &[i32]) -> &i32  { 
    let mut largest = &list[0]; 

    for item in list { 
        if item > largest { 
            largest = item;
        }
    } 

    largest
}


fn main() { 
    let number_list = vec![34, 50, 24, 100, 65]; 

    let result = largest(&number_list); 
    println!("The largest number is {}", result); 
    assert_eq!(*result, 100); 

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8]; 
    println!("The largest number is {}", result); 
    assert_eq(*result, 6000);
}
