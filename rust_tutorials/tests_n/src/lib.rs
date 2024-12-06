// --snip-- 
/* 
   Example: 
   Testing for a panic! with a panic message containing a speficied substring
*/

pub struct Guess { 
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        }

        Guess { value }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    #[should_panic(expected = "less than or equal to 100")]
    fn greater_than_100() {
        Guess::new(200);
    }
} 

/* 
   - This test will pass because the value we put in the should_panic attribute's expected parameter is 
     a substring of the message that the Guess::new function panics with.
   - We could have specified the entire panic message that we expect, which in this case would be Guess value
     must be less than or equsl to 100, got 200.
   - What you choose to specify depends on how musch of the panic message is unique or dynamic and how precise
     you want your test to be.
   - In this case, a substring of the panic message is enough to ensure that the code in the test function executes
     the else if value > 100 case.
     
*/