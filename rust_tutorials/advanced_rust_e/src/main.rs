// An attempted implementation of split_at_mut using only safe Rust
// When we try to compile the code we'll get an error

fn split_at_mut(values: &mut [i32], mid: usize) -> (&mut [i32], &mut [i32]) { 
    let len = values.len(); 

    assert!(mid <= len); 

    (&mut values[..mid], &mut values[mid..])
}

fn main() { 
    let mut vector = vec![1, 2, 3, 4, 5, 6];
    let (left, right) = split_at_mut(&mut vector, 3);
}
