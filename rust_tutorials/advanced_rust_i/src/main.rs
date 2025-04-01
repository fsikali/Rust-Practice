// Reading from or writing to a mutable static variable is unsafe 

static mut COUNTER: u32 = 0; 

fn add_to_count(inc: u32) { 
    unsafe { 
        COUNTER += inc;
    }
} 

fn main() { 
    add_to_count(3); 

    unsafe { 
        println!("COUNTER: {}", COUNTER);
    }
}
