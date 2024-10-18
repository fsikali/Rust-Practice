// ---- Defining Modules to Control Scope and Privacy ----- 

// use - Is a keyword that brings a path into scope
// pub - Is a keyword to make items public  

// ---- Modules Cheat Sheet ---- 
// Start from the crate root - When compiling a crate, the compiler first looks in the crate root 
// file (usually src/lib.rs for a library crate or src/main.rs for a binary crate) for code to compile 

// Declaring modules - In the crate root file, you can declare new modules; day, you declare a "garden" 
// module with mode garden; The compiler will look for the module's code in these places: 

//  1.) Inline, within curly brackets that replace the semicolon following mod garden 
//  2.) In the file src/garden.rs 
//  3.) In the file src/garden/mod.rs 

// Declaring submodules - In any file other than the crate, you can declare submodules. 
// Example - You might declare mod vegetables; in src/garden.rs. The compiler will lokk for
// the submodule's code within the directory named for the parent module in these places 
// 1.) Inline, directory following mod vegetables, within curly brackets instead of the semicolon 
// 2.) In the file src/garden/vegetables.rs 
// 3.) In the files src/garden/vegetables/mod.rs 

// Paths to code in modules - Once a module is a part of your crate, you can refer to code in that
// module from anywhere else in that same crate, as long as the privacy rules alllow, using the path
// to the code. For e.g, An Asparagus type in the garden vegetables module would be found at 
// crate::garden::vegetables::Asparagus 

// Private vs public: Code within a module is private from its parent modules by default. To make a module 
// public, declare it with pub mod instead of mod. To make items within a public module public as well, use
// pub before their declarations 

// The use keyword - Within a scope, the use keyword creates shortcuts to items to reduce repetition of long
// paths. In any scope that can refer to crate::garden::vegetables::Asparagus, you can create a shortcut with use crate::garden::vagetables::Asparagus; 
// and from then on you only need to write Asparagus to make use of that type in the scope. 

