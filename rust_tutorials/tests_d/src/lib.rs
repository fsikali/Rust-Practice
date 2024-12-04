/*
   Checking Results with the assert! Macro
   - The assert! macro, provided by the standard library, is useful when you want to ensure that some
     condition in a test evaluates to true.
   - We give the assert! macro an argument that evaluates to a Boolean
   - If the value is true, nothing happens and the test passes 
   - If the value is false, the assert! macro calls panic! to cause the test to fail
   - Using the assert! macro helps us check that our code is functioning the way we intend  
   
   Example: A test for can_hold that checks whether a larger rectangle can indeed hold a smaller rectangle
   - The can_hold method returns a Boolean, which means it's a perfect use case for the assert! macro.
   - 
*/

#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Rectangle) -> bool {
        self.width > other.width && self.height > other.height
    }
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn larger_can_hold_smaller() {
        let larger = Rectangle { 
            width: 8, 
            height: 7,
        }; 
        let smaller = Rectangle {
            width: 5,
            height: 1,
        }; 

        assert!(larger.can_hold(&smaller));
    }
}

/*
   - Because the tests module is an inner module, we need to bring the code under test in the outer
     module into the scope of the inner module. 
   - We use glob here so anything we define in the outer module is available to thhis tests module
   - We've named our test larger_can_hold_smaller, and we've created the two Rectangle instances that we need.
   - Then we called the assert! macro and passed it the result of calling larger.can_hold(&smaller)
   - This expression is supposed to return true, so our test should pass

*/