pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

/*
   Explanation:
   - This is a test module and function generated automatically by cargo new
   - It is also a template test not an actual code 
   - Note the #[test] annotation: this attribute indicates this is a test function, so the test runner
     knows to treat this function as a test
   - We might also have non-test functions in the tests module to help set up common scenarios or
     perform common operations,so we always need to indicate which functions are tests.
   - The example function body uses the assert_eq! macro to assert that result, which contains the result
     of adding 2 and 2, equals 4. 
   - This assetion serves as an example of the format for a typical test. 
   - The cargo test command runs all test in our project 
   - It's possible to mark a test as ignored so it doesn't run in a particular instance
   - We can also pass an argument to  the cargo test command to run only tests whose name matches a string; this is 
     called filtering.
   - The 0 measured statistic is for benchmark tests that measure performance
   - Benchmark tests are, as of this writing, only available in nightly Rust
   - Doc-tests is for the results of any documentation tests
   - We don't have any documentation tests yet, but Rust can compile any code examples that appear
     in our API documentation
   - This feature helps keep your docs and your code in sync!
   - 


*/