// Example: Calling std::mem::drop to explicitly drop a value before it goes out of scope

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
        data: String::from("Some data"),
    };
    println!("CustomSmartPointer created.");
    drop(c); 
    println!("CustomSmartPointer dropped before the end of main.");
}
