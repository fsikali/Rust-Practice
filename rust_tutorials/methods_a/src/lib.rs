/*
- We can choose to give a method the same name as one of the struct's fields.
- For example, we can define a method on a Rectangle that is also named width:
*/

pub struct Rectangle { 
    pub width: u32,
    pub height: u32,
} 
  
impl Rectangle { 
    pub fn width(&self) -> bool { 
        self.width > 0
    }
}  


