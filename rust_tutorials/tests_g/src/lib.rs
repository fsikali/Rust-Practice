/*
    Testing Equality with the assert_eq! and assert_ne! Macros
    - A common way to verify functionality is to test for equality between the result of the code under
      test and the value you expect the code to return.
    - This is done using assert! macro and passing it an expression using the == operator
    - This is such a common test that the standard library provides a pair of macros assert_eq! 
      and assert_ne! to perform this test more conveniently
    - These macros compare two arguments for equality or inequality, respectively
    - They'll also print the two values if the assertion fails, which makes it easier to see why the test 
      failed; conversely, the assert! macro only indicates that it got a false value for the == expression,
      without printing the values that led to the false value. 

*/ 

pub fn add_two(a: i32) -> i32 { 
    a + 2
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_adds_two() {
        assert_eq!(4, add_two(2));
    }
} 


/* 
   We pass 4 as the argument to assert_eq!, which is equal to the result of calling add_two(2).
   The line for this test is test tests::it_adds_two ... ok, and the ok text indicates that our test passed!
*/
