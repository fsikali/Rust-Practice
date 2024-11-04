/* 
     The Array 

     - Every element of an array must have the same type
     - Arrays in Rust have a fixed length 
     - Arrays are useful when you want your data allocated on the stack rather
       than the heap or when you want to ensure you always have a fixed number
       of elements.
     - 
 */

 // We write an array's type using square brackets with the type of each element,
 // a semicolon, and then the number of elements in the array
 let a: [i32; 5] = [1, 2, 3, 4, 5];
 
 // We can initialize an array to contain the same value for each element by specifying
 // the initial value, followed by a semicolon, and then the length of the array
 // in square brackets. 

 let a = [3; 5];  /* same as writing */ let a = [3, 3, 3, 3, 3, 3] 


 /*
     Accessing Array Elements
     The variable named first gets the value 1 because that is the value at index[0] in the array
     The variable named second will get the value 2 from index [1] in the array
 */ 

 let a: [i32; 4] = [1, 2, 3, 4, 5]; 
 
 let first = a[0];
 let second = a[1];


