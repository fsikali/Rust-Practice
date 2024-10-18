// Example - Moving the eat_at_restaurant function into a new child module named customer, 
// which is then a different scope than the use statement, so function body won't compile 

mod front_of_house { 
    pub mod hosting { 
        pub fn add_to_waitlist() {}
    }
} 

use crate::front_of_house::hosting; 

mod costomer { 
    pub fn eat_at_restaurant() { 
        hosting::add_to_waitlist();
    }
}

// N/B - A use statement only applies in the scope it's in 
// The compiler error shows that the shortcut no longer applies within the customer module: 

// N/B - There's also a warning that the use is no longer used in its scope! To fix this problem, 
// move the use within the customer module too, or reference the shortcut in the parent module 
// with super::hosting within the child customer module
 
 