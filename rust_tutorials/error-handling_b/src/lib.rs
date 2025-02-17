/*
--- Matching on Different Errors
- In the code error-handling_a will panic! no matter why File::open failed
- However,we want to take different actions for different failure reasons
- If File::open failed because the file doesn't exist, we want to create the file and return 
  the handle to the new file.
- If File::open failed for any reason, we still want the code to panic! in the same way
*/
