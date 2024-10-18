// Bringing Paths into Scope with the use Keyword

// Example - We bring the crate::front_of_house::hosting module into the scope 
// of the eat_at_restaurant function so we only have to specify hosting::add_to_waitlist 
// to call the add_to_waitlist function in eat_at_restaurant   

// Example: Bringing a module into scope with use 

mod front_of_house { 
    pub mod hosting { 
        pub fn add_to_waitlist() {}
    }
}  

use crate::front_of_house::hosting; 

pub fn eat_at_restaurant() { 
    hosting::add_to_waitlist();
}

// Adding use and path in a scope is similar to creating a symbolic link in the filesystem 
// By adding use crate::front_of_house::hosting in the crate root, hosting is now a valid name in 
// that scope, just as though the hosting module had been defined in the crate root 

// Path brought into scope with use also check privacy, like any other paths 

// N/B - use only creates that shortcut for the particular scope in which the use occurs 
