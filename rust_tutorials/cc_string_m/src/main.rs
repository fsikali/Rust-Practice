/*
- Rather then indexing using [] with a single number, you can use [] with a range to create a string
  slice containing particular bytes:
- Here, s will be a &str that contains the first 4 bytes of the string.
- Each of these characters has 2 bytes
- If we were to try to slice only part of a characte's bytes with something like &hello[0..1], Rust
  would panic at runtime in the same way as if an invalid index were accessed in a vector:
- N/B - Ranges should used with caution to create string slices, because doing so can crash your 
        program.
*/ 

#![allow(unused)] 
fn main() { 
    let hello = "Здравствуйте";

    let s = &hello[0..4];

    println!("{s}");
}
