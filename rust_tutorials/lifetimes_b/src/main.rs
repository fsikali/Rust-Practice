// A valid reference because the data has a longer lifetime than the reference
// No dangling reference hence the code compiles without any errors

fn main() { 
    let x = 5;             //-----------+-- 'b
                           //           |
    let r = &x;            //--+-- 'a   |
                           //  |        |
    println!("r: {}", r);  //  |        |
                           //--+        |
}                          //-----------+