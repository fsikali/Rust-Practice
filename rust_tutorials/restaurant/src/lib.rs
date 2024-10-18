// GROUPING RELATED CODE IN MODULES 
// Modules -They let us orgainze code within a crate readability and asy reuse 
// Modules - Allow us to control the privacy of items, because code within a module is private by default 
// Private items are internal implementation details not available for outside use 
// We can choose to make modules and the items within them public, which exposes them to allow external 
// code to use and depend on them 

// Example : Write a library crate that provides the functionality of a restaurant. We'll define the signatures 
// of functions but leave their bodies empty to concentrate on the organization of the code, rather than the 
// implementation of a restaurant 

// In the restaurant industry, some parts of a restaurant are referred to as front of houe and others as back 
// of house. Front of house is where customers are; this encompasses where the hosts seat customers, servers take orders
// and payment, and bartenders make drinks. Back of house is where the chefs and cooks work in the kitchen, dishwashers
// clean up, and managers do administrative work 

// To structure our crate in this way, we can organize its function into nested modules. Create a new library named 
// restaurant by running cargo new restaurant --lib; then enter the code to define some modules and function 
// signatures.  

// Example: A front_of_house module containing other modules that then contain functions  
// Example: Calling the add_to_waitlist function using absolute and relative paths

mod front_of_house { 
   pub mod hosting { 
       pub fn add_to_waitlist() {} 

       // fn seat_at_table() {}
    }  


}   


pub fn eat_at_restaurant() { 
    // Absolute path
    crate::front_of_house::hosting::add_to_waitlist(); 

    // Relative path 
    front_of_house::hosting::add_to_waitlist();
}




// We define a module with the mod keyword followed by the name of the module (in this case, front_of_house). 
// The body of the module then goes inside curly brackets 
// Inside modules, we can place other modules, as in this case with the modules hosting and serving
// Modules can also hold definitions for other items, such as structs, enums, constants, traits 

// By using modules, we can grow related definitions together and name why they're related 
// Programmers using this code can navigate the code based on the groups rather than having to 
// read through all the definitions, making it easier to find the definitions relevant to them
// Programmers adding new functionality to this code would know where to place the code to keep 
// the program organized 

// Earlier, we mentioned that src/main.rs and src/lib.rs are called crate roots
// The reason for their name is that the contents of either of these two files form a  
// module named crate at the root of the crate's module structure, known as the module tree 

// PATHS FOR REFERRING TO AN ITEM IN THE MODULE TREE 

// To show Rust where to find an item in a module tree, we use a path in the same way we use
// a path when navigating a filesystem. To call a function, we need to know its path. 
// A path can take two forms: 
// --- An absolute path is the full path starting from a crate root; for code from an external 
// crate, the absolute path begins with crate name, and for code from the current crate, it starts
// with the literal crate 
// --- A relative path starts from the current module and uses self, super, or an identifier in the
// current module 

// Both absolute and relative paths are followed by one or more idetifiers separated by double colons 
// (::). 

// 