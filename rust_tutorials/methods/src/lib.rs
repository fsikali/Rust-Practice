/*
--- Methods ---
- Methods are similar to functions
- We declare them with the fn keyword and a name
- They contain some code that run's when the method is called from somewhere else
- Unlike functions, methods are defined within the context of a struct, enum or a trait
- Their first parameter is always self, which represents the instance of the struct the method
  is being called on.

--- Defining Methods
- Let's change the area function that has a Rectangle instance as a parameter and instead make
  an area method defined on the Rectangle struct:
*/ 

#[derive(Debug)]
pub struct Rectangle { 
    pub width: u32, 
    pub height: u32,
} 

impl Rectangle {
    pub fn area(&self) -> u32 { 
        self.width * self.height
    }
}


