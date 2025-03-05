/*
- A customSmartPointer struct whose only custom functionality is that it will print Dropping CustomSmartPointer! 
  when the instance goes out of scope, to show when Rust runs the drop function.
*/

// Example: A CustomSmartPointer struct that implements the Drop trait where we would put our cleanup code

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
        data: String::from("my stuff"),
    }; 
    let d = CustomSmartPointer { 
        data: String::from("other stuff"),
    }; 
    println!("CustomSmartPointers created.");
}
