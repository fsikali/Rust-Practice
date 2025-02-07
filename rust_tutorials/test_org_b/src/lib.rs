/*
--- Integration Tests
- In Rust, integration tests are entirely external to your library
- They use your library in the same way any other code would, which means they can only call functions
  that are part of your library's public API.
- Their purposse is to test whether many parts of your library work together correctly
- Units of code that work correctly on their own could have problems when integrated, so test coverage
  of the integrated code is important as well.
- To create integration tests, you first need a tests directory

--- The tests Directory
- We create a tests directory at the top level of our project directory, next to src.
- Cargo knows to look for integration test files in this directory.
- We can then make as many test files as we want, and Cargo will compile each of the files as an
  individual crate.
- Let's create an integration test.
- With the code still in the src/lib.rs file, make a tests directory, and create a new file named
  tests/integration_test.rs.
- 
 */ 

 pub fn add_two(a: i32) -> i32 {
    internal_adder(a, 2)
}

fn internal_adder(a: i32, b: i32) -> i32 {
    a + b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn internal() {
        assert_eq!(4, internal_adder(2, 2));
    }
}

/*
- The three sections of output include the unit tests, the integration test, and the doc tests.
- Note that if any test in a section fails, the following sections will not be run.
- For example, if a unit test fails, there won't be any output for integration and doc tests 
  beacuse those tests will only be run if all unit tests are passing.
- The first section for the unit tests is the same as we've been seeing: one line for each unit
  test (one names internal that we added) and then a summary line for the unit tests.
- The integration tests section starts with the line Running tests/integration_test.rs.
- Next there is a line for each test function in that integration test and summary line for the 
  results of the integration test just before the Doc-tests adder section starts.
- Each integration test file has its own section, so if we add more files in the tests directory,
  there will be more integration test sections.
- We can still run a particular test function by specifying the test function's name as an argument to
  cargo test.
- To run all the tests in a particular integration test file, use the --test argument of cargo test
  followed by the name of the file:
- This command runs only the tests in the tests/integration_test.rs file. 

--- Submodules in Integration Tests
- As you add more integration tests, you might want to make more files in the tests
  directory to help organize them; for example, you can group the test functions by the
  functionality they're testing.
- As mentioned earlier, each file in the tests directory is compiled as its own separate
  crate, which is useful for creating separate scopes to more closely imitate the way end
  users will be using your crate.
- However, this means files in the tests directory don't share the same behavior as files
  in src do.
- The different behavior of tests directory files is most noticeable when you have a set
  of helper functions to use in multiple integration test files and you try to follow the steps
  in the "Separating Modules into Different Files" to extraxt them into a common module.
- For example, if we create tests/common.rs and place a function named setup in it, we can
  add some code to setup that we want to call from multiple test functions in multiple test
  files: 
- When we run the tests again, we'll see a new section in the test output for the common.rs file,
  even though this file doesn't contain any test functions nor did we call the setup function
  from anywhere:
- Having common appear in the test results with running 0 tests displayed for it is not what we
  wanted.
- We just wanted to share some code with the other integration test files.
- To avoid having common appear in the test output, instead of creating tests/common.rs,
  we'll create tests/common/mod.rs.
- This is the older naming convection that Rust also understands that we mentioned in the
  "Alternative File Paths" section.
- Naming the file this way tells Rust not to treat the common module as an integration test file.
- When we move the setup function code into tests/common/mod.rs and delete the tests/common.rs file,
  the section in the test output will no longer appear.
- Files in subdirectories of the tests directory don't get compiled as separate crates or have
  sections in the test output.
- After we've created tests/common/mod.rs, we can use it from any of the integration test files as a
  module.
- Here's an example of calling the setup function from the it_adds_two test in tests/integration_test.rs
*/
