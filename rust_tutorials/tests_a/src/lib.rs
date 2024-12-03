/*
    How to Write Tests 
    -- Tests are Rust functions that verify that the non-test code is functioning in the expected manner
    -- The bodies of test functions typically perform these three actions:
    
    1. Set up any needed data or state
    2. Run the code you want to test
    3. Assert the results are what you expect 

    The Anatomy of a Test Function
    -- At its simplest,a test in Rust is a function that's annotated with the test attribute
    -- Attributes are metadata about pieces of Rust code; one example is the derive attribute used
       with structs 
    -- To change a function into a test function, add #[test] on the line before fn.
    -- When you run your tests with the cargo test command, Rust builds a test runner binary that runs
       the annotated funtions and reports on whether each test function passes or fails.
    -- Whenever we make a new library project with Cargo, a test module with a test function in it is automatically
       generated for us.
    -- This module gives you a template for writing your tests so you don't have to look up the exact structure
       and syntax every time you start a new project.
    -- You can add as many additional test functions and as many test modules as you want! 
        
*/