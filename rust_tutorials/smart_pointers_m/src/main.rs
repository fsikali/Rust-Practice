/*
- If we try to call the Drop trait's drop method manually by modifying the main function,
  we'll get a compile error.
*/

// Example: Attempting to call the drop method from the Drop trait manually to clean up early

struct CustomSmartPointer { 
    data: String,
}

impl Drop for CustomSmartPointer { 
    fn drop(&mut self) { 
        println!("Dropping CustomSmartPointer with data `{}`!", self.data);
    }
}

fn main() { 
    let c = CustomSmartPointer { 
        data: String::from("some data"),
    }; 
    println!("CustomSmartPointer created.");
    c.drop(); 
    println!("CustomSmartPointer dropped before the end of main.");
}
