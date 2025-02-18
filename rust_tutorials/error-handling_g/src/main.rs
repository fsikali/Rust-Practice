use std::fs::File;
use std::io::{self, Read};

// A function that returns errors to the calling code using the ? operator 

#![allow(unused)] 
fn main() { 

    fn read_username_from_file() -> Result<String, io::Error> { 
        let mut username_file = File::open("hello.txt")?; 
        let must username = String::new(); 
        username_file.read_to_string(&muust username)?;
        Ok(username)
    }
}
