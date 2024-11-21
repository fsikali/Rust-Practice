/*
   - In this example, string1 is valid until the end of the outer scope, string2 is valid until the end
     of the inner scope, and result references something that is valid until the end of the inner scope.
   - Borrow checker approves when this code is executed
   - It compiles and prints: The longest string is long string is long
   

*/