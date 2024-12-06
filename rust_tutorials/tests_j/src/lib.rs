/*
   - Let's intoduce a bug into this code by changing greeting to exclude name to see what the default
     test failure looks like:
*/

pub fn greeting(_name: &str) -> String { // check the why an underscore is needed before the parameter
    String::from("Hello")
} 

#[cfg(test)]
mod tests { 
    use super::*;

    #[test]
    fn greeting_contains_name() {
        let result = greeting("Carol");
        assert!(result.contains("Carol"));
    }
} 

/* 
   - This result just indicates that the assertion failed and which line the assertion is on.
   - A more useful failure message would print the value from the greeting function
*/