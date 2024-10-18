// Example: In contrast, if we make an enum public all of its variants are then public 
// We only need the pub before the enum keyword 

mod back_of_house { 
    pub enum Appetizer { 
        Soup, 
        Salad,
    }
}
 
pub fn eat_at_restaurant() { 
    let order1 = back_of_house::Appetizer::Soup; 
    let order2 = back_of_house::Appetizer::Salad;
 } 

 // Because we made the Appetizer enum public, we can use the Soup and Salad variants
 // in eat_at_restaurant 

 // Enums aren't very useful unless their variants are public; it would be annoying to have 
 // to annonate all enum variants with pub in every case, so default for enum variants is to 
 // public.

 // Structs are often useful without their fields being public, so struct fields follow 
 // the general rule of everything being private by default unless annotated with pub 

 // There's one more situation involving pub that we haven't covered, and that is our last
 // module system feature: the use keyword. We'll cover use by itself first, and then we'll 
 // show how to combine pub and use.