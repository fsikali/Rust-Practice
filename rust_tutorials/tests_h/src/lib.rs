/* 
   Lets's introduce a bug into our code to see what assert_eq! looks like when it fails
   Change the implementatiion of the add_two function to instead add 3:
*/ 

pub fn add_two(a: i32) -> i32 { 
    a + 3
} 

#[cfg(test)] 
mod tests { 
    use super::*;
    
    #[test]
    fn it_adds_two() { 
        assert_eq!(4, add_two(2));
    }
} 

/* 
   - Our test caught the bug! 
   - The it_adds_two test failed, and the message tells us that the assertion that fails was assertion
     failed: `(left == right)` and what the left and right values are. 
   - Note that in some languages and test frameworks, the parameters to equality assertion functions are called
     expected and actaul, and the order in which we specify the arguments matters.
   - However, in Rust, they're called left and right, and the order in which we specify the value we expect and
     the value the code produces doesn't matter.
   - The assert_ne! macro will pass if the two values we give it are not equal and fail if they're equal
   - This macro is most useful for cases when we're not sure what a value will be, but we know what the value
     definitely shouldn't be. 
   - Under the surface, the assert_eq! and assert_ne! macros use the operators == and !=, respectively.
   - When the assertions fail, these macros print their arguments using debug formatting, which means the
     values being compared must implement the PartialEq and Debug traits.
   - All primitive types and most of the standard library types implement these traits
   - For structs and enums that you define yourself, you'll need to implement PartialEq to assert equality of
     those types.
   - You'll also need to implement Debug to print the values when the assertion fails
   - Because both traits are derivable traits this is usually as straightfoward as adding the 
     #[derive(PartialEq, Debug)] annotation to your struct or enum definition.
*/
