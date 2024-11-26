/*
   Lifetime Annotations in Struct Definitions
   - The structs we've defined all hold owned types
   - We can define structs to hold references but in that case we would need to add
     a lifetime annotation on every reference in the struct's definition
   
   Explanation
   - This struct has the single field part that holds a string slice, which is a reference.
   - As with generic data types, we declare the name of the generic lifetime parameter inside angle
     brackets after the name of the struct so we can use the lifetime parameter in the body of the
     struct definition.
   - This annotation means an instance of ImportantExcerpt can't outlive the reference it holds in
     its part field.
   - The main function here creates an instance of the ImportantExcerpt struct holds a reference to the
     first sentence of the String owned by the variable novel.
   - The data in novel exists before the ImportantExcerpt instance is created
   - In addition, novel doesn't go out of scope until after the ImportantExcerpt goes out of scope,
     so the reference in the ImportantExcerpt instance is valid.
*/
