/* - Attempting to use indexing syntax with String
   - This code will result into an error
   - This error and the note tell the story: Rust strings don't support indexing.
*/

fn main() { 
    let a = String::from("hello"); 
    let b = a[0];
}
