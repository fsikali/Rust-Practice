/*
   Adding Custom Failure Messages
   - You can also add a custom message to be printed with the failure message as optional arguments to 
     the assert!, assert_eq!, and assert_ne! macros. 
   - Any arguments specified after the required arguments are passed along to the format! macro 
   - So you can pass a format string that contains {} placeholders and values to go in those placeholders.
   - Custom messages are useful for documenting what an assertion means; when a test fails, you'll have a
     better idea of what the problem is with the code.
    
    Example: A function that greets people by name and we want to test that the name we pass into the
             function appears in the output
*/ 

pub fn greeting(name: &str) -> String { 
    format!("Hello {}!", name)
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
   - The requirements for this program haven't been agreed upon yet, and we're pretty sure the Hello text
     at the beginning of the greeting will change.
   - We decided we don't want to have to update the test when the requirements change, so instead of checking
     for exact equality to the value returned from greeting function, we'll just assert that the output contains
     the text of the input parameter.
   
*/