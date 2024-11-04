/* 
    Invalid Array Element Access

    - When you attempt to access an element using indexing, Rust will check that 
      the index you've specified is less than the array length.
    - If the index is greater than or equal to the length, Rust will panic
    - This check has to happen at runtime, especially in this case, because the compiler
      can't possibly know what value a user will enter when they run the code later.
    - This is an example of rust's memory safely principles in action.
    - In many low-level languages, this kind of check is not done, and when you provide an
      incorrect index, invalid memory can be accessed. 
    - Rust protects you against this kind of error by immediately exiting instead of allowing
      the memory access and continuing
*/