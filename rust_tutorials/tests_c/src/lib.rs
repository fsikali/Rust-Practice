pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn exploration() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }

    #[test]
    fn another() {
        panic!("Make this test fail");
    }
} 

/*
    - Changed the name of the it_works function to exploration
    - Added another test, that fails 
    - Tests fails when something in the test function panics
    - Each test is run in a new thread, and when the main thread sees that a test thread has died, the
      test is marked as failed.
    - The simplest way to panic is to call the panic! macro

*/
