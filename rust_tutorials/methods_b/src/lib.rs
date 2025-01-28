/*
--- Finding the perimeter of a rectangle using struct, impl and methods
 */  

 pub struct Rectangle {
    pub width: u32,
    pub height: u32,
 } 

 impl Rectangle {
    pub fn perimeter(&self) -> u32 { 
        (&self.width + &self.height) * 2
    }
 }

