/* 
   - Let's add a custom failure message composed of a format string with a placeholder filled in 
     with the actual value we got from the greeting function.
*/

pub fn greeting(_name: &str) -> String { 
    String::from("Hello!")
} 

#[cfg(test)]
mod tests { 
    use super::*;

    #[test]
    fn greeting_contains_name() { 
        let result = greeting("Carol");
        assert!( 
            result.contains("Carol"),
            "Greeting did not contain name, value was `{}`",
            result
        );
    }
}

/* 
   We can see the value we actually got in the test output, which would help us debug what happened
   instead of what we were expecting to happen. 

   */
  