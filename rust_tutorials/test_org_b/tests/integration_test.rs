/* An integration test of a function in the test_org_b crate */

use test_org_b;  

mod common;

#[test]
fn it_adds_two() {
    common::setup();
    assert_eq!(4, test_org_b::add_two(2));
 }

/*
 -Each file in the tests directory is a separaate crate, so we need to bring
   our library into each test crate's scope.
- For that reason we add use test_org_b at the top of the code, which we didn't
  need in the unit tests.
- We don't need to annotate any code in tests/integration_test.rs with #[cfg(test)]
- Cargo treats the tests directory specially and compiles files in this directory only 
  when we run cargo test.
- Run cargo test now:
*/ 
