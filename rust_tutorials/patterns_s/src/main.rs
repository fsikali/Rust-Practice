/*
--- An unused variable starting with an underscore still binds the value, 
    which might take ownership of the value
*/ 

fn main() {
    let s = Some(String::from("Hello!"));

    if let Some(_s) = s {
        println!("found a string");
    }

    println!("{:?}", s);
}
