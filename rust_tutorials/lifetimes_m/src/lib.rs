/*
   Lifetime Annotations in Method Definitions
   - When we implement methods on a struct with lifetimes, we use the same syntax as that of
     generic type parameters
   - Where we declare and use the lifetime parameters depends on whether they're related to the
     struct fields or the method parameters and return values
   - Lifetime names for struct fields always need to be declared after the impl keyword and then
     used after the struct's name, because those lifetimes are part of the struct's type.
   - In method signatures inside the impl block, references might be tied to the lifetime of references
     in the struct's field, or they might be independent.
   - In addition, the lifetime elision rules often make it so that lifetime annotations aren't necessary
     in method signatures.
   - 
*/