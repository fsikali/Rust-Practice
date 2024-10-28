/** 
 * Removing Duplication by Extracting a Function 
 * 
 */ 

 // Example - Code to find the largest number in two lists of numbers 

 fn main() {
    let number_list = vec![34, 50, 68, 55, 100]; 

    let mut largest = &number_list[0]; 

    for number in &number_list { 
        if number > largest { 
            largest = number;
        }
    } 

    println!("The largest number is {}", largest);

    let number_list = vec![102, 34, 6000, 89, 54, 2, 43, 8]; 

    let mut largest = &number_list[0]; 

    for number in &number_list { 
        if number > largest { 
            largest = number;
        }
    }

    println!("The largest number is {}", largest);
 } 


 /*  
    Although this code works, duplicating code is tedious and error prone
    We also have to remember to duplicate the code in multiple places when we want 
    to change it.  

    To eliminate this duplication, we'll create an abstraction by defining a function that operates 
    on any list of integers passed in a parameter. 
    This solution makes our code clearer and lets us express the concept of finding the largest 
    number in a list abstractly

 */