// Managing growing projects with packages, crates and modules
// As you write large programs, organizing your code will become increasingly important
// By - 
  // 1.) By grouping related functionality  
  // 2.) Separating code with distinct features 

// You'll clarify where to find code that implements a particular feature and where
// to go to change how a feature works

// As a project grows, you should organize code by splitting it into multiple modules
// and then multiple files 
// A package - can contain multiple binary crates and optionally one library crate 
// As a package grows you can extract parts into separate crates that become external dependecies   

// N/B - Encapsulation of implementaion details helps to use code at a higher level

// Once you've implemented an operation, other code can call your code via its public interface without having
// to know how the implementation works 
// The way you write your code defines which parts are public for other code to use and which parts are 
// private implementatiion details that you reserve the right to change 

// N/B - This is another way to limit the amount of detail you have to keep in your head 

// Scope - The nested context in which code is written has a set of names that are defined as "in scope"
// You can create scopes and change which names are in or out of scope 
// N/B - You can't have two items with the same name in the same scope; tools are available to
// resolve name conflicts

// Example of module system: 
// - Packages : A cargo feature that lets you build, test and share crates
// - Crates : A tree of modules that produes a library  or executab;e 
// - Modules and use : Let you control the organization, scope, and privacy of paths
// - Path : A way of naming an item, such as struct, function, or module












