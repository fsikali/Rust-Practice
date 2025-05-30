use std::fs::File;
use std::io::ErrorKind;

// Handling different kinds of erroes in different ways

fn main() { 
    let greeting_file_result = File::open("hello.txt");

    let greeting_file = match greeting_file_result { 
        Ok(file) => file,
        Err(error) => match error.kind() { 
            ErrorKind::NotFound => match File::create("hello.txt") { 
                Ok(fc) => fc,
                Err(e) => panic!("Problem creating the file: {:?}", e),
            }, 
            other_error => { 
                panic!("Problem opening the file: {:?}", other_error);
            }
        },
    };
}
