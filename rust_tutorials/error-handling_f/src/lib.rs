/*
--- Propagating Errors
- When a function's implementation calls something that might fail, instead of handling
  the error within the function itself, you can return the error to the calling code so
  that it can decide what to do.
- This is known as propagating the error and gives more control to the calling code, where
  there might be more information or logic that dictates how the error should be handled
  than what you have available in the context of your code.
- Example in main.rs
*/
