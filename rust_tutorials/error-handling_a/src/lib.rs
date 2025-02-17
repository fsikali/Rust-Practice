/*
- Calling a function that returns a Result value because the function could fail
- Example in main.rs

- The return type of File::open is a Result<T, E>
- The generic parameter T has been filled in by the implementation of File::open with the type of the
  success value std::fs::File, which is a file handle
- The type of E used in the error value is std::io::Error
- This return type means the call to File::open might succeed and return a file handle that we can read
  from or write to.
- In a case where File::open succeeds, the value in the variable greeting_file_result will be an instance
  of Ok that contains a file handle.
- In the case where it fails, the value in greeting_file_result will be an instance of Err that contains
  more information about the kind of error that happened.

- N/B - Like the Option enum, the Result enum and its variants have been brought inot scope by the prelude,
        so we don't need to specify Result:: before the Ok and Err variants in the match arms.

- When the result is Ok, this code will return the inner file value out of the Ok varint, and we then assign
  that file handle value to the variable greeting_file.
- After the match, we can use the file handle for reading or writing
- The other arm of the match handles the case where we get an Err value from File::open.
- In this example, we've chosen call panic! macro.
*/