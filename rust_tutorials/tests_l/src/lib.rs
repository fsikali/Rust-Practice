/* 
   Checking for Panics with should_panic
   - In addition to checking return values, it's important to check that our code handles error conditions
     as we expect.
   - We can write a test that ensures that attempting to create a Guess instance with a value outside that
     range panics.

   Example: 
   - Testing that a condition will cause a panic!
*/ 

pub struct Guess { 
    value: i32,
} 

impl Guess { 
    pub fn new(value: i32) -> Guess { 
        if value < 1 || value > 100 { 
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
   - We place the #[should_panic] attribute after the #[test] attribute and before the test function
     it applies to.
*/
