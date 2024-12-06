/* 
    - Code that contains a bug
*/ 

pub struct Guess {
    value: i32,
}

// --snip--
impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!("Guess value must be between 1 and 100, got {}.", value);
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic]
    fn greater_than_100() {
        Guess::new(200);
    }
}

/* 
  - We don't get a helpful message in this case, but when we look at the test function, we see that
    it's annotated with #[should_panic].
  - The failure we got means that the code in the test function did not cause a panic.
  - Tests that use should_panic can be imprecise
  - A should_panic test would pass even if the test panics for a different reason from the one we were
    expecting.
  - To make should_panic tests more precise, we can add an optional expected parameter to the should_panic
    attribute.
  - The test harness will make sure that the failure message contains the provided text
  
*/