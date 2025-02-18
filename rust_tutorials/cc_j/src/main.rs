// Showing where the vector and its elements are dropped

fn main() { 
    
    { 

        let v = vec![1, 2, 3, 4];

        // do stuff with v
    } // <- v goes out of scope and is freed here

}

