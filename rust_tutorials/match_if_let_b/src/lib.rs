/*
- The syntax if let takes a pattern and an expression separated by an equal sign
- It works the same way as a match, where the expression is given to the match and the pattern is its
  first arm.
- In this case, the pattern is Some(max), and the max binds to the value inside the Some.
- The code in the if let block isn't run if the value doesn't match the pattern
- Using if let means less typing, less indentation, and less boilerplate code.
- However, you lose the exhausitve checking that match enforces
 */
