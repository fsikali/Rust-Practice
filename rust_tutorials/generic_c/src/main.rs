/*  
   Removing Duplication by Extracting a Function 
   Example - Two functions that differ only in their names and the types in their signature

 */  

fn largest_i32(list:&[i32]) -> &i32 { 
    let mut largest = &list[0]; 

    for item in list { 
        if item > largest { 
            largest = item;
        }
    } 

    largest 
} 

fn largest_char(list: &[char]) -> &char { 
    let mut largest = &list[0]; 

    for item in list { 
        if item > largest { 
            largest = item;
        }
    } 

    largest
}

fn main() { 
    let number_list = vec![34, 50, 40, 25, 100];  
    
    let result = largest_i32(&number_list);
    println!("The largest number is {}", result); 
    assert_eq!(*result, 100); 

    let char_list = vec!['y', 'n', 'l', 'q']; 
    
    let result = largest_char(&char_list); 
    println!("The largest char is {}", result); 
    assert_eq!(*result, 'y'); 

}
 

