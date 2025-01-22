/*
--- Integration Tests for Binary Crates
- If our project is a binary crate that only contains a src/main.rs file and doesn't have a
  src/lib.rs file, we can't create integration tests in the tests directory and bring functions
  defined in the src/main.rs file into scope with a use statement.
- Only library crates expose functions that other crates can use; binary creates are meant to be run
  on their own.
- This is one of the reasons Rust projects that provide a binary have a straightfoward src/main.rs file
  that calls logic that lives in the src/lib.rs.
- Using that structure, integration tests can test the library crate with use to make the important 
  functionality available.
- If the important functionality works, the small amount of code in the src/main.rs file will work as well,
  and that small amount of code doesn't need to be tested. 

--- Summary ---
- Rust's testing features provide a way to specify how code should function to ensure it continues to work
  as you expect, even as you make changes.
- Unit tests exercise different parts of a library separately and can test private implementation details.
- Integration tests check that many parts of the library work together correctly, and they use the library's 
  public API to test the code in the same way external code will use it.
- Even though Rust's type system and ownership rules help prevent some kinds of bugs, tests are still important
  to reduce logic bugs having to do with how your code is expected to behave.
*/
