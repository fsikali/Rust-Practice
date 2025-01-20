/*
--- Showing Function Output
- By default, if a test passes, Rust's test library captures anything printed to a standard output.
- For example, if we call println! in a test and the test passes, we won't see the println! output in the
  terminal; we'll see only the line that indicates the test passed.
- If a test fails, we'll see whatever was printed to standard output with the rest of the failure message
- Example in main.rs:

- Note that nowhere in this output do we see I got the value 4, which is what is printed when the 
  test that passes runs.
- That output has been captured.
- The output from the test that failed, I got the value 8, appears in the section of the test summary
  output, which also shows the cause of the test failure.
- If we want to see printed values for passing tests as well, we can tell Rust to also show the output of
  successful tests with;
cargo test -- --show-output
- When we run the tests again with the --show-output flag, we see the following output:
*/


