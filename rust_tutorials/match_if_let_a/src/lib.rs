/*
--- Concise Control Flow with if let
- The if let syntax lets you combine if and let into a less verbose way to handle values
  that match one pattern while ignoring the rest. 

- Example:  A match that only cares about executing code when the value is Some 

- If the value is Sme, we print out the value in the Some variant by binding the value to the
  variable max in the pattern.
- We don't want to do anything with the None value.
- To satisfy the match expression, we had to add _=> () after processing just one variant, which
  is annoying boilerplate code to add.
 */ 
