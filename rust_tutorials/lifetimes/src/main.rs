// Example -  An attempt to use a referece whose value has gone out of scope results to a compile-time error

fn main() { 
    let r;

    { 
        let x = 5;
        r = &x;
    }
} 
