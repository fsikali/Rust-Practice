/*
   Example:
   - Struct name ImportantExcerpt that holds a string slice
   - A struct that holds a reference, requiring a lifetime annotation 

   - Explanation in lib.rs
   
*/ 

struct ImportantExcerpt<'a> {
    part: &'a str,
}

fn main() {
    let novel = String::from("Call me Ishmael. Some years ago...");
    let first_sentence = novel.split('.').next().expect("Could not find a '.'");
    let i = ImportantExcerpt {
        part: first_sentence,
    }
}