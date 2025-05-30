/*  
--- Controlling How Tests Are Run ---

- Just as cargo run compiles your code and then runs the resulting binary, cargo test
  compiles your code in test mode and runs the resulting test binary.
- The default behavior of the binary produced by cargo test is to run all the tests in 
  parallel and capture output generated during test runs, preventing the output from being
  and making it easier to read the output related to the test results.
- You can however, specify command line options to change this default behavior
- Some command line options go to cargo test, and some go to the resulting test binary
- To separate these two types of arguments, you list the arguments that go to cargo test followed
  by separator -- and then the ones that go to the test binary.
- Running cargo test -- --help displays the options you can use after the separator
 */ 
