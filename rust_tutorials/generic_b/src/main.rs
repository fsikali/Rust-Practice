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

    let result = largest(&number_list);
    println!("The largest number is {}", result); 
    assert_eq!(*result, 6000);
} 

/* The largest number has a parameter called list, which represents any concrete slice
of i32 values we might pass into the function.
As a result, when we call the function, the code runs on the specific values that 
we pass in.

Steps used to change the code from listing
  1.) Identify duplicate code
  2.) Extract the duplicate code into the body of the function and specify the inputs and
      return values of that code in the function signature
  3.) Update the two instances of duplicated code to call the function instead 
  
 */ 

//fn larget(list: &[i32]) -> &i32