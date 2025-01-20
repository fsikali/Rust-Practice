#[test]
fn it_works() { 
    assert_eq!(2 + 2, 4);
}

#[test]
#[ignore]
fn expensive_test() {
    // code that takes an hour to run
}

/*
- After #[test] we add the #[ignore] line to the test we want exclude
- Now when we run our tests, it-works runs, but expensive_test doesn't:
- The expensive_test function is listed as ignored.
- If we want to run only the ignored tests, we can use cargo test -- --ignored:
- By controlling which tests run, you can make sure your cargo test results will be fast.
- When you're at a  point where it makes sense to check the results of the ignored tests and 
  you have time to wait for the results, you can run cargo test -- --ignored instead.
- If you want to run all tests whether they're ignored or not, you can run cargo test -- --include-ignored.
*/
