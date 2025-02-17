/*
- Similary, the expect method lets us also choose the panic! error message.
- Using expect instead of unwrap and providing good error messages can convey your intent
  and make tracking down the source of a panic easier
- The syntax of expect looks like this:
- Example in main.rs

- We use expect in the same way as unwrap:to return the file handle or call the panic! macro.
- The error message used by expect in its call panic! will be the parameter that we pass to 
  expect, rather than the default panic! message that unwrap uses.
- In production-quality code, most Rustaceans choose expect rather than unwrap and give more
  context about why the operation is expected to always succeed.
- That way, if your assumptions are ever proven wrong, you have more information to use in 
  debugging:
*/
