// Annotations of the lifetimes of r and x, named 'a and 'b, respectively

fn main() { 
    let r;                    //-------------+-- 'a
                              //             |
    {                         //             |
        let x = 5;            //-+-- 'b      |
        r = &x;               // |           |
    }                         //-+           |
                              //             |
    println!("r: {}", r);     //             |
}                             //-------------+