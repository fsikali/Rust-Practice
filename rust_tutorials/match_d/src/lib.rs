/* 
--- Matching with Option<T>
- A function that uses a match expression on an Option<i32> 
-
*/

fn plus_one(x: Option<i32>) -> Option<i32> { 
    match x { 
        None => None, 
        Some(i) => Some(i + 1),
    }
} 

