// Creating a raw pointer to an arbitrary memory address

fn main() { 
    let address = 0x012345usize;
    let r = address as *const i32;
}
