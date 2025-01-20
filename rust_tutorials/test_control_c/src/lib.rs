/*
--- Running a Subset of Tests by Name

- Sometimes, running a full test site can take a long time.
- If you're working on code in a particular area, you migbt want to run only the tests pertaining
  to that code.
- You can choose which tests to run by passing cargo test the name or names of the test(s) you want
  to run as an argument.
- To demonstrate how to run a subset of tests, we'll first create three tests for our add_two function,
  and choose which ones to run.
- If we run the tests without passing any arguments, as we saw earlier, all the tests will run in parallel:

--- Running Single Tests
- We can pass the name of any test function to cargo test to run only that test:
- Only the test with the name one_hundred ran; the other two tests didn't match that name.
- The test output let us know we had more tests that didn't run by displaying 2 filterd out at the end.
- We can't specify the names of multiple tests in this way; only the first value given to cargo test
  will be used. But there is a way to run multiple tests.
 */

 