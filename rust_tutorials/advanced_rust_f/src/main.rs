// Creating a slice from an arbitrary memory location 

fn main() { 
    use std::slice; 

    let address = 0x01234usize; 
    let r = address as *mut i32; 

    let values: &[i32] = unsafe { slice::from_raw_parts_mut(r, 1000) };
}
