// Defining and calling a closurre that captures a mutable reference

fn main() { 
    let mut list = vec![1, 2, 3];
    println!("Before defining closure: {:?}", list); 

    let mut borrows_mutably = || list.push(7); 

    borrows_mutably();
    println!("After calling closure: {:?}", list);
}

