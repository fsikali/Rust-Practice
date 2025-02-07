/*
--- Test Organization ---
- As mentioned at the start of the chapter, testing is a complex discipline, and different people use different
  terminology and organization.
- The Rust community thinks about tests in terms of two main categories: unit tests and integration tests.
- Unit tests ara small and more focused, testing one module in isolation at a time, and can test private
  interfaces.
- Integration tests are entirely external to your library and use your code in the same way any other
  external code would, using only the public interface and potentially exercising multiple modules per test.
- Writing both kinds of tests is important to ensure that the pieces of your library are doing what you
  expect them to, separately and together.

--- Unit Tests --- 
- The purpose of unit tests is to test each unit of code in isolation from the rest of the code to quickly
  pinpoint where code is and isn't working as expected.
- You'll put unit tests in the src directory in each file with the code that they're testing.
- The convection is to create a module named tests in each file to contain the test functions and to 
  annotate the module with cfg(test).

--- The Tests Module and #[cfg(test)] ---
- The #[cfg(test)] annotation on the tests module tells Rust to compile and run the test code only when you
  run cargo test, not when you run cargo build.
- This saves compile time when you only want to build the library and saves space in the resulting compiled
  artifact because the tests are not included.
- You'll see that because integration tests go in a different directory, they don't need the #[cfg(test)]
  annotation.
- However, because unit tests go in the same files as the code, you'll use #[cfg(test)] to specify that they
  shouldn't be included in the compiled result.
- Recall that when we generated the new adder project in the first section of this chapter, Cargo generated
  this code for us:
- This code is the automatically generated test module.
- The attribute cfg stand for configuration and tells Rust that the following item should only be
  included given a certain configuration option.
- In this case, the configuration option is test, which is provided by Rust for compiling and running
  tests.
- By using the cfg attribute, Cargo compiles our test code only if we actively run tests with cargo test.
- This includes any helper functions that might be within this module, in addition to the functions annotated
  with #[test].
 */

 