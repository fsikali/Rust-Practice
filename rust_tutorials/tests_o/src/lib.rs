/* 
   - To see what happens when a should_panic test with an expected message fails, let's again introduce
     a bug into our code by swapping the bodies of the if value < 1 and else if value > 100 blocks: 
*/

pub struct Guess {
    value: i32,
}

impl Guess {
    pub fn new(value: i32) -> Guess {
        if value < 1 {
            panic!(
                "Guess value must be less than or equal to 100, got {}.",
                value
            );
        } else if value > 100 {
            panic!(
                "Guess value must be greater than or equal to 1, got {}.",
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
    Explanation:
    - The failure message indicates that this test did indeed panic as we expected, but the panic message did
      not include the expected string 'Guess value must be less than or equal to 100'.
    - The panic message that we did get in this case was Guess value must be greater than or equal to 1,
      got 200.
    - Now we can start figuring out where our bug is!
*/