/*
   Lifetime Annotation Syntax

   - Lifetime annotations don't change how long any of the references live.
   - Rather, they decribe the relationships of the lifetimes of multiple references
     to each other without affecting the lifetimes. 
   - Just as functions can accept any type when the signature specifies a generic type parameter,
     functions can accept references with any lifetime by specifying a generic lifetime parameter
 
   - Lifetime annotations have a slightly unusual syntax: the names of lifetime parameters must
     start with an apostrophe(') and are usually all lowercase and very short, like generics types.

   - Most people use the name 'a for the first lifetime annotation.
   - We place lifetime parameter annotations after the & of a reference, using a space to separate
     the annotation from the reference's type. 
   
   - One lifetime annotation by itself doesn't have much meaning, because the annotations are meant
     to tell Rust how generic lifetime parameters of multiple references relate to each other.
*/