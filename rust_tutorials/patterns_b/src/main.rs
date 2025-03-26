// Using a while let loop to print values for as long as stock.pop() returns Some 

fn main() { 
    let mut stack = Vec::new(); 

    stack.push(1); 
    stack.push(2); 
    stack.push(3); 

    while let Some(top) = stack.pop() { 
        println!("{}", top);
    }
}
