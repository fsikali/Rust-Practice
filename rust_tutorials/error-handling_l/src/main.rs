use std::error::Error;
use std::fs::File; 

// Changing amin to return Result<(), E> allows the use of the ? operator on Result values

fn main() -> Result<(), Box<dyn Errr>> { 
    let greeting_file = File::open("hello.txt")?;

    Ok(())
}
