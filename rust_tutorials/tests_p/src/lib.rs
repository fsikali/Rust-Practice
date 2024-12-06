/* 
    Using Result<T, E> in Tests
    - Our tests so far all panic when they fail
    - We can alsoo write tests that use Result<T, E>!
    - Here'e the test rewritten to use Result<T, E> and return an Err instead of panicking
*/

#[cfg(test)]
mod tests { 
    #[test]
    fn it_works() -> Result<(), String> { 
        if 2 + 2 == 4 { 
            Ok(())
        } else { 
            Err(String::from("two plus two does not equal four"));
        }
    }
} 

/* 
  - The it_works function now has the Result<(), String> type.
  - In the body of the function, rather than calling the assert_eq! macro, Ok(()) when the test
    passes and an Err with a String inside when the test fails.
  - Writing tests so they return a Result<T, E> enables you to use the question mark operator in the body
    of tests, which can be a convenient way to write tests that should fail if any operation within them 
    returns an Err variant.
  - You can't use the #[should_panic] annotation on tests that use Result<T, E>.
  - To assert that an operation returns an Err variant, don't use the question mark operator
    on the Result<T, E> value.
  - Instead, use assert!(value.is_err())
  - 
*/
