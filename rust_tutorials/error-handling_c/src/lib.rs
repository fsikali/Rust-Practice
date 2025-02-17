/*
--- Alternatives to Using match with Result<T, E>
- That's a lot of match! The match expression is very usefull but also very much primitive
- In Chapter 13, you'll learn about closures, which are used with many of the methods defined
  on Result<T, E>.
- These methods can be more concise than using match when handling Result<T, E> values in your
  code.
- For example, here's another way to write the same logic shown in error-handling_b this time
  using closures and the unwrap_or_else method:
- Although this code has the same behavior as error-handling_b, it doesn't contain any match 
  expressions and is cleaner to read.
- Come back to this example after you've read Chapter 14, and look up the unwrap_or_else method
  in the standard library documentation.
- Many more of these methods can clean up huge nested match expressions when you're dealing with
  errors.
*/
