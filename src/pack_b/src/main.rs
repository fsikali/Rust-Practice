// Packages and Crates

// Crate - Is the smallest amout of code that the Rust compiler considers at a time
// Crates can contain modules, and the modules may be defined in other files that get
// compiled with the crate 

// A crate can come in one of two forms: 
// 1.Binary crates - Are programs you can compile to an executable that you can run, such as 
// a command-line program or a server, each must have a function called main that defines
// waht happens when the executable runs. 

// Library crates - They don't have a main function , and they don't compile to an executable,
// instead they define functionality intended to be shared with multiple projects. 

// Example - The rand crate provides functionality that generates random numbers 
// N/B - Most of the time when Rustaceans say "crate", they mean library crate, and they use "crate"
// interchangeably with the general programming concept of a "library" 

// The crate root is a source file that the Rust compiler starts from and makes up the root 
// module of your crate 

// A package - Is a bundle of one or more crates that provided a set of functionality 
// A package contains a Cargo.toml file that describes how to build those crates 

// Cargo - Is actually a package that contains the binary crate for the command-line tool
// you've been using to build your code. 

// The Cargo package also contains a library crate that the binary crate depends on. 

// Other projects can depend on the Cargo library crate to use the same logic the Cargo 
// command-line tool uses

// A package - can contain as many binary crates as you like, but at most only one library crate 
// A package must contain at least one crate, whether that's library or binary crate


