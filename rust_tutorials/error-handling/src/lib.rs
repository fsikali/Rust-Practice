/*
--- Recoverable Errors with Result 
- The T and E are generic type parameters
- T represents the type of the value that will be returned in a success case within the 
  Ok variant
- E represents the type of the error that will be returned in a failure case within the
  Err variant
*/

#![allow(unused)] 

enum Result<T, E> { 
    Ok(T), 
    Err(E),
}
