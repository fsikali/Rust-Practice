/*
--- Matching on Different Errors
- In the code error-handling_a will panic! no matter why File::open failed
- However,we want to take different actions for different failure reasons
- If File::open failed because the file doesn't exist, we want to create the file and return 
  the handle to the new file.
- If File::open failed for any reason, we still want the code to panic! in the same way
- The type of the value that File::open returns inside the Err variant is io::Error, which is 
  a struct provided by the standard library
- This struct has a mthod kind that we can call to get an io::ErrorKind value
- The enum io::ErrorKind is provided by the standard library and has variants representing the
  different kinds of errors that might result from an io operation.
- The variant we want to use is ErrorKind::NotFound, which indicates the file we're tyring to 
  open doesn't exist yet
- So we match on greeting_file_result, but we also have an inner match on error.kind().
- The condition we want to check in the inner match is whether the value returned by error.kind()
  is the NotFound variant of the ErrorKind enum.
- If it is, we try to create the file with File::create
- However, because File::create, could also fail, we need a second arm in the inner match expression.
- When the file can't be created, a different error message is printed
- The second arm of the outer match stays the same, so the program panics on any error besides the missing
  file errror
*/
