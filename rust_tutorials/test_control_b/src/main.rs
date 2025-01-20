/*
- A silly function that prints the value of its parameter and returns 10, as well as a test that
  passes and a test that fails.
- Tests for a function that calls println! 
 */

fn prints_and_returns_10(a: i32) -> i32 { 
    println!("I got the value {}", a);
    10
}

#[cfg(test)]
mod tests { 
    use super::*;

#[test]
fn this_test_will_pass() { 
    let value = prints_and_returns_10(4);
    assert_eq!(10, value);
}

#[test]
fn this_test_will_fail() {
    let value = prints_and_returns_10(8);
    assert_eq!(5, value);
  } 
}  


